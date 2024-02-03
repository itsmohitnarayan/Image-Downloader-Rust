# Rust Image Downloader 🌐🦀

A simple Rust program to download an image from the web using reqwest and tempfile.

## Features 🚀

- Downloads an image from a specified URL.
- Saves the image in a temporary directory using tempfile.

## Prerequisites 🛠️

Make sure you have Rust installed on your system.

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

3. Provide the URL of the image when prompted.

## Dependencies 📚

- reqwest: For making HTTP requests.
- tempfile: For creating temporary directories.

```
tempfile = "3.9.0"
error-chain = "0.12.4"
reqwest = "0.11.24"
tokio = { version = "1.36.0", features = ["full"] }
```

## Error Handling 🚨

The code uses the `error_chain` crate to handle errors, including I/O errors and HTTP request errors.



Feel free to contribute and improve this Rust image downloader! 🤝