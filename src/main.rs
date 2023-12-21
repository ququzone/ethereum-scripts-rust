use clap::Parser;
use ethereum_scripts_rust::command::{Cli, Commands};
use ethers::{
    abi::Address,
    prelude::Provider,
    providers::{Http, Middleware},
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Balance { address } => {
            let provider = Provider::<Http>::try_from(cli.rpc).expect("parse rpc fail");
            let address = address.parse::<Address>().expect("parse address fail");
            let balance = provider
                .get_balance(address, None)
                .await
                .expect("fetch account balance fail");

            println!("Account {} balance {}", address, balance);
        }
    }
}
