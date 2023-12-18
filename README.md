# html-datetime-local

## html-datetime-local

[![GitHub license](https://img.shields.io/github/license/tomsik68/html-datetime-local?style=for-the-badge)](https://github.com/tomsik68/html-datetime-local/blob/master/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/tomsik68/html-datetime-local/rust.yml?branch=master&style=for-the-badge)](https://github.com/tomsik68/html-datetime-local/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/html-datetime-local?style=for-the-badge)](https://crates.io/crates/html-datetime-local)
[![Crates.io (latest)](https://img.shields.io/crates/dv/html-datetime-local?style=for-the-badge)](https://crates.io/crates/html-datetime-local)

### Overview

`html-datetime-local` is a Rust library for parsing local date and time strings based on the [WHATWG HTML Living Standard](https://html.spec.whatwg.org/multipage/common-microsyntaxes.html#local-dates-and-times).

This may be helpful for server-side code that deals with values from `<input type="datetime-local" />`.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
html-datetime-local = "0.1"
```

Then, in your Rust code:
```rust
use html_datetime_local::Datetime;
use std::str::FromStr;

let input = "2023-12-31T23:59:59";
match Datetime::from_str(input) {
    Ok(datetime) => println!("Parsed datetime: {:?}", datetime),
    Err(err) => eprintln!("Error parsing datetime: {}", err),
}
```

## Contributing

Pull requests and bug reports are welcome! If you have any questions or suggestions, feel free to open an issue.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

Special thanks to [ChatGPT](https://www.openai.com/gpt), an AI language model by OpenAI, for providing invaluable assistance during the development of this project. ChatGPT helped with code suggestions, problem-solving, and provided guidance throughout the development process.

License: MIT
