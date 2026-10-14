//! Observe the library's native relay boundary without inspecting media or changing negotiation.

use std::net::SocketAddr;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

use anyhow::Result;
use async_channel::Receiver;
use bytes::Bytes;
use whatsapp_rust::async_trait;
use whatsapp_rust::voip::transport::{
    RelayMediaChannelFactory, RelayPacketKind, classify_relay_packet,
};
use whatsapp_rust::voip::{
    RelayEndpointParams, RelayTransport, RelayTransportEvent, RelayTransportFactory,
    RelayTransportProvider,
};

#[derive(Default)]
struct Link {
    received: AtomicU64,
    handed: AtomicU64,
    rtp: AtomicU64,
    rtcp: AtomicU64,
    stun_like: AtomicU64,
    other: AtomicU64,
    dropped: AtomicU64,
    submitted: AtomicU64,
    send_failed: AtomicU64,
    closed: AtomicBool,
}

pub(crate) struct Trace {
    generation: u64,
    links: Mutex<Vec<Arc<Link>>>,
}

impl Trace {
    pub fn install(client: &whatsapp_rust::Client, generation: u64) -> Arc<Self> {
        let trace = Arc::new(Self {
            generation,
            links: Mutex::new(Vec::new()),
        });
        client.set_relay_transport_provider(Arc::new(Provider(trace.clone())));
        trace
    }

    pub fn log(&self, marker: &str) {
        for (connection, link) in self
            .links
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .enumerate()
        {
            log::info!(
                "[CALL] relay at={} generation={} connection={} received={} handed={} rtp={} rtcp={} stun_like={} other={} dropped={} submitted={} send_failed={} closed={}",
                marker,
                self.generation,
                connection,
                link.received.load(Ordering::Relaxed),
                link.handed.load(Ordering::Relaxed),
                link.rtp.load(Ordering::Relaxed),
                link.rtcp.load(Ordering::Relaxed),
                link.stun_like.load(Ordering::Relaxed),
                link.other.load(Ordering::Relaxed),
                link.dropped.load(Ordering::Relaxed),
                link.submitted.load(Ordering::Relaxed),
                link.send_failed.load(Ordering::Relaxed),
                link.closed.load(Ordering::Relaxed),
            );
        }
    }
}

struct Provider(Arc<Trace>);
#[async_trait]
impl RelayTransportProvider for Provider {
    async fn factory(&self, relay: &RelayEndpointParams) -> Result<Arc<dyn RelayTransportFactory>> {
        // The same native factory the client uses by default, with the same dial policy.
        Ok(Arc::new(Factory {
            inner: RelayMediaChannelFactory::new(relay.addr, Arc::new(whatsapp_rust::TokioRuntime)),
            trace: self.0.clone(),
        }))
    }
}

struct Factory {
    inner: RelayMediaChannelFactory,
    trace: Arc<Trace>,
}
#[async_trait]
impl RelayTransportFactory for Factory {
    async fn connect(&self) -> Result<(Arc<dyn RelayTransport>, Receiver<RelayTransportEvent>)> {
        let (transport, events) = self.inner.connect().await?;
        Ok(observe(transport, events, self.trace.clone()))
    }
}

struct Observed {
    inner: Arc<dyn RelayTransport>,
    trace: Arc<Trace>,
    link: Arc<Link>,
    pump: tokio::task::JoinHandle<()>,
}

fn observe(
    inner: Arc<dyn RelayTransport>,
    input: Receiver<RelayTransportEvent>,
    trace: Arc<Trace>,
) -> (Arc<dyn RelayTransport>, Receiver<RelayTransportEvent>) {
    let link = Arc::new(Link::default());
    trace
        .links
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .push(link.clone());
    // One extra slot, rather than another media backlog. Backpressure stays with the native
    // transport, whose bounded queue already drops and reports excess media. Closing either side
    // retires the pump even when the peer sends nothing.
    let (output, events) = async_channel::bounded(1);
    let counters = link.clone();
    let pump = tokio::spawn(async move {
        loop {
            let event = tokio::select! {
                event = input.recv() => match event { Ok(event) => event, Err(_) => break },
                _ = output.closed() => break,
            };
            match &event {
                RelayTransportEvent::PacketReceived(packet) => {
                    counters.received.fetch_add(1, Ordering::Relaxed);
                    let counter = match classify_relay_packet(packet) {
                        RelayPacketKind::Rtp => &counters.rtp,
                        RelayPacketKind::Rtcp => &counters.rtcp,
                        RelayPacketKind::Stun => &counters.stun_like,
                        RelayPacketKind::Other => &counters.other,
                    };
                    counter.fetch_add(1, Ordering::Relaxed);
                }
                RelayTransportEvent::InboundDropped(count) => {
                    counters
                        .dropped
                        .fetch_add(u64::from(*count), Ordering::Relaxed);
                }
                _ => {}
            }
            let packet = matches!(event, RelayTransportEvent::PacketReceived(_));
            if output.send(event).await.is_err() {
                break;
            }
            if packet {
                counters.handed.fetch_add(1, Ordering::Relaxed);
            }
        }
        counters.closed.store(true, Ordering::Relaxed);
    });
    (
        Arc::new(Observed {
            inner,
            trace,
            link,
            pump,
        }),
        events,
    )
}

