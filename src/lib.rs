use mdbook::errors::Error;

use mdbook::{
    book::{Book, Chapter},
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};

#[derive(Default)]
pub struct ReadingTime;

impl ReadingTime {
    pub fn new() -> ReadingTime {
        ReadingTime
    }
}

static WORDS_PER_MINUTE: usize = 200;

impl Preprocessor for ReadingTime {
    fn name(&self) -> &str {
        "reading-time"
    }

    fn run(&self, _ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        let mut error: Option<Error> = None;

        let mut book = book;
        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                if let Err(err) = handle_chapter(chapter) {
                    error = Some(err)
                }
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

fn handle_chapter(chapter: &mut Chapter) -> Result<(), Error> {
    let word_count = chapter.content.split_whitespace().count();
    let reading_time = word_count / WORDS_PER_MINUTE;
    let minutes = if reading_time == 1 {
        "minute"
    } else {
        "minutes"
    };

    chapter.content = chapter
        .content
        .replace("{{ #word_count }}", word_count.to_string().as_str())
        .replace(
            "{{ #reading_time }}",
            &format!("{} {minutes}", reading_time.to_string().as_str()),
        );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reading_preprocessor_run() {
        let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "nop": {}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n #{{ #word_count }}\n\n{{ #reading_time }}",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let input_json = input_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let result = ReadingTime::new().run(&ctx, book);
        assert!(result.is_ok());

        let actual_book = result.unwrap();
        let chapter = actual_book.iter().next().unwrap();

        match chapter {
            BookItem::Chapter(chapter) => {
                assert_eq!(chapter.content, "# Chapter 1\n #9\n\n0 minutes");
            }
            _ => panic!("Expected a chapter"),
        };
    }
}
