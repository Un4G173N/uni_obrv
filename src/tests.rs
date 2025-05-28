use alloy::{
    providers::{ProviderBuilder, RootProvider},
    transports::http::reqwest::Url,
};
use dotenv::dotenv;
use once_cell::sync::Lazy;

pub(crate) static RPC_URL: Lazy<Url> = Lazy::new(|| {
    dotenv().ok();
    std::env::var("ETH_RPC_URL").unwrap().parse().unwrap()
});
pub(crate) static PROVIDER: Lazy<RootProvider> = Lazy::new(|| {
    ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_http(RPC_URL.clone())
});
