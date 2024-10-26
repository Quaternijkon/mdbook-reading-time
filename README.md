# mdbook-reading-time

A processor for [mdbook](https://github.com/rust-lang/mdBook) that calculates the reading time and word count of each chapter.

## Usage

```bash
cargo install mdbook-reading-time
```

In `book.toml`

```toml
[preprocessor.reading-time]
```

This configuration replaces:

`{{ #word_count }}` with the number of words in the chapter.
`{{ #reading_time }}` with reading time, e.g. `9 minutes`.



