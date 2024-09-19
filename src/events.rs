use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

pub struct EventSystem<T>
where
    T: Clone + Send + 'static,
{
    sender: broadcast::Sender<T>,
}

impl<T> Default for EventSystem<T>
where
    T: Clone + Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> EventSystem<T>
where
    T: Clone + Send + 'static,
{
    // Create a new EventSystem
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100); // Create a broadcast channel with 100 capacity
        EventSystem { sender }
    }

    // Function to subscribe to the event stream
    pub fn subscribe(&self) -> impl tokio_stream::Stream<Item = T> {
        let receiver = self.sender.subscribe();
        // Wrap receiver into a BroadcastStream which implements Stream trait
        BroadcastStream::new(receiver).filter_map(|res| res.ok())
    }

    // Function to publish an event
    pub fn publish(&self, event: T) {
        let _ = self.sender.send(event); // Ignore the error for simplicity
    }
}
