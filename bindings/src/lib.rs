extern crate core;

#[swift_bridge::bridge]
mod ffi {    
    extern "Rust" {
        #[swift_bridge(swift_name = "calculateOutGivenIn")]
        fn calculate_out_given_in(
            reserves: String,
            asset_in: u32,
            asset_out: u32,
            amount_in: String,
            amplification: String,
            fee: String,
        ) -> String;

        #[swift_bridge(swift_name = "calculateInGivenOut")]
        fn calculate_in_given_out(
            reserves: String,
            asset_in: u32,
            asset_out: u32,
            amount_out: String,
            amplification: String,
            fee: String,
        ) -> String;

        #[swift_bridge(swift_name = "calculateAmplification")]
        fn calculate_amplification(
            initial_amplification: String,
            final_amplification: String,
            initial_block: String,
            final_block: String,
            current_block: String,
        ) -> String;

        #[swift_bridge(swift_name = "calculateShares")]
        fn calculate_shares(
            reserves: String,
            assets: String,
            amplification: String,
            share_issuance: String,
            fee: String,
        ) -> String;

        #[swift_bridge(swift_name = "calculateSharesForAmount")]
        fn calculate_shares_for_amount(
            reserves: String,
            asset_in: u32,
            amount: String,
            amplification: String,
            share_issuance: String,
            fee: String,
        ) -> String;

        #[swift_bridge(swift_name = "calculateAddOneAsset")]
        fn calculate_add_one_asset(
            reserves: String,
            shares: String,
            asset_in: u32,
            amplification: String,
            share_issuance: String,
            fee: String,
        ) -> String;
    }
}

macro_rules! to_u128 {
    ($($x:expr),+) => (
        {($($x.parse::<u128>().unwrap_or(0)),+)}
    );
}

fn error() -> String {
    "-1".to_string()
}

use hydra_dx_math::stableswap::types::AssetReserve;
use std::collections::HashMap;

use serde::Deserialize;
use sp_arithmetic::Permill;
#[cfg(test)]
use sp_core::crypto::UncheckedFrom;
#[cfg(test)]
use sp_core::Hasher;
#[cfg(test)]
use sp_runtime::traits::IdentifyAccount;

use serde_aux::prelude::*;

macro_rules! parse_into {
    ($x:ty, $y:expr) => {{
        let r = if let Some(x) = $y.parse::<$x>().ok() {
            x
        } else {
            return error();
        };
        r
    }};
}
const D_ITERATIONS: u8 = 128;
const Y_ITERATIONS: u8 = 64;

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct AssetBalance {
    asset_id: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    amount: u128,
    decimals: u8,
}

impl From<&AssetBalance> for AssetReserve {
    fn from(value: &AssetBalance) -> Self {
        Self {
            amount: value.amount,
            decimals: value.decimals,
        }
    }
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct AssetAmount {
    asset_id: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    amount: u128,
}

#[no_mangle]
pub fn calculate_out_given_in(
    reserves: String,
    asset_in: u32,
    asset_out: u32,
    amount_in: String,
    amplification: String,
    fee: String,
) -> String {
    let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
    if reserves.is_err() {
        return error();
    }
    let mut reserves = reserves.unwrap();
    reserves.sort_by_key(|v| v.asset_id);

    let idx_in = reserves.iter().position(|v| v.asset_id == asset_in);
    let idx_out = reserves.iter().position(|v| v.asset_id == asset_out);

    if idx_in.is_none() || idx_out.is_none() {
        return error();
    }

    let amount_in = parse_into!(u128, amount_in);
    let amplification = parse_into!(u128, amplification);
    let fee = Permill::from_float(parse_into!(f64, fee));

    let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();

    let result = hydra_dx_math::stableswap::calculate_out_given_in_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
        &balances,
        idx_in.unwrap(),
        idx_out.unwrap(),
        amount_in,
        amplification,
        fee,
    );

    if let Some(r) = result {
        r.0.to_string()
    } else {
        error()
    }
}

#[no_mangle]
pub fn calculate_in_given_out(
    reserves: String,
    asset_in: u32,
    asset_out: u32,
    amount_out: String,
    amplification: String,
    fee: String,
) -> String {
    let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
    if reserves.is_err() {
        return error();
    }
    let mut reserves = reserves.unwrap();
    reserves.sort_by_key(|v| v.asset_id);

    let idx_in = reserves.iter().position(|v| v.asset_id == asset_in);
    let idx_out = reserves.iter().position(|v| v.asset_id == asset_out);

    if idx_in.is_none() || idx_out.is_none() {
        return error();
    }

    let amount_out = parse_into!(u128, amount_out);
    let amplification = parse_into!(u128, amplification);
    let fee = Permill::from_float(parse_into!(f64, fee));

    let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();

    let result = hydra_dx_math::stableswap::calculate_in_given_out_with_fee::<D_ITERATIONS, Y_ITERATIONS>(
        &balances,
        idx_in.unwrap(),
        idx_out.unwrap(),
        amount_out,
        amplification,
        fee,
    );

    if let Some(r) = result {
        r.0.to_string()
    } else {
        error()
    }
}

