# X-Downloader

X-Downloader is a command-line tool written in Rust that allows users to track and reinstall packages on their system using a JSON file.

## Features

- **Package Tracking**: Track packages installed on your system.
- **Generate JSON File**: Generate a JSON file containing the list of tracked packages.
- **Install from JSON File**: Reinstall packages from a JSON file after system crashes or migrations.

## Installation

To install X-Downloader, you'll need to have Rust and Cargo installed. Then, you can simply clone the repository and build the project using Cargo:

```sh
git clone https://github.com/yourusername/x-downloader.git
cd x-downloader
cargo build --release
```

## Usage

### Tracking Packages

To track a package, use the `track` command followed by the package name:

```sh
x-downloader track PACKAGE_NAME
```

### Generating a JSON File

To generate a JSON file containing the tracked packages, use the `generate-file` command:

```sh
x-downloader generate-file --file FILE_NAME.json
```

### Installing from a JSON File

To install packages listed in a JSON file, use the `install-from-file` command:

```sh
x-downloader install-from-file --file FILE_NAME.json
```

## Example

Here's an example workflow:

1. Track packages:
   ```sh
   x-downloader track curl
   x-downloader track wget
   ```

2. Generate a JSON file:
   ```sh
   x-downloader generate-file --file packages.json
   ```

3. After a system crash or migration, reinstall packages from the JSON file:
   ```sh
   x-downloader install-from-file --file packages.json
   ```

## Contributing

Contributions are welcome! If you'd like to contribute to X-Downloader, please fork the repository, make your changes, and submit a pull request.
