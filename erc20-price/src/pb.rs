#[rustfmt::skip]
#[allow(unused_imports)]
#[path = "../target/pb/messari.chainlink.v1.rs"]
pub(in crate::pb) mod chainlink_v1;

pub mod chainlink {
    pub mod v1 {
        pub use super::super::chainlink_v1::*;
    }
}

#[rustfmt::skip]
#[allow(unused_imports)]
#[path = "../target/pb/messari.erc20.v1.rs"]
pub(in crate::pb) mod erc20_v1;

pub mod erc20 {
    pub mod v1 {
        pub use super::super::erc20_v1::*;
    }
}

#[rustfmt::skip]
#[allow(unused_imports)]
#[path = "../target/pb/messari.erc20_price.v1.rs"]
pub(in crate::pb) mod erc20_price_v1;

pub mod erc20_price {
    pub mod v1 {
        pub use super::super::erc20_price_v1::*;
    }
}
