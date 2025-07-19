# holderplace

A simple and fast command-line tool for generating placeholder images. Highly customizable for your development or design needs.

## Features

*   Specify custom image dimensions (width and height).
*   Choose custom background and foreground (text) colors using hex codes.
*   Add custom text to be displayed in the center of the image.
*   Supports multiple image formats: `png`, `jpeg`, `gif`, `bmp`, and `webp`.
*   Cross-platform: works on Windows, macOS, and Linux.

## Installation

To build from source, clone the repository and run:

```sh
cargo build --release
```

The binary will be available at `./target/release/holderplace`.

You can also install the binary directly using `cargo`:

```sh
cargo install --path .
```

## Usage

The core command requires you to specify the `width` and `height` of the desired image.

### Basic Usage

To generate a 300x200 placeholder image with default settings:

```sh
holderplace --width 300 --height 200
```

This will create a file named `out.png` in the current directory with a gray background and dark text.

### Advanced Usage

You can customize the image with various options. For example, to create a blue 800x600 JPEG image with the text "800x600" in white:

```sh
holderplace --width 800 --height 600 --bg "#007bff" --fg "#ffffff" --text "800x600" --output "my-placeholder.jpeg" --format "jpeg"
```

### All Command-line Options

You can see all available options by running the tool with the `--help` flag.

```sh
$ holderplace --help
Placeholder image generator

Usage: holderplace --width <WIDTH> --height <HEIGHT> [OPTIONS]

Options:
  -w, --width <WIDTH>
          Output image width
  -h, --height <HEIGHT>
          Output image height
      --bg <BG>
          Background color in hex, e.g. "#cccccc"
          [default: #cccccc]
      --fg <FG>
          Foreground/text color in hex, e.g. "#333333"
          [default: #333333]
  -t, --text <TEXT>
          Text to display in the center
          [default: Placeholder]
  -f, --format <FORMAT>
          Output format: png, jpeg, gif, bmp, webp
          [default: png]
  -o, --output <OUTPUT>
          Output file path
          [default: out.png]
      --help
          Print help (see more with '--help')
      --version
          Print version
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.