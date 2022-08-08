# Konpeito

Konpeito is a personal key-value store database that can be used to store anything.

It is written in Rust and is powered by `sled`. And is created as an offline replacement for [Skate](https://github.com/charmbracelet/skate).


It works by storing data inside a sled database, encrypted using a key (using the private SSH key by default).

## Usage
To store items:

```bash
konpeito set key value
```

If you want to store a file or something from stdin:

```bash
konpeito set key < file
```


To retrieve items:

```bash
konpeito get key
```

To delete keys:

```bash
konpeito delete key
```


To list all keys:

```bash
konpeito list
```

## Installation

Simply install this crate

```bash
cargo install .
```