# Ragnar: A CLI for Generating Bitcoin Private Keys from 12-Word Mnemonic Phrases

## Introduction

Ragnar is a command-line interface (CLI) tool written in Rust, designed to generate secure Bitcoin private keys from 12-word mnemonic phrases. This codebase was created to easily show how this is done in the most minimal way possible.

<img width="1069" alt="Screenshot 2023-09-18 at 8 31 07 AM" src="https://github.com/richardanaya/ragnar/assets/294042/980b663f-eb7b-4ea9-9f7c-a24b971e4ab1">


## Why is a private key important in Bitcoin?

Private/public keys are the foundation of Bitcoin technology. Your public key basically represents your wallet address, and your private key is used to make transactions on it by creating digital signatures for your transactions. So having an easy way to summon forth your private key with simple 12 words can be pretty powerful.

[![IMAGE ALT TEXT](http://img.youtube.com/vi/s22eJ1eVLTU/0.jpg)](http://www.youtube.com/watch?v=s22eJ1eVLTU "Signing with public/private keys")

## BIP-39: A High-Level Overview

[BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) (Bitcoin Improvement Proposal 39) is a protocol that describes the implementation of a mnemonic sentence—a group of easily readable words—to serve as a back-up or seed for deterministic wallets. These mnemonics are built up from a special list of 2048 words, so you can't just use any words and it's important they are randomly chosen ( not just hand picked by your favorite words ). These mnemonic sentences are mathematically related to your wallet's private key, providing a human-readable format that is easier to write down or memorize than a raw binary or hexadecimal string.

## Why 12 Words?

A 12-word mnemonic of BIP-39 generates 128 bits of entropy. If you have 2048 possible words to choose from, you have 2048^12 combinations, or in another term 2^132 (138-bits). The security level provided by 128 bits of entropy is considered to be strong and adequate for most cryptographic purposes. To put the security level into perspective: using the most advanced supercomputers, it would take longer than the age of the universe to brute-force a 128-bit entropy.

## Installation

```
cargo install ragnar
```

## Usage

To generate a Bitcoin private key, execute the Ragnar CLI and pass your 12-word mnemonic phrase as an argument:

```bash
ragnar -m "word1 word2 word3 ... word12"
```

The CLI will then output the corresponding Bitcoin private key (starts with letters "xprv"), calculated based on the BIP-39 specification.

```bash
ragnar
```

Will simply generate random 12 words and a private key. If you ever need to generate the private key, you'll have to remember the 12 words and put them in using the above method.


```bash
ragnar --help
```

Can give you additional info.

## Using the Private Key in a Wallet

### Electrum

1. When creating a wallet select "standard wallet"
2. Select "Use a master key"
3. Copy and paste the private key into the next box and hit next.
4. You can now receive and send bitcoin.

### Other

Many wallets support just the words themselves.

## Can this be used for other things than Bitcoin?

Yes, many crypto currencies use similar keys, but it's beyond the scope of this project.

## Contributing

Feel free to contribute to Ragnar by submitting pull requests or reporting bugs and suggestions as GitHub issues.

## License

Ragnar is released under the MIT License. Check out the [LICENSE](LICENSE) file for more information.
