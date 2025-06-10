//! Shared utilities and types for the Rust template workspace
//!
//! This crate demonstrates how to share common functionality across
//! multiple projects in a Cargo workspace.

use serde::{Deserialize, Serialize};

/// Common data structures used across multiple examples
pub mod types {
    use super::*;

    /// A generic message structure that can be used across different examples
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct Message {
        pub content: String,
        pub timestamp: Option<String>,
        pub metadata: Option<std::collections::HashMap<String, String>>,
    }

    impl Message {
        pub fn new(content: impl Into<String>) -> Self {
            Self {
                content: content.into(),
                timestamp: None,
                metadata: None,
            }
        }

        pub fn with_timestamp(mut self, timestamp: String) -> Self {
            self.timestamp = Some(timestamp);
            self
        }

        pub fn with_metadata(mut self, key: String, value: String) -> Self {
            self.metadata
                .get_or_insert_with(Default::default)
                .insert(key, value);
            self
        }
    }

    /// A generic user info structure
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub struct UserInfo {
        pub name: String,
        pub email: Option<String>,
        pub metadata: Option<std::collections::HashMap<String, String>>,
    }

    impl UserInfo {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                email: None,
                metadata: None,
            }
        }

        pub fn with_email(mut self, email: String) -> Self {
            self.email = Some(email);
            self
        }
    }
}

/// Common mathematical utilities
pub mod math {
    /// Calculate the factorial of a number
    ///
    /// # Examples
    /// ```
    /// use shared::math::factorial;
    /// assert_eq!(factorial(5), 120);
    /// assert_eq!(factorial(0), 1);
    /// ```
    pub fn factorial(n: u64) -> u64 {
        match n {
            0 | 1 => 1,
            _ => n * factorial(n - 1),
        }
    }

    /// Calculate the nth Fibonacci number
    ///
    /// # Examples
    /// ```
    /// use shared::math::fibonacci;
    /// assert_eq!(fibonacci(10), 55);
    /// assert_eq!(fibonacci(0), 0);
    /// ```
    pub fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    /// Add two numbers (simple utility for examples)
    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }
}

/// String utilities
pub mod text {
    /// Reverse a string
    ///
    /// # Examples
    /// ```
    /// use shared::text::reverse_string;
    /// assert_eq!(reverse_string("hello"), "olleh");
    /// ```
    pub fn reverse_string(input: &str) -> String {
        input.chars().rev().collect()
    }

    /// Create a greeting message
    pub fn create_greeting(name: &str) -> String {
        format!("Hello, {}! 👋", name)
    }
}

// Re-export commonly used items for convenience
pub use math::{add, factorial, fibonacci};
pub use text::{create_greeting, reverse_string};
pub use types::{Message, UserInfo};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"), "tsur");
    }

    #[test]
    fn test_message() {
        let msg = Message::new("test")
            .with_timestamp("2023-01-01".to_string())
            .with_metadata("key".to_string(), "value".to_string());

        assert_eq!(msg.content, "test");
        assert_eq!(msg.timestamp, Some("2023-01-01".to_string()));
    }

    #[test]
    fn test_user_info() {
        let user = UserInfo::new("Alice").with_email("alice@example.com".to_string());

        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, Some("alice@example.com".to_string()));
    }
}
