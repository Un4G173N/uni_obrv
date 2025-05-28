# Uniswap Observer
This project provides a fast and efficient way to fetch tick data from Uniswap V3 pools using a single RPC call. It consists of two main components:

UniObrv Solidity Contract: A deployed Ethereum smart contract (0xb16Fe18dF6A79f8CE3049FfECEBD9FEaAf39f808) that queries tick data from Uniswap V3 pools.
Rust Program: A lightweight Rust library (uni_obrv) that interfaces with the UniObrv contract to retrieve tick data with minimal overhead.

## Features

Single RPC Call: Retrieves Uniswap V3 pool tick data efficiently in one Ethereum RPC call.
High Performance: Optimized for speed, leveraging the UniObrv contract to batch tick data queries.
Extensible: The contract and library can be extended to fetch additional Uniswap V3 state data, such as positions, fees, or other pool metrics.


Getting Started

Installation

Clone the repository:git clone https://github.com/<your-repo>/uniswap-v3-tick-fetcher.git
cd uniswap-v3-tick-fetcher


Install Rust dependencies:cargo build


Set up your .env file with your Ethereum RPC URL:echo "ETH_RPC_URL=<your-rpc-url>" > .env



Usage

Run Tests: Execute the Rust tests to verify tick data fetching:
cargo test


Example Code: Fetch tick data for a Uniswap V3 pool (e.g., USDC/WETH):

```
use uni_obrv::{get_tick_data, PopulatedTick, UNI_OBRV};
use alloy::primitives::{address, I24, U256};
use alloy::providers::ProviderBuilder;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = ProviderBuilder::new().on_http("your-rpc-url".parse()?);
    let pool = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");
    let tick_lower = I24::from_str("188258")?;
    let tick_upper = I24::from_str("276278")?;
    let max_ticks = U256::from(1000000);

    let ticks = get_tick_data(pool, tick_lower, tick_upper, max_ticks, provider, UNI_OBRV).await?;
    for tick in ticks {
        println!("Tick: {}, Liquidity Gross: {}", tick.tick, tick.liquidity_gross);
    }
    Ok(())
}

```


Solidity Testing: Use Foundry to test the UniObrv contract:
```
forge test
````


## Extending the Project
The UniObrv contract and Rust library can be extended to support additional Uniswap V3 data, such as:

Positions: Query user positions and their associated liquidity.
Fees: Fetch accumulated fees for specific ticks or positions.
Pool State: Retrieve global pool state like current price or liquidity.


## Contributing
Contributions are welcome! To contribute:

Fork the repository.
Create a new branch (git checkout -b feature/your-feature).
Make your changes and commit (git commit -m "Add your feature").
Push to your branch (git push origin feature/your-feature).
Open a pull request with a clear description of your changes.

Please ensure your code follows the existing style and includes tests for new functionality.
