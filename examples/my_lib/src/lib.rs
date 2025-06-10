use anyhow::Result;
use serde::{Deserialize, Serialize};

// Re-export commonly used functions from shared crate
pub use shared::{factorial, fibonacci};

/// A simple calculator library demonstrating Rust best practices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calculator {
    name: String,
}

impl Calculator {
    /// Create a new calculator instance
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Add two numbers
    pub fn add(&self, left: f64, right: f64) -> f64 {
        left + right
    }

    /// Subtract two numbers
    pub fn subtract(&self, left: f64, right: f64) -> f64 {
        left - right
    }

    /// Multiply two numbers
    pub fn multiply(&self, left: f64, right: f64) -> f64 {
        left * right
    }

    /// Divide two numbers, returning an error if dividing by zero
    pub fn divide(&self, left: f64, right: f64) -> Result<f64> {
        if right == 0.0 {
            anyhow::bail!("Division by zero is not allowed");
        }
        Ok(left / right)
    }

    /// Get the calculator's name
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_add() {
        let calc = Calculator::new("Test Calculator");
        assert_eq!(calc.add(2.0, 2.0), 4.0);
    }

    #[test]
    fn test_calculator_subtract() {
        let calc = Calculator::new("Test Calculator");
        assert_eq!(calc.subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_calculator_multiply() {
        let calc = Calculator::new("Test Calculator");
        assert_eq!(calc.multiply(3.0, 4.0), 12.0);
    }

    #[test]
    fn test_calculator_divide() {
        let calc = Calculator::new("Test Calculator");
        assert_eq!(calc.divide(10.0, 2.0).unwrap(), 5.0);
    }

    #[test]
    fn test_calculator_divide_by_zero() {
        let calc = Calculator::new("Test Calculator");
        assert!(calc.divide(10.0, 0.0).is_err());
    }

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
    fn test_calculator_serialization() {
        let calc = Calculator::new("Serializable Calculator");
        let json = serde_json::to_string(&calc).unwrap();
        let deserialized: Calculator = serde_json::from_str(&json).unwrap();
        assert_eq!(calc.name(), deserialized.name());
    }
}
