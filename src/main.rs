use bip32::{Prefix, Seed, XPrv};
use bip39::{Language, Mnemonic};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The mnemonic phrase of twelve BIP-39 words to use to generate the private key
    #[arg(short, long)]
    mnemonic: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    /*
        What this app does:

        Get a BIP-39 phrase (i.e. list of 12 words that generate entropy)
        E.g. "hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay length"

        Converts it into a private key that can be used alone or other BIP-32 heirarchal deterministic wallets
    */

    // if user typed in words, use them, otherwise generate random words
    let m = if let Some(all_words) = args.mnemonic {
        Mnemonic::parse(all_words)
    } else {
        let mut rng = bip39::rand_core::OsRng;
        bip39::Mnemonic::generate_in_with(&mut rng, Language::English, 12)
    };

    // Make sure they used legit words
    let mnemonic = if let Ok(r) = m {
        r
    } else {
        println!("Please check that your mnemonic phrase uses 12 words from BIP39 english word list ( https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt ) separated by single spaces and try again.");
        return Ok(());
    };

    // Turn that seed into private key
    let private_key = XPrv::new(&Seed::new(mnemonic.to_seed("")))?;

    // You can use this private key as is with bitcoin
    // Or you can use it with other BIP-32 sub-wallets

    println!(
        "{}\n{}",
        mnemonic.to_string(),
        private_key.to_string(Prefix::XPRV).to_string()
    );

    Ok(())
}
