# Text_analyzer
# 📝 TextAnalyzer

A small but educational Rust program that slices a text into selected word ranges and analyzes which word is the longest.

This project demonstrates key Rust concepts like:
- Structs and `impl` blocks
- String slicing and lifetimes
- Ownership and borrowing
- Basic text processing

---

## 📌 Example

Given this input:

```rust
let text = "All my life I wanted money and power";
let word_slice = ta.get_word_in_range(1, 4);  // ["my", "life", "I"]
