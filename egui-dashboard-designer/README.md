# EGUI Dashboard Designer

This project is a dashboard designing application built using EGUI, a simple and fast GUI library for Rust. The application allows users to create and customize dashboards with various widgets.

## Project Structure

```
egui-dashboard-designer
├── src
│   ├── main.rs          # Entry point of the application
│   ├── app.rs           # Main application structure and state management
│   ├── components       # UI components for the dashboard
│   │   ├── mod.rs       # Module for components
│   │   ├── dashboard.rs  # Main dashboard view
│   │   ├── widget_panel.rs # UI for adding and configuring widgets
│   │   └── property_panel.rs # UI for modifying widget properties
│   ├── widgets          # Dashboard widgets
│   │   ├── mod.rs       # Module for widgets
│   │   ├── chart.rs     # Chart widget
│   │   ├── gauge.rs     # Gauge widget
│   │   └── table.rs     # Table widget
│   └── utils            # Utility functions
│       ├── mod.rs       # Module for utilities
│       └── layout.rs     # Layout management functions
├── Cargo.toml           # Rust project configuration
└── README.md            # Project documentation
```

## Setup Instructions

1. **Clone the repository:**
   ```
   git clone https://github.com/yourusername/egui-dashboard-designer.git
   cd egui-dashboard-designer
   ```

2. **Install Rust:**
   Ensure you have Rust installed on your machine. You can install it from [rustup.rs](https://rustup.rs/).

3. **Build the project:**
   ```
   cargo build
   ```

4. **Run the application:**
   ```
   cargo run
   ```

## Usage Guidelines

- Upon running the application, you will be presented with a blank dashboard.
- Use the Widget Panel to add various widgets such as charts, gauges, and tables.
- Select a widget to modify its properties using the Property Panel.
- Arrange the layout of your dashboard as needed.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any suggestions or improvements.