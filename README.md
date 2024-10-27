# mdbook-reading-time [![Latest Version](https://img.shields.io/crates/v/mdbook-reading-time.svg)](https://crates.io/crates/mdbook-reading-time) [![GH Actions](https://github.com/pawurb/mdbook-reading-time/actions/workflows/ci.yml/badge.svg)](https://github.com/pawurb/mdbook-reading-time/actions)

A processor for [mdBook](https://github.com/rust-lang/mdBook) that calculates the reading time and word count of each chapter.

## Usage

```bash
cargo install mdbook-reading-time
```

In `book.toml`

```toml
[preprocessor.reading-time]
```

This configuration replaces:

- `{{ #word_count }}` with the number of words in the chapter.
- `{{ #reading_time }}` with reading time, e.g. `9 minutes`.

You can customize the default `200` WPM reading speed:

```toml
[preprocessor.reading-time]
words-per-minute = 250
```
