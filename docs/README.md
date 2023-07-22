# llm-rs 🚀

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/kaikiat/llm-rs/)
[![Issues](https://img.shields.io/github/issues/kaikiat/llm-rs.svg?style=flat-square&maxAge=600)](https://github.com/kaikiat/llm-rs/issues)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/kaikiat/llm-rs/blob/master/CHANGELOG.md)


Why limit yourself to using ChatGPT solely through a browser? With `llm-rs`, you can leverage the power of ChatGPT right from your command line! `llm-rs` is a cutting-edge tool, meticulously crafted using the Rust programming language. 

Special thanks to @sobelio for making this possible

# Examples
Run `llm --help` to see available commands

Setting up llm configuration
```
$ llm config set

Please enter OpenAI API key [OPENAI_API_KEY]: XXXXXX
```

View llm configuration
```
$ llm config view

[OPENAI_API_KEY]: XXXXXX
```

Ask a question 
```
$ llm "Tell me more about Rust ?"
```

Summarise an article, can be json or text file
```
$ llm --file /path/to/article.txt
```

export url=https://github.com/golang-migrate/migrate/releases/download/$version/migrate.$os-$arch.tar.gz