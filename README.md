<img align="left" style="vertical-align: middle" width="120" height="120" src=https://user-images.githubusercontent.com/4886639/183321432-b17aa134-9f7a-41ee-91d2-778b6062813f.png>

# Konpeito

Konpeito is a personal key-value store CLI that can be used to store **anything** securely to/from the system's keyring.

[![Crates.io](https://img.shields.io/crates/v/konpeito)](https://crates.io/crates/konpeito)
![License](https://img.shields.io/crates/l/konpeito)

## How it works

Konpeito uses the Secret Service API (provided by the `keyring` crate) to store items. This means that the items are stored in the system's keyring, which is a secure(ish) way to store items. The keyring is encrypted and can only be accessed by the user that created it.

This mean that Konpeito is not an actual database, but a wrapper around the keyring API to store tokens, passwords, or any other kind of data and easily retrieve it
for CLI usage or scripting. It also means that Konpeito relies on an existing keyring implementation, which means
security is not guaranteed as the encryption is done by the keyring implementation, which varies depending on the OS, as follows:

| OS      | Keyring implementation                    |
| ------- | ----------------------------------------- |
| Linux   | Linux keyutils, GNOME Keyring, or KWallet |
| macOS   | macOS Keychain                            |
| Windows | Windows Credential Manager                |

Konpeito reads and writes values using Base64 encoding, this means that you can store any kinds of data, not limited to just text.

## Usage

To store items:

```bash
konp set key value
```

If you want to store a file or something from stdin:

```bash
konp set key < file
```

To retrieve items:

```bash
konp get key
```

To delete keys:

```bash
konp delete key
```

## Installation

Simply install this crate

From source:

```bash
cargo install --path .
```

From crates.io:

```bash
cargo install konpeito
```
