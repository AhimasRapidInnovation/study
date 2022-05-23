
use ethers::prelude::Middleware;
use ethers::providers::{Http, Provider};
use ethers::types::{NameOrAddress};
use std::{convert::TryFrom};
use dotenv::dotenv;
use std::env;

use anyhow::Error;
use tokio;

#[tokio::main]
async fn main () -> Result<(), Error> {

    dotenv().ok();
    let infura_uri = env::var("INFURA_ENDPOINT")?;
    //  create json-rpc over http
    let provider = Provider::<Http>::try_from(infura_uri)?;
    
    // just querying the random address
    let address: NameOrAddress = "0x73BCEb1Cd57C711feaC4224D062b0F6ff338501e".into();
    
    let address: NameOrAddress = "0x0AcA4bF1e72935347290A4b2D4f52254ebDA6f11".into();
    
    // // getting the balance for address
    let bal = provider.get_balance(address.clone(), None).await?;

    println!("Balance for {:?} is {:?}", address, bal);
    // let block = provider.get_block(14829404).await?;
    // println!("block {:?}", block);
    Ok(())
}