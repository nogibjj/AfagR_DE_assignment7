### User Guide: Command-Line Calculator in Rust

#### Overview
This Rust-based tool is designed to perform multiplication operations. The tool takes two input values, multiplies them, and returns the product. It is built with Rust and includes CI/CD integration for automated testing and building.


#### Prerequisites
Before running the program, ensure the following are set up:

1. **Step 1: Install Rust**: Ensure Rust is installed. Here are the detailed steps for installing **rust**

- Open your terminal and run 

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

- Verify the installation by checking the Rust version:

   ```bash
   rustc --version
   ```

   Cargo: Installed along with Rust as its package manager.  


#### Usage Instructions

1. **Navigate to the Project Directory**:
   Make sure you're in the correct directory where the `Cargo.toml` file is located. You can do this with the following command:
   
   ```bash
   cd /path/to/your/rust/project
   ```

2. **Building a project**:
   Once inside the project directory, build the project using Cargo:
   
   ```bash
   cargo build
   ```
   For a release build:
   ```bash
   cargo build --release
   ```
This will generate the binary in the target/debug/ or target/release/ directory, depending on the build mode you choose.

3. **Run the Program**:
   After building the project, you can run the program by executing:

   ### Command Syntax:
   ```bash
   cargo run -- <operation> <num1> <num2>
   ```

   Or if using the release build:
   ```bash
   ./target/release/<operation>
   ```


   Where <operation> is the name of your binary, typically matching the package name defined in Cargo.toml.

#### Usage

   This tool performs multiplication of two numbers. It reads the numbers from the command line or standard input.
   
   ```bash
   cargo run -- multiplication 6 4
   ```
   **Output**:
   ```
   The multiplication of 6 and 4 is 24
   ```
   You can modify the input numbers to perform other multiplication operations.

#### Running Tests
   This project includes automated tests to ensure the multiplication functionality is working correctly.
   ```bash
   cargo test
   ```
   The test results will be displayed in the terminal, showing whether the tests passed or failed.

#### CI/CD Pipeline

This project is equipped with a CI/CD pipeline using GitHub Actions, which automates the following tasks:

- Testing: The project is tested automatically on each push.
- Building: The code is compiled into a binary in release mode.
- Artifact Upload: The built binary is uploaded as an artifact, allowing users to download and run the compiled program.
