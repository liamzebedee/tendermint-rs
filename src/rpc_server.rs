use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use warp::Filter;

pub struct Server<T> {
    sender: mpsc::Sender<T>,
    // receiver: mpsc::Receiver<T>,
    receiver: Arc<Mutex<mpsc::Receiver<T>>>,
    pub port: u16,
}

impl<T> Server<T>
where
    T: Send + Sync + 'static + Serialize + DeserializeOwned + std::fmt::Debug + Clone,
{
    pub fn new(port: u16) -> Self {
        let (sender, receiver) = mpsc::channel(100);
        Server { sender, receiver: Arc::new(Mutex::new(receiver)), port }
    }

    pub fn get_receiver(&self) -> Arc<Mutex<mpsc::Receiver<T>>> {
        self.receiver.clone()
    }

    pub async fn run(&self) {
        let sender = self.sender.clone();
        let port = self.port;
        println!("Server running on port {}", port);

        // Define the warp route that listens on /inbox and sends incoming messages to the sender
        let inbox_route = warp::path("inbox").and(warp::post()).and(warp::body::json()).and_then(
            move |message: T| {
                // Print message JSON, prefixed with port [:3030] Received message: Message { data:
                // "Hello, world!" }
                println!("[:{}] Received message: {:?}", port, message);

                let sender = sender.clone();
                async move {
                    sender.send(message).await.unwrap(); // Send the message to the channel
                    Ok::<_, warp::Rejection>(warp::reply::json(&"Message received"))
                }
            },
        );

        // Start the server
        warp::serve(inbox_route).run(([127, 0, 0, 1], self.port)).await;
    }
}
