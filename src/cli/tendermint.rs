pub mod cmd;
mod utils;

use crate::utils::{CmdAsync, CmdSync};
use clap::{Parser, Subcommand};
use cmd::node::NodeArgs;
use cmd::accounts::AccountsArgs;

#[derive(Debug, Parser)]
#[clap(name = "tendermint")]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Debug, Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    Node(NodeArgs),
    Accounts(AccountsArgs),
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();
    match opts.sub {
        Subcommands::Node(cmd) => {
            cmd.run().await.unwrap();
        },
        Subcommands::Accounts(cmd) => {
            cmd.run().unwrap();
        }
    }

    // TODO: consider returning Result<T,E> for error codes.
    // Ok(())
}