#[async_trait]
impl RelayTransport for Observed {
    async fn send(&self, data: Bytes) -> Result<()> {
        let result = self.inner.send(data).await;
        if result.is_ok() {
            self.link.submitted.fetch_add(1, Ordering::Relaxed);
        } else {
            self.link.send_failed.fetch_add(1, Ordering::Relaxed);
        }
        result
    }

    async fn disconnect(&self) {
        self.inner.disconnect().await;
        self.pump.abort();
        self.link.closed.store(true, Ordering::Relaxed);
    }

    async fn reconnect(
        &self,
        endpoint: SocketAddr,
    ) -> Result<(Arc<dyn RelayTransport>, Receiver<RelayTransportEvent>)> {
        let (transport, events) = self.inner.reconnect(endpoint).await?;
        Ok(observe(transport, events, self.trace.clone()))
    }
}

impl Drop for Observed {
    fn drop(&mut self) {
        self.pump.abort();
        self.link.closed.store(true, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[derive(Default)]
    struct FakeTransport(Mutex<Vec<Bytes>>);
    #[async_trait]
    impl RelayTransport for FakeTransport {
        async fn send(&self, packet: Bytes) -> Result<()> {
            self.0.lock().unwrap().push(packet);
            Ok(())
        }
        async fn disconnect(&self) {}
    }

    fn trace(generation: u64) -> Arc<Trace> {
        Arc::new(Trace {
            generation,
            links: Mutex::new(Vec::new()),
        })
    }

    #[tokio::test]
    async fn relay_tap_preserves_packets_control_and_call_isolation() {
        let trace = trace(1);
        let other_call = super::tests::trace(2);
        let native = Arc::new(FakeTransport::default());
        let (input, events) = async_channel::bounded(2);
        let (transport, observed) = observe(native.clone(), events, trace.clone());
        let link = trace.links.lock().unwrap()[0].clone();
        let (_other_input, other_events) = async_channel::bounded(1);
        let (_other_transport, _other_output) = observe(
            Arc::new(FakeTransport::default()),
            other_events,
            other_call.clone(),
        );
        input.send(RelayTransportEvent::Connected).await.unwrap();
        assert!(matches!(
            observed.recv().await.unwrap(),
            RelayTransportEvent::Connected
        ));
        for packet in [
            Bytes::from_static(&[0x90, 120, 1]),
            Bytes::from_static(&[0x80, 200, 1]),
            Bytes::from_static(&[0, 1, 2]),
        ] {
            input
                .send(RelayTransportEvent::PacketReceived(packet.clone()))
                .await
                .unwrap();
            let received = tokio::time::timeout(Duration::from_secs(1), observed.recv())
                .await
                .unwrap()
                .unwrap();
            assert!(
                matches!(received, RelayTransportEvent::PacketReceived(bytes) if bytes == packet)
            );
            transport.send(packet).await.unwrap();
        }
        input
            .send(RelayTransportEvent::InboundDropped(7))
            .await
            .unwrap();
        assert!(matches!(
            observed.recv().await.unwrap(),
            RelayTransportEvent::InboundDropped(7)
        ));
        assert_eq!(link.received.load(Ordering::Relaxed), 3);
        assert_eq!(link.submitted.load(Ordering::Relaxed), 3);
        assert_eq!(link.dropped.load(Ordering::Relaxed), 7);
        assert_eq!(native.0.lock().unwrap().len(), 3);
        assert_eq!(
            other_call.links.lock().unwrap()[0]
                .received
                .load(Ordering::Relaxed),
            0
        );
        drop(transport);
        tokio::time::timeout(Duration::from_secs(1), input.closed())
            .await
            .unwrap();
        assert!(link.closed.load(Ordering::Relaxed));
    }

    #[tokio::test]
    async fn dropping_a_stalled_sink_retires_the_receive_pump() {
        let trace = trace(3);
        let (input, events) = async_channel::bounded(1);
        let (transport, observed) =
            observe(Arc::new(FakeTransport::default()), events, trace.clone());
        for _ in 0..2 {
            input.send(RelayTransportEvent::Connected).await.unwrap();
        }
        drop(observed);
        tokio::time::timeout(Duration::from_secs(1), input.closed())
            .await
            .unwrap();
        assert!(
            trace.links.lock().unwrap()[0]
                .closed
                .load(Ordering::Relaxed)
        );
        drop(transport);
    }
}
