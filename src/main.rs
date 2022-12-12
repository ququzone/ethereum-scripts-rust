use ethers::{prelude::*};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from("https://babel-api.mainnet.iotex.io")?;

    let address = "0x173553c179bbf5af39D8Db41F0B60e4Fc631066a".parse::<Address>()?;

    let balance_before = provider.get_balance(address, None).await?;

    println!("balance = {}", balance_before);

    Ok(())
}
