
use ethers::prelude::Middleware;
// use ethers::providers::{Http, Provider};
// use ethers::types::{NameOrAddress};
// use std::{convert::TryFrom};
// use dotenv::dotenv;
// use std::env;

// use anyhow::Error;
// use tokio;

// #[tokio::main]
// async fn main () -> Result<(), Error> {

//     dotenv().ok();
//     let infura_uri = env::var("INFURA_ENDPOINT")?;
//     //  create json-rpc over http
//     let provider = Provider::<Http>::try_from(infura_uri)?;
    
//     // just querying the random address
    // let address: NameOrAddress = "0x73BCEb1Cd57C711feaC4224D062b0F6ff338501e".into();
    
//     let address  = "0x0AcA4bF1e72935347290A4b2D4f52254ebDA6f11";
    
//     // // getting the balance for address
//     let bal = provider.get_balance(address, None).await?;

//     println!("Balance for {:?} is {:?}", address, bal);
//     // let block = provider.get_block(14829404).await?;
//     // println!("block {:?}", block);
//     Ok(())
// }

use web3;
use dotenv::dotenv;
use std::env;
use anyhow::Error;

#[tokio::main]
async fn main () -> Result<(), Error> {

    dotenv().ok();
    let infura_uri = env::var("INFURA_ENDPOINT")?;
    
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    
    let web3 = web3::Web3::new(transport);


    // let balance = web3.eth().balance("0x0AcA4bF1e72935347290A4b2D4f52254ebDA6f11", None);
    //  create json-rpc over http
    // let provider = Provider::<Http>::try_from(infura_uri)?;
    
    // // just querying the random address
    // let address: NameOrAddress = "0x73BCEb1Cd57C711feaC4224D062b0F6ff338501e".into();
    
    // let address  = "0x0AcA4bF1e72935347290A4b2D4f52254ebDA6f11";
    
    // // // getting the balance for address
    // let bal = provider.get_balance(address, None).await?;

    // println!("Balance for {:?} is {:?}", address, bal);
    // let block = provider.get_block(14829404).await?;
    // println!("block {:?}", block);
    
    
    Ok(())
}