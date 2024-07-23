use std::{sync::Arc, time::Duration};

use actix_web::rt::time::interval;
use actix_web_lab::{sse::{self, Sse}, util::InfallibleStream};
use futures::future;
use std::sync::Mutex; // probably need a parking_lot crate with own Mutex
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    clients: Vec<mpsc::Sender<sse::Event>>,
}

impl Broadcaster {
    pub fn create() -> Arc<Self> {
        let this = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });
        Broadcaster::spawn_ping(Arc::clone(&this));
        this
    }

    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().unwrap().clients.clone();
        let mut ok_clients = Vec::new();
        for client in clients {
            if client
                .send(sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.push(client.clone());
            }
        }
        self.inner.lock().unwrap().clients = ok_clients;
    }

    pub async fn new_client(&self) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
        let (tx, rx) = mpsc::channel(10);
        tx.send(sse::Data::new("connected").into()).await.unwrap();

        self.inner.lock().unwrap().clients.push(tx);
        Sse::from_infallible_receiver(rx)
    }

    pub async fn broadcast(&self, msg: &str) {
        let clients = self.inner.lock().unwrap().clients.clone();
        let send_futures = clients 
            .iter()
            .map(|client| client.send(sse::Data::new(msg).into()));
        let _ = future::join_all(send_futures).await;
    }
}
