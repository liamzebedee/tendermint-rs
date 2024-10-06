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
}
