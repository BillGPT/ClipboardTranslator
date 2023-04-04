# ClipboardTranslator

ClipboardTranslator is a real-time translation tool that monitors your clipboard content and automatically translates it using OpenAI GPT-3.5-turbo. It's designed to be simple, efficient, and easy to use.

## DEMO
![](https://github.com/BillGPT/ClipboardTranslator/demo.gif)

## Features
- [ ] Supports multiple languages
- [x] Real-time clipboard content monitoring
- [x] Automatic translation using OpenAI GPT-3.5-turbo
- [x] Streamed output for faster results
- [x] translate from English to simplified Chinese
## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already.
2. Clone this repository:

```bash
git clone https://github.com/BillGPT/ClipboardTranslator.git
```

3. Change to the project directory:

```bash
cd ClipboardTranslator
```

4. Build the project:

```bash
cargo build --release
```

5. Set your OpenAI API key as an environment variable:

```bash
export OPENAI_API_KEY=your_api_key_here
```

## Usage

1. Run the compiled binary:

```bash
./target/release/clipboard_translator
```

2. Copy any text to your clipboard, and the program will automatically translate it and display the result in the terminal.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
