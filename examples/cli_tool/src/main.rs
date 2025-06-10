use clap::{Parser, Subcommand};
use shared::{create_greeting, factorial, Message};
use std::fs;

#[derive(Parser)]
#[command(name = "cli_tool")]
#[command(about = "A sample CLI tool built with Rust")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Greet someone
    Greet {
        /// Name of the person to greet
        #[arg(short, long)]
        name: String,

        /// Number of times to greet
        #[arg(short, long, default_value_t = 1)]
        count: u8,
    },
    /// Calculate factorial
    Factorial {
        /// Number to calculate factorial for
        number: u64,
    },
    /// File operations
    File {
        #[command(subcommand)]
        action: FileCommands,
    },
}

#[derive(Subcommand)]
enum FileCommands {
    /// Create a JSON file
    Create {
        /// Path to create the file
        path: String,
        /// Content for the file
        #[arg(short, long)]
        content: Option<String>,
    },
    /// Read a JSON file
    Read {
        /// Path to read the file from
        path: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Greet { name, count } => {
            let greeting = create_greeting(&name);
            for i in 1..=count {
                if count > 1 {
                    println!("{}. {}", i, greeting);
                } else {
                    println!("{}", greeting);
                }
            }
        }
        Commands::Factorial { number } => {
            let result = factorial(number);
            println!("Factorial of {} is: {}", number, result);
        }
        Commands::File { action } => match action {
            FileCommands::Create { path, content } => {
                let message =
                    Message::new(content.unwrap_or_else(|| "Hello from Rust CLI!".to_string()))
                        .with_timestamp(chrono::Utc::now().to_rfc3339())
                        .with_metadata("version".to_string(), "1.0.0".to_string());

                let json = serde_json::to_string_pretty(&message)?;
                fs::write(&path, json)?;
                println!("✅ Created file: {}", path);
            }
            FileCommands::Read { path } => {
                let content = fs::read_to_string(&path)?;
                let message: Message = serde_json::from_str(&content)?;
                println!("📄 File content:");
                println!("  Content: {}", message.content);
                if let Some(timestamp) = message.timestamp {
                    println!("  Timestamp: {}", timestamp);
                }
                if let Some(metadata) = message.metadata {
                    println!("  Metadata: {:?}", metadata);
                }
            }
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use shared::factorial;

    #[test]
    fn test_factorial_from_shared() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }
}
