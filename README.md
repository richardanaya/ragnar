# Ragnar: A CLI for Generating Bitcoin Private Keys from 12-Word Mnemonic Phrases

## Introduction

Ragnar is a command-line interface (CLI) tool written in Rust, designed to generate secure Bitcoin private keys from 12-word mnemonic phrases.

## BIP-39: A High-Level Overview

[BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) (Bitcoin Improvement Proposal 39) is a protocol that describes the implementation of a mnemonic code or mnemonic sentence—a group of easily readable words—to serve as a back-up or seed for deterministic wallets. These mnemonic sentences are mathematically related to your wallet's private key, providing a human-readable format that is easier to write down or memorize than a raw binary or hexadecimal string.

## Why 12 Words?

A 12-word mnemonic of BIP-39 generates 128 bits of entropy. The security level provided by 128 bits of entropy is considered to be strong and adequate for most cryptographic purposes. To put the security level into perspective: using the most advanced supercomputers, it would take longer than the age of the universe to brute-force a 128-bit entropy.

## Installation

```
cargo install ragnar
```

## Usage

To generate a Bitcoin private key, execute the Ragnar CLI and pass your 12-word mnemonic phrase as an argument:

```bash
ragnar -m "word1 word2 word3 ... word12"
```

The CLI will then output the corresponding Bitcoin private key, calculated based on the BIP-39 specification.

```bash
ragnar
```

Will simply generate random 12 words and a private key. If you ever need to generate the private key, you'll have to remember the 12 words and put them in using the above method.


```bash
ragnar --help
```

Can give you additional info.

## Contributing

Feel free to contribute to Ragnar by submitting pull requests or reporting bugs and suggestions as GitHub issues.

## License

Ragnar is released under the MIT License. Check out the [LICENSE](LICENSE) file for more information.
