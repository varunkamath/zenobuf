//! Pub/Sub benchmarks for Zenobuf
//!
//! Run with: `cargo bench --package zenobuf-core`

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use zenobuf_core::{Message, Node, QosProfile};

/// Simple test message for benchmarking
#[derive(Clone, Default, PartialEq)]
struct BenchMessage {
    /// Payload data
    data: Vec<u8>,
}

impl prost::Message for BenchMessage {
    fn encode_raw(&self, buf: &mut impl prost::bytes::BufMut)
    where
        Self: Sized,
    {
        if !self.data.is_empty() {
            prost::encoding::bytes::encode(1, &self.data, buf);
        }
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: prost::encoding::WireType,
        buf: &mut impl prost::bytes::Buf,
        ctx: prost::encoding::DecodeContext,
    ) -> std::result::Result<(), prost::DecodeError> {
        if tag == 1 {
            prost::encoding::bytes::merge(wire_type, &mut self.data, buf, ctx)
        } else {
            prost::encoding::skip_field(wire_type, tag, buf, ctx)
        }
    }

    fn encoded_len(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            prost::encoding::bytes::encoded_len(1, &self.data)
        }
    }

    fn clear(&mut self) {
        self.data.clear();
    }
}

impl Message for BenchMessage {
    fn type_name() -> &'static str {
        "BenchMessage"
    }
}

impl BenchMessage {
    fn with_size(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
        }
    }
}

/// Benchmark message serialization/deserialization
fn bench_message_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_encoding");

    for size in [64, 256, 1024, 4096, 16384].iter() {
        let msg = BenchMessage::with_size(*size);
        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(BenchmarkId::new("encode", size), size, |b, _| {
            b.iter(|| {
                let mut buf = Vec::with_capacity(*size + 10);
                prost::Message::encode(&msg, &mut buf).unwrap();
                black_box(buf)
            })
        });

        let mut encoded = Vec::new();
        prost::Message::encode(&msg, &mut encoded).unwrap();

        group.bench_with_input(BenchmarkId::new("decode", size), size, |b, _| {
            b.iter(|| {
                let decoded: BenchMessage = prost::Message::decode(encoded.as_slice()).unwrap();
                black_box(decoded)
            })
        });
    }

    group.finish();
}

/// Benchmark publish latency (time to publish a message)
fn bench_publish_latency(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("publish_latency");
    group.measurement_time(Duration::from_secs(10));

    // Create node and publisher once
    let (node, publisher) = rt.block_on(async {
        let node = Node::new("bench_publisher").await.unwrap();
        let publisher = node
            .publisher::<BenchMessage>("bench/latency")
            .build()
            .await
            .unwrap();
        (node, publisher)
    });

    for size in [64, 256, 1024, 4096].iter() {
        let msg = BenchMessage::with_size(*size);
        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(BenchmarkId::new("publish", size), size, |b, _| {
            b.iter(|| {
                publisher.publish(black_box(&msg)).unwrap();
            })
        });
    }

    drop(publisher);
    drop(node);
    group.finish();
}

/// Benchmark end-to-end pub/sub throughput
fn bench_pubsub_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("pubsub_throughput");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(20);

    for size in [256, 1024, 4096].iter() {
        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(BenchmarkId::new("roundtrip", size), size, |b, &size| {
            b.to_async(&rt).iter(|| async move {
                let received = Arc::new(AtomicUsize::new(0));
                let received_clone = received.clone();

                // Create pub and sub nodes
                let pub_node = Node::new("bench_pub").await.unwrap();
                let sub_node = Node::new("bench_sub").await.unwrap();

                // Create subscriber first
                let _subscriber = sub_node
                    .subscriber::<BenchMessage>("bench/throughput")
                    .build(move |_msg| {
                        received_clone.fetch_add(1, Ordering::SeqCst);
                    })
                    .await
                    .unwrap();

                // Give subscriber time to connect
                tokio::time::sleep(Duration::from_millis(100)).await;

                // Create publisher
                let publisher = pub_node
                    .publisher::<BenchMessage>("bench/throughput")
                    .build()
                    .await
                    .unwrap();

                // Publish messages
                let msg = BenchMessage::with_size(size);
                let num_messages = 100;

                for _ in 0..num_messages {
                    publisher.publish(&msg).unwrap();
                }

                // Wait for messages and process callbacks
                let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
                while received.load(Ordering::SeqCst) < num_messages
                    && tokio::time::Instant::now() < deadline
                {
                    sub_node.spin_once().unwrap();
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }

                black_box(received.load(Ordering::SeqCst))
            })
        });
    }

    group.finish();
}

/// Benchmark callback executor performance
fn bench_executor(c: &mut Criterion) {
    use zenobuf_core::executor::CallbackExecutor;

    let mut group = c.benchmark_group("executor");

    group.bench_function("enqueue_process_100", |b| {
        let executor = CallbackExecutor::new();
        let counter = Arc::new(AtomicUsize::new(0));

        b.iter(|| {
            // Enqueue 100 callbacks
            for _ in 0..100 {
                let counter = counter.clone();
                executor.enqueue(Box::new(move || {
                    counter.fetch_add(1, Ordering::Relaxed);
                }));
            }

            // Process all
            let processed = executor.process_pending();
            black_box(processed)
        })
    });

    group.bench_function("enqueue_only", |b| {
        let executor = CallbackExecutor::new();
        let counter = Arc::new(AtomicUsize::new(0));

        b.iter(|| {
            let counter = counter.clone();
            executor.enqueue(Box::new(move || {
                counter.fetch_add(1, Ordering::Relaxed);
            }));
        });

        // Clean up
        executor.process_pending();
    });

    group.finish();
}

/// Benchmark QoS profile creation and mapping
fn bench_qos(c: &mut Criterion) {
    use zenobuf_core::qos::{QosPreset, Reliability};

    let mut group = c.benchmark_group("qos");

    group.bench_function("profile_default", |b| {
        b.iter(|| black_box(QosProfile::default()))
    });

    group.bench_function("profile_from_preset", |b| {
        b.iter(|| {
            let profile: QosProfile = QosPreset::SensorData.into();
            black_box(profile)
        })
    });

    group.bench_function("profile_builder", |b| {
        b.iter(|| {
            black_box(
                QosProfile::new()
                    .reliability(Reliability::BestEffort)
                    .depth(5),
            )
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_message_encoding,
    bench_publish_latency,
    bench_pubsub_throughput,
    bench_executor,
    bench_qos,
);
criterion_main!(benches);
