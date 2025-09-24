# Pie

Pie is a simple inference engine and knowledge base system in Rust. The project aims to provide efficient and scalable processing capabilities by leveraging Rust's safety and performance features, as well as by providing a powerful inference engine.

## Features

- **Inference Engine**: Processes and evaluates logic rules.
- **Knowledge Base**: Manages and stores dynamic data for inference and decision-making.
- **Interactive Query Mode**: Allows users to enter queries interactively.
- **Proof Mode**: Prove specific facts using command-line options.
- Modular design for ease of extension and integration.

## Project Structure

- `src/main.rs`: Entry point of the application.
- `src/inference_engine.rs`: Contains the logic for the inference engine.
- `src/knowledge_base.rs`: Implements the knowledge storage and retrieval mechanisms.
- `src/tests.rs`: Unit tests for the project components.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Building the Project

Open a terminal in the project directory and run:

```bash
cargo build
```

This will compile the project and its dependencies.

### Running the Project

To run the project, execute:

```bash
cargo run
```

#### Available Command-Line Options

- `--file` or `-f <path>`: Specify a custom knowledge base file.
- `--debug` or `-d`: Enable debug mode.
- `--query` or `-q`: Enter interactive query mode.
- `--prove` or `-p <fact>`: Prove a specific fact.
- `--help` or `-h`: Display help message.

### Running Tests

Run the tests using:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.