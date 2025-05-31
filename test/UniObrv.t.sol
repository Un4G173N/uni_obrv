// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "contracts/UniObrv.sol";

contract UniObrvTest is Test {
    UniObrv uniObrv;
    UniObrv uniObrv_new;
    address constant POOL_ADDRESS = 0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640; // DAI/USDC 0.05% pool
    address constant UNI_OBRV = 0xb16Fe18dF6A79f8CE3049FfECEBD9FEaAf39f808;


    struct PopulatedTick {
        int24 tick;
        int128 liquidityNet;
        uint128 liquidityGross;
        uint256 feeGrowthOutside0X128;
        uint256 feeGrowthOutside1X128;
    }



    function setUp() public {
        uint256 mainnetFork = vm.createFork("http://127.0.0.1:8545"); // Use a recent block number
        vm.selectFork(mainnetFork);
        // Verify pool exists
        (bool success, ) = POOL_ADDRESS.call(abi.encodeWithSignature("tickSpacing()"));
        require(success, "Pool does not exist or is not a valid Uniswap V3 pool");       
        uniObrv = UniObrv(UNI_OBRV);
        uniObrv_new = new UniObrv();
    }

    function test_GetTickData() public view{


        int24 tick_lower = 188258; 
        int24 tick_upper = 188558;
        uint256 max_ticks = 1000000;

        (UniObrv.PopulatedTick[] memory ticks, int24 tickSpacing) = uniObrv.getTickData(POOL_ADDRESS, tick_lower, tick_upper, max_ticks);
        UniObrv.PopulatedTick memory tick = ticks[0];
        console.log(tick.feeGrowthOutside0X128);
        console.log("Space: ", tickSpacing);
        assert(tick.liquidityGross != 0);
    }

    function test_GetTickData_new_deployed() public view{


        int24 tick_lower = 188258; 
        int24 tick_upper = 188558;
        uint256 max_ticks = 1000000;

        (UniObrv.PopulatedTick[] memory ticks, int24 tickSpacing) = uniObrv_new.getTickData(POOL_ADDRESS, tick_lower, tick_upper, max_ticks);
        UniObrv.PopulatedTick memory tick = ticks[0];
        console.log(tick.feeGrowthOutside0X128);
        console.log("Space: ", tickSpacing);
        assert(tick.liquidityGross != 0);
    }

    

}



