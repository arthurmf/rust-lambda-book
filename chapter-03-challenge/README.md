# Crafting Lambda Functions in Rust

## Description
This project serves as a practical guide for building serverless applications using AWS Lambda with Rust. The repository includes examples and instructions to help users set up their environment and implement a URL shortener service.

## Installation Instructions
To get started, you need to install the following tools:
- [Rust](https://www.rust-lang.org/tools/install)
- [AWS CLI](https://aws.amazon.com/cli/)
- [Docker](https://www.docker.com/get-started)
- [AWS SAM CLI](https://aws.amazon.com/serverless/sam/)

Follow the installation instructions provided in the `README.md` file for more detailed steps.

## Usage Notes
This project contains a serverless URL shortener application built using AWS Lambda. Ensure that you have the necessary tools installed and configured (AWS CLI) before running the examples.

## Project File Summaries

### 1. README.md
This file contains the introductory information about the book "Crafting Lambda Functions in Rust," including requirements for running the examples found in this repository. It provides instructions on obtaining Rust and setting up AWS CLI, Docker, and SAM CLI.

### 2. chapter-03-challenge/.gitignore
This file specifies ignored files and directories for the project. It includes common patterns for Rust, such as compiled files and executables, and specific tools for Rust development and AWS Lambda tools.

### 3. chapter-03-challenge/Cargo.toml
This file defines the project's metadata and dependencies for the Rust project, specifying the name, version, and dependencies including `lambda_http`, `serde`, and `cuid2`. It also specifies a binary target for the serverless application.

### 4. chapter-03-challenge/samconfig.toml
This configuration file contains deployment parameters for the AWS Serverless Application Model (SAM). It specifies stack name, S3 prefix, region, and other settings for deploying the serverless application.

### 5. chapter-03-challenge/src/core.rs
This file contains the implementation of the URL shortening logic using a struct called `UrlShortener`. It handles creating shortened URLs, retrieving original URLs, and managing stored URLs in memory using a thread-safe approach (`RwLock`).

### 6. chapter-03-challenge/src/main.rs
This is the entry point for the AWS Lambda function. It contains the function handler for processing incoming HTTP requests, routing them based on HTTP methods (GET, POST), and providing responses according to the URL shortening functionality.

### 7. chapter-03-challenge/src/utils.rs
This file contains helper functions to create HTTP responses, including JSON responses, empty responses, and redirect responses, allowing for streamlined HTTP response handling in the Lambda function.

### 8. chapter-03-challenge/template.yaml
This AWS CloudFormation template defines serverless resources for the URL shortener service. It specifies the AWS Lambda function properties, including event sources (HTTP APIs for creating and retrieving shortened URLs).

## **About This README**
 This README was **automatically generated** by [ReadMeGenie](https://github.com/arthurmf/readmegenie/tree/main/src/readmegenie) — a multi-agent system designed to analyze GitHub repositories and create detailed documentation.
