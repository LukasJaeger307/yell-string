# yell-string
Ever felt the need to sOuNd LiKe A yElLiNg IdIoT on social media but found it too annoying to type this way? Struggle no more, _yell-string_ is here to help you out. This command line tool takes arbitrary strings and applies a ridiculous caps pattern on it.

## Install
_yell-string_ is written in Rust so make sure you have cargo installed. Clone the repo, change directories into it and execute
```bash
cargo install --path .
```
Now you are good to go.

## Usage
Execute _yell-string_ like this:
```bash
yell-string "[Some dumb sentence]"
```
where _Some dumb sentence_ is the sentence you want to post on social media. The tool will return it with a ridiculous caps pattern. You can also pass multiple sentences to _yell-string_:
```bash
yell-string "[Some dumb sentence]" "[Another dumb sentence]" "[Yet another dumb sentence]"
```
