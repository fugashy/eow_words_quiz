# eow_words_quiz
Ask you that what the word means from EOW word list

# Environments

- MacOS Catalina 10.15.5 (19F101)

# Requirments

- translate-shell

  ```bash
  $ brew install translate-shell
  ```

# How to use

- Download your word list from EOW

  https://eow.alc.co.jp

  Save the list as /tmp/words.tsv, for example.

```bash
cargo run /tmp/words.tsv
```
