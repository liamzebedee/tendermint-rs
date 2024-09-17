use reqwest::Client;
use serde::Serialize;
use std::{fmt::Debug, sync::Arc};
use tokio::sync::{mpsc, Mutex};

#[derive(Debug, Clone)]
pub struct RpcClient<T> {
    inbox_url: String,
    sender: mpsc::Sender<T>, // Exposes the mpsc::Sender to send messages
    client: Arc<Client>,     // Reqwest HTTP client wrapped in Arc for shared ownership
    receiver: Arc<Mutex<mpsc::Receiver<T>>>, // Internal receiver used by the background task
}

impl<T> RpcClient<T>
where
    T: Send + 'static + Serialize + Debug, // T must implement Serialize to be posted via HTTP
{
    // Initialize the RpcClient with a given channel capacity and reqwest client
    pub fn new(capacity: usize, inbox_url: String) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);
        RpcClient {
            inbox_url,
            sender,
            client: Arc::new(Client::new()),
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    // Start the task that listens to the mpsc::Receiver and sends HTTP POST requests
    pub async fn start(self) {
        let receiver = self.receiver.clone();
        let client = self.client.clone();

        // Spawn a background task that listens for messages and sends them to /inbox
        tokio::spawn(async move {
            let mut receiver = receiver.lock().await;

            // Loop over the messages in the receiver
            while let Some(message) = receiver.recv().await {
                let client = client.clone();
                let url = self.inbox_url.clone();

                // Send the HTTP POST request with the message
                tokio::spawn(async move {
                    match client.post(&url).json(&message).send().await {
                        Ok(response) => {
                            if response.status().is_success() {
                            } else {
                                eprintln!("Failed to send message: {:?}", response);
                                // print response body
                                eprintln!("Response body: {:?}", response.text().await);
                                eprintln!("Message: {:?}", message);
                            }
                        }
                        Err(err) => {
                            eprintln!("HTTP error: {:?}", err);
                        }
                    }
                });
            }
        });
    }

    // Expose the mpsc::Sender so that other parts of the code can send messages
    pub fn get_sender(&self) -> mpsc::Sender<T> {
        self.sender.clone()
    }
}
