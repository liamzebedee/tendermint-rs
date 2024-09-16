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

// #[tokio::main]
// async fn main() {
//     let event_system = EventSystem::new();

//     // Subscriber 1
// let mut subscriber1 = event_system.subscribe();
// tokio::spawn(async move {
//     while let Some(event) = subscriber1.next().await {
//         println!("Subscriber 1 received: {}", event);
//     }
// });

//     // Subscriber 2
//     let mut subscriber2 = event_system.subscribe();
//     tokio::spawn(async move {
//         while let Some(event) = subscriber2.next().await {
//             println!("Subscriber 2 received: {}", event);
//         }
//     });

//     // Publish events
//     event_system.publish("Event 1".to_string());
//     event_system.publish("Event 2".to_string());

//     // Allow some time for the events to be processed
//     tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
// }
