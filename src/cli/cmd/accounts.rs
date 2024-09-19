use crate::utils::CmdSync;
use clap::Parser;
use serde_json::Result;
use tendermint::{config::AccountConfig, crypto::ECDSAKeypair};

pub struct AccountsOutput {}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct AccountsArgs {
    // --list
    #[clap(long, help = "List all accounts")]
    pub list: bool,
    // --new
    #[clap(long, help = "Create a new account")]
    pub new: bool,
}

impl CmdSync for AccountsArgs {
    type Output = Result<AccountsOutput>;

    fn run(self) -> Self::Output {
        if self.list {
            // TODO.
        } else if self.new {
            new_account()
        }
        Ok(AccountsOutput {})
    }
}

fn new_account() {
    let keypair = ECDSAKeypair::new();
    let datum = AccountConfig {
        pubkey: keypair.get_public_key().to_string(),
        privkey: keypair.get_secret_key().display_secret().to_string(),
    };
    println!("{}", serde_json::to_string_pretty(&datum).unwrap());

    // Print pubkey,privkey in hex.
    // println!(
    //     "Keypair pub={:?} prv={:?}",
    //     keypair.get_public_key().to_string(),
    //     keypair.get_secret_key().display_secret().to_string()
    // );

    // Verify generated keypair.
    // let keypair2 = ECDSAKeypair::new_from_privatekey(
    //     &keypair.get_secret_key().display_secret().to_string(),
    // );
    // println!(
    //     "Keypair pub={:?} prv={:?}",
    //     keypair2.get_public_key().to_string(),
    //     keypair2.get_secret_key().display_secret().to_string()
    // );
}
