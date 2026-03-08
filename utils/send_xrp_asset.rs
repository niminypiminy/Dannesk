// src/utils/send_xrp_asset.rs
use dioxus_native::prelude::ReadableExt;  
use crate::context::{XrpContext, RlusdContext, EuroContext, SgdContext};

#[derive(Clone, PartialEq, Debug)]
pub enum SendAsset {
    XRP,
    RLUSD,
    EURO,
    SGD,
    // ── ADD NEW TOKENS HERE ──
}

impl SendAsset {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "RLUSD" => Self::RLUSD,
            "EUROP" => Self::EURO,
            "XSGD" => Self::SGD,

            _ => Self::XRP,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::XRP => "XRP",
            Self::RLUSD => "RLUSD",
            Self::EURO => "EUROP",
            Self::SGD => "XSGD",
        }
    }


    pub fn has_usd_equivalent(&self) -> bool {
        matches!(self, Self::XRP)
    }

    pub fn fiat_rate_key(&self) -> Option<&'static str> {
        match self {
            Self::XRP => Some("XRP/USD"),
            _ => None,
        }
    }

    pub fn balance(&self, xrp: &XrpContext, rlusd: &RlusdContext, euro: &EuroContext, sgd: &SgdContext) -> f64 {
        match self {
            Self::XRP => xrp.wallet_balance.read().0,
            Self::RLUSD => rlusd.rlusd.read().0,
            Self::EURO => euro.euro.read().0,
            Self::SGD => sgd.sgd.read().0,

        }
    }

   
}