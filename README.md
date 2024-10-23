# Project overview 

This project demonstrates the creation of a simple multiplication tool using the Rust programming language. As part of the Data Engineering Systems, this assignment showcases how fundamental programming concepts such as multiplication operations can be implemented and automated in Rust, while incorporating best practices like continuous integration and deployment (CI/CD) pipelines.

[![CI](https://github.com/nogibjj/AfagR_DE_assignment7/actions/workflows/CI.yml/badge.svg)](https://github.com/nogibjj/AfagR_DE_assignment7/actions/workflows/CI.yml)


# Technologies Used

- Rust: The core functionality of the project is written in Rust, a systems programming language known for its memory safety and performance.
- GitHub Actions: Used to automate the testing, building, and artifact uploading processes for the CI/CD pipeline.
- Docker (if applicable): Can be used to containerize the application for easy deployment and scalability.

# Getting Started

## Prerequisites

To run this project locally, you need to have the following tools installed:

- Rust (https://www.rust-lang.org/tools/install)
- Cargo (Rust’s package manager, typically installed with Rust)
- Docker (optional, if you're containerizing the project)

# CI/CD Pipeline

This project is equipped with a CI/CD pipeline using GitHub Actions, which automates the following tasks:

- Testing: The project is tested automatically on each push.
- Building: The code is compiled into a binary in release mode.
- Artifact Upload: The generated binary is uploaded as an artifact, allowing users to download and run the compiled program.