#[no_mangle]
pub fn calculate_amplification(
    initial_amplification: String,
    final_amplification: String,
    initial_block: String,
    final_block: String,
    current_block: String,
) -> String {
    let initial_amplification = parse_into!(u128, initial_amplification);
    let final_amplification = parse_into!(u128, final_amplification);
    let initial_block = parse_into!(u128, initial_block);
    let final_block = parse_into!(u128, final_block);
    let current_block = parse_into!(u128, current_block);

    hydra_dx_math::stableswap::calculate_amplification(
        initial_amplification,
        final_amplification,
        initial_block,
        final_block,
        current_block,
    )
    .to_string()
}

#[no_mangle]
pub fn calculate_shares(
    reserves: String,
    assets: String,
    amplification: String,
    share_issuance: String,
    fee: String,
) -> String {
    let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
    if reserves.is_err() {
        return error();
    }
    let mut reserves = reserves.unwrap();
    reserves.sort_by_key(|v| v.asset_id);

    let assets: serde_json::Result<Vec<AssetAmount>> = serde_json::from_str(&assets);
    if assets.is_err() {
        return error();
    }
    let assets = assets.unwrap();
    if assets.len() > reserves.len() {
        return error();
    }

    let mut updated_reserves = reserves.clone();

    let mut liquidity: HashMap<u32, u128> = HashMap::new();
    for a in assets.iter() {
        let r = liquidity.insert(a.asset_id, a.amount);
        if r.is_some() {
            return error();
        }
    }
    for reserve in updated_reserves.iter_mut() {
        if let Some(v) = liquidity.get(&reserve.asset_id) {
            reserve.amount += v;
        }
    }
    let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();
    let updated_balances: Vec<AssetReserve> = updated_reserves.iter().map(|v| v.into()).collect();
    let amplification = parse_into!(u128, amplification);
    let issuance = parse_into!(u128, share_issuance);
    let fee = Permill::from_float(parse_into!(f64, fee));

    let result = hydra_dx_math::stableswap::calculate_shares::<D_ITERATIONS>(
        &balances,
        &updated_balances,
        amplification,
        issuance,
        fee,
    );

    if let Some(r) = result {
        r.to_string()
    } else {
        error()
    }
}

#[no_mangle]
pub fn calculate_shares_for_amount(
    reserves: String,
    asset_in: u32,
    amount: String,
    amplification: String,
    share_issuance: String,
    fee: String,
) -> String {
    let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
    if reserves.is_err() {
        return error();
    }
    let mut reserves = reserves.unwrap();
    reserves.sort_by_key(|v| v.asset_id);
    let idx_in = reserves.iter().position(|v| v.asset_id == asset_in);
    if idx_in.is_none() {
        return error();
    }
    let amount_in = parse_into!(u128, amount);
    let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();
    let amplification = parse_into!(u128, amplification);
    let issuance = parse_into!(u128, share_issuance);
    let fee = Permill::from_float(parse_into!(f64, fee));

    let result = hydra_dx_math::stableswap::calculate_shares_for_amount::<D_ITERATIONS>(
        &balances,
        idx_in.unwrap(),
        amount_in,
        amplification,
        issuance,
        fee,
    );

    if let Some(r) = result {
        r.to_string()
    } else {
        error()
    }
}

#[no_mangle]
pub fn calculate_add_one_asset(
    reserves: String,
    shares: String,
    asset_in: u32,
    amplification: String,
    share_issuance: String,
    fee: String,
) -> String {
    let reserves: serde_json::Result<Vec<AssetBalance>> = serde_json::from_str(&reserves);
    if reserves.is_err() {
        return error();
    }
    let mut reserves = reserves.unwrap();
    reserves.sort_by_key(|v| v.asset_id);
    let idx_in = reserves.iter().position(|v| v.asset_id == asset_in);
    if idx_in.is_none() {
        return error();
    }

    let balances: Vec<AssetReserve> = reserves.iter().map(|v| v.into()).collect();
    let shares = parse_into!(u128, shares);
    let amplification = parse_into!(u128, amplification);
    let issuance = parse_into!(u128, share_issuance);
    let fee = Permill::from_float(parse_into!(f64, fee));

    let result = hydra_dx_math::stableswap::calculate_add_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
        &balances,
        shares,
        idx_in.unwrap(),
        issuance,
        amplification,
        fee,
    );

    if let Some(r) = result {
        r.0.to_string()
    } else {
        error()
    }
}

#[no_mangle]
pub fn pool_account_name(share_asset_id: u32) -> Vec<u8> {
    let mut name = "sts".as_bytes().to_vec();
    name.extend_from_slice(&(share_asset_id).to_le_bytes());
    return name;
}