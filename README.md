# Rust Image Downloader 🌐🦀

A simple Rust program that downloads an image from the web using reqwest, tempfile, and logging.

## Features 🚀

- Downloads an image from a specified URL.
- Saves the image in a temporary directory using tempfile.
- Logs download information using the `log` crate.

## Prerequisites 🛠️

Make sure you have Rust installed on your system. If not, you can install it by running:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Usage 📦

1. Clone the repository:

```bash
git clone https://github.com/itsmohitnarayan/Image-Downloader-Rust.git

cd Image-Downloader-Rust
```

2. Build and run the program:

```bash
cargo run
```

3. View logs for download information.

## Logging Information 📝

This program uses the `log` crate to log download information. View the logs to see details about the downloaded file.

## Dependencies 📚

- reqwest: For making HTTP requests.
- tempfile: For creating temporary directories.
- log: For logging information.

```
error-chain = "0.12.4"
reqwest = "0.11.24"
tokio = { version = "1.36.0", features = ["full"] }
tempfile = "3.9.0"
log = "0.4.20"
env_logger = "0.11.1"
```

Feel free to contribute and improve this Rust image downloader! 🤝