struct TextAnalyzer {
    text: String,
}

impl TextAnalyzer {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }

    fn analyze_word_length(&self, word_slice: &Vec<&str>) -> (usize, String) {
        let mut max_length = 0;
        let mut longest_word = "";

        for &word in word_slice {
            if word.len() > max_length {
                max_length = word.len();
                longest_word = word;
            }
        }

        (max_length, longest_word.to_string())
    }

    fn get_word_in_range(&self, start: usize, end: usize) -> Vec<&str> {
        let words = self.text.split_whitespace().collect::<Vec<&str>>();
        words[start..end].to_vec()
    }
}

fn main() {
    let text = "All my life I wanted money and power";
    let ta = TextAnalyzer::new(text);

    let word_slice = ta.get_word_in_range(1, 4);
    println!("Selected words: {:?}", word_slice);

    let (len, word) = ta.analyze_word_length(&word_slice);
    println!("Longest word: \"{}\" with length {}", word, len);
}
