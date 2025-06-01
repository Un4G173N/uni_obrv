#![doc = "Module for interacting with the UniswapObrv Solidity contract to query Uniswap V3 pool tick data."]

use alloy::{
    network::Network, primitives::{address, aliases::I24, Address, U256 }, providers::Provider, sol
};

use alloc::vec::Vec;
use alloc::string::ToString;
use crate::error::Error;

// Define the PopulatedTick struct (matching LensOn.sol)
#[derive(Debug, Clone, Copy)]
pub struct PopulatedTick {
    pub tick: I24,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_0x128: U256,
    pub fee_growth_outside_1x128: U256,
}

// Define the LensOn contract interface
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniObrvContract,
    "UniObrv.sol/UniObrv.json"
);

/// Constant address of the deployed UniswapObrv contract.
pub const UNI_OBRV: Address = address!("4d2c0A1d2d3725618c6799b3DE9FDdEf1d4118B5");

/// Queries tick data from a Uniswap V3 pool via the LensOn contract.
///
/// # Arguments
/// * `pool` - The address of the Uniswap V3 pool.
/// * `tick_lower` - The lower tick bound of the range.
/// * `tick_upper` - The upper tick bound of the range.
/// * `max_ticks` - The maximum number of ticks to return.
/// * `provider` - The alloy provider for Ethereum RPC communication.
/// * `uni_obrv_address` - The lens_on_contract address "Optional".
/// 
/// # Returns
/// A `Result` containing a vector of `PopulatedTick` structs or an error if the query fails.
// #[cfg(feature = "std")]
pub async fn get_tick_data<P, N>(
    pool: Address,
    tick_lower: I24,
    tick_upper: I24,
    max_ticks: U256,
    provider: P,
    uni_obrv_address: Address,
) -> Result<(Vec<PopulatedTick>, I24), Error> 
where 
N: Network,
P: Provider<N>
{
    // Input validation
    if pool == Address::ZERO {
           return Err(Error::InvalidPoolAddress);
    }

    if tick_lower > tick_upper {
        return Err(Error::InvalidTickRange);
    }
    let tick_lower_min = I24::MIN;
    let tick_upper_max = I24::MAX;

    if tick_lower < tick_lower_min || tick_upper > tick_upper_max {
        return Err(Error::TicksOutOfRange);
    }

    // Create contract instance
    let contract = UniObrvContract::new(
        if uni_obrv_address.is_zero() { UNI_OBRV } else { uni_obrv_address },
        provider,
    );

    // Call the contract
    let result = contract
        .getTickData(pool, tick_lower, tick_upper, max_ticks)
        .call()
        .await
        .map_err(|e| Error::ContractCall(e.to_string()))?;

    // Map results to PopulatedTick
    Ok((
        result.populatedTicks
            .into_iter()
            .map(|tick| PopulatedTick {
                tick: tick.tick,
                liquidity_net: tick.liquidityNet,
                liquidity_gross: tick.liquidityGross,
                fee_growth_outside_0x128: tick.feeGrowthOutside0X128,
                fee_growth_outside_1x128: tick.feeGrowthOutside1X128,
            })
            .collect(),
        result.tickSpacing,
    ))
}



#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use super::*;
    use tokio;
    use crate::tests::*;

    const POOL: Address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"); // Example Uniswap V3 pool (USDC/WETH)


    #[tokio::test]
    async fn test_get_tick_data_valid() -> Result<(), Error> {
        let provider = PROVIDER.clone();
        let tick_lower = I24::from_str("188258").unwrap();
        let tick_upper = I24::from_str("276278").unwrap();
        let max_ticks = U256::from(1000000);

        let (ticks, tick_spacing) = get_tick_data(
            POOL,
            tick_lower,
            tick_upper,
            max_ticks,
            provider,
            Address::ZERO,
        )
        .await?;

        // Assert that ticks are not empty
        if ticks.is_empty() {
            return Err(Error::ContractCall("Empty tick data".to_string()));
        }

        // Add assertions to verify data
        assert!(!ticks.is_empty(), "Tick data should not be empty");
        let tick = &ticks[0];
            assert!(tick.liquidity_gross > 0, "Liquidity gross should be positive");
            println!("Tick liquidity gross: {}", tick.liquidity_gross);
        
        assert!(tick_spacing >= I24::ZERO, "Tick spacing should be non-negative");
        println!("Tick spacing: {}", tick_spacing);

        Ok(())
    }


    #[tokio::test]
    async fn test_get_tick_data_invalid_pool() {
        let provider = PROVIDER.clone();
        let tick_lower = I24::from_str("188258").unwrap();
        let tick_upper = I24::from_str("276278").unwrap();
        let max_ticks = U256::from(1000000);

        let result = get_tick_data(Address::ZERO, tick_lower, tick_upper, max_ticks, provider, Address::ZERO).await;
        assert!(result.is_err(), "Expected error for zero pool address");
        if let Err(e) = result {
            assert_eq!(e.to_string(), "Invalid pool address");
        }
    }

}