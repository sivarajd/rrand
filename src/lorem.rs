use rand::Rng;

pub const LOREM_WORDS: &[&str] = &[
    "lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit",
    "sed", "do", "eiusmod", "tempor", "incididunt", "ut", "labore", "et", "dolore",
    "magna", "aliqua", "enim", "ad", "minim", "veniam", "quis", "nostrud",
    "exercitation", "ullamco", "laboris", "nisi", "aliquip", "ex", "ea", "commodo",
    "consequat", "duis", "aute", "irure", "in", "reprehenderit", "voluptate",
    "velit", "esse", "cillum", "fugiat", "nulla", "pariatur", "excepteur", "sint",
    "occaecat", "cupidatat", "non", "proident", "sunt", "culpa", "qui", "officia",
    "deserunt", "mollit", "anim", "id", "est", "laborum", "at", "vero", "eos",
    "accusamus", "iusto", "odio", "dignissimos", "ducimus", "blanditiis",
    "praesentium", "voluptatum", "deleniti", "atque", "corrupti", "quos", "dolores",
    "quas", "molestias", "excepturi", "obcaecati", "cupiditate", "provident",
    "similique", "mollitia", "fuga", "harum", "quidem", "rerum", "facilis",
    "expedita", "distinctio", "nam", "libero", "tempore", "cum", "soluta", "nobis",
    "eligendi", "optio", "cumque", "nihil", "impedit", "quo", "minus", "quod",
    "maxime", "placeat", "facere", "possimus", "omnis", "voluptas", "assumenda",
    "repellendus", "temporibus", "autem", "quibusdam", "officiis", "debitis",
    "saepe", "eveniet", "voluptates", "repudiandae", "recusandae", "itaque",
    "earum", "hic", "tenetur", "sapiente", "delectus", "reiciendis", "maiores",
    "alias", "consequatur", "perferendis", "doloribus", "asperiores", "repellat",
];

pub const CLASSIC_OPENING: &str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

/// Generates `count` space-separated random Latin words.
pub fn generate_words(count: usize) -> String {
    if count == 0 {
        return String::new();
    }
    let mut rng = rand::thread_rng();
    let words: Vec<&str> = (0..count)
        .map(|_| {
            let idx = rng.gen_range(0..LOREM_WORDS.len());
            LOREM_WORDS[idx]
        })
        .collect();
    words.join(" ")
}

/// Generates a single random Latin sentence.
fn generate_single_sentence() -> String {
    let mut rng = rand::thread_rng();
    let word_count = rng.gen_range(6..=14);
    let mut words: Vec<String> = (0..word_count)
        .map(|_| {
            let idx = rng.gen_range(0..LOREM_WORDS.len());
            LOREM_WORDS[idx].to_string()
        })
        .collect();

    // Capitalize the first word
    if let Some(first) = words.first_mut() {
        let mut chars = first.chars();
        if let Some(c) = chars.next() {
            *first = c.to_uppercase().collect::<String>() + chars.as_str();
        }
    }

    words.join(" ") + "."
}

/// Generates `count` random Latin sentences.
pub fn generate_sentences(count: usize, start_with_lorem: bool) -> String {
    if count == 0 {
        return String::new();
    }
    let mut sentences = Vec::with_capacity(count);
    if start_with_lorem {
        sentences.push(CLASSIC_OPENING.to_string());
    }

    let remaining = if start_with_lorem {
        count.saturating_sub(1)
    } else {
        count
    };

    for _ in 0..remaining {
        sentences.push(generate_single_sentence());
    }

    sentences.join(" ")
}

/// Generates `count` paragraphs of random Latin text separated by two newlines.
pub fn generate_paragraphs(count: usize, start_with_lorem: bool) -> String {
    if count == 0 {
        return String::new();
    }
    let mut rng = rand::thread_rng();
    let mut paragraphs = Vec::with_capacity(count);

    for i in 0..count {
        let sentence_count = rng.gen_range(3..=6);
        let start_lorem = i == 0 && start_with_lorem;
        paragraphs.push(generate_sentences(sentence_count, start_lorem));
    }

    paragraphs.join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_words() {
        let words = generate_words(10);
        let count = words.split_whitespace().count();
        assert_eq!(count, 10);
    }

    #[test]
    fn test_generate_sentences_with_classic() {
        let s = generate_sentences(2, true);
        assert!(s.starts_with("Lorem ipsum dolor sit amet"));
        let sentence_count = s.matches('.').count();
        assert_eq!(sentence_count, 2);
    }

    #[test]
    fn test_generate_paragraphs() {
        let p = generate_paragraphs(3, true);
        let paragraphs: Vec<&str> = p.split("\n\n").collect();
        assert_eq!(paragraphs.len(), 3);
        assert!(paragraphs[0].starts_with("Lorem ipsum"));
    }
}
