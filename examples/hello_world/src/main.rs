use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    message: String,
    emoji: String,
}

#[tokio::main]
async fn main() {
    println!("🦀 Welcome to your Rust development environment!");
    println!("Happy coding! 🚀");

    let greeting = Greeting {
        message: "Hello from Rust!".to_string(),
        emoji: "🦀".to_string(),
    };

    println!("Structured greeting: {:?}", greeting);

    // Example of async operation
    println!("Running async example...");
    async_example().await;
}

async fn async_example() {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    println!("✅ Async operation completed!");
}
