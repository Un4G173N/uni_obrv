// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity ^0.8.20;

interface IUniswapV3Pool {
    function tickBitmap(int16 wordPos) external view returns (uint256);
    function ticks(int24 tick) external view returns (
        uint128 liquidityGross,
        int128 liquidityNet,
        uint256 feeGrowthOutside0X128,
        uint256 feeGrowthOutside1X128,
        int56 tickCumulativeOutside,
        uint160 secondsPerLiquidityOutsideX128,
        uint32 secondsOutside,
        bool initialized
    );
    function tickSpacing() external view returns (int24);
}

contract UniObrv {
    struct PopulatedTick {
        int24 tick;
        int128 liquidityNet;
        uint128 liquidityGross;
        uint256 feeGrowthOutside0X128;
        uint256 feeGrowthOutside1X128;
    }

    function getTickData(
        address pool,
        int24 tickLower,
        int24 tickUpper,
        uint256 maxTicks
    ) external view returns ( PopulatedTick[] memory populatedTicks) {
        require(pool != address(0), "Invalid pool address");
        require(tickLower <= tickUpper, "Invalid tick range");
        require(tickLower >= -887272 && tickUpper <= 887272, "Ticks out of range");
        


    uint256[] memory bitmaps;

        int24 tickSpacing = IUniswapV3Pool(pool).tickSpacing();
        tickLower = (tickLower / tickSpacing) * tickSpacing;
        tickUpper = (tickUpper / tickSpacing) * tickSpacing;

        int24 compressed = tickLower / tickSpacing;
        int16 wordPosLower = int16(compressed >> 8);
        compressed = tickUpper / tickSpacing;
        int16 wordPosUpper = int16(compressed >> 8);

        uint256 count;
        uint256 length = uint16(wordPosUpper - wordPosLower + 1);
        bitmaps = new uint256[](length);
        for (int16 wordPos = wordPosLower; wordPos <= wordPosUpper; wordPos++) {
            uint256 bitmap = IUniswapV3Pool(pool).tickBitmap(wordPos);
            bitmaps[uint16(wordPos - wordPosLower)] = bitmap;
            count += popCount(bitmap);
        }

        count = count > maxTicks ? maxTicks : count;
        populatedTicks = new PopulatedTick[](count);
        uint256 idx;

        for (uint256 i = 0; i < length && idx < count; i++) {
            uint256 bitmap = bitmaps[i];
            int16 wordPos = int16(wordPosLower + int16(uint16(i)));
            for (uint256 bitPos = 0; bitPos < 256 && idx < count; bitPos++) {
                if (bitmap & (1 << bitPos) != 0) {
                    int24 tick = tickSpacing * (int24(wordPos) * 256 + int24(uint24(bitPos)));
                    (uint128 liquidityGross, int128 liquidityNet, uint256 feeGrowthOutside0X128, uint256 feeGrowthOutside1X128,,,,) = IUniswapV3Pool(pool).ticks(tick);
                    populatedTicks[idx++] = PopulatedTick({
                        tick: tick,
                        liquidityNet: liquidityNet,
                        liquidityGross: liquidityGross,
                        feeGrowthOutside0X128: feeGrowthOutside0X128,
                        feeGrowthOutside1X128: feeGrowthOutside1X128
                    });
                }
            }
        }
    }

    function popCount(uint256 x) internal pure returns (uint256 count) {
        unchecked {
            while (x != 0) {
                x &= x - 1;
                count++;
            }
        }
    }
}
