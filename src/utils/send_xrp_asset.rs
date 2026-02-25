// src/utils/send_xrp_asset.rs
use dioxus_native::prelude::ReadableExt;  // ← THIS WAS THE MISSING PIECE
use crate::context::{XrpContext, RlusdContext, EuroContext};

#[derive(Clone, PartialEq, Debug)]
pub enum SendAsset {
    XRP,
    RLUSD,
    EURO,
    // ── ADD NEW TOKENS HERE ──
}

impl SendAsset {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "RLUSD" => Self::RLUSD,
            "EURO" => Self::EURO,
            _ => Self::XRP,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::XRP => "XRP",
            Self::RLUSD => "RLUSD",
            Self::EURO => "EURO",
        }
    }

    pub fn reserve_requirement(&self) -> f64 {
        match self {
            Self::XRP => 1.0,
            _ => 0.0,
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

    pub fn balance(&self, xrp: &XrpContext, rlusd: &RlusdContext, euro: &EuroContext) -> f64 {
        match self {
            Self::XRP => xrp.wallet_balance.read().0,
            Self::RLUSD => rlusd.rlusd.read().0,
            Self::EURO => euro.euro.read().0,
        }
    }

    pub fn insufficient_funds_error(&self) -> String {
        match self {
            Self::XRP => "ERR: INSUFFICIENT_FUNDS_RESERVE_REQUIRED".to_string(),
            other => format!("ERR: INSUFFICIENT_FUNDS_{}", other.label()),
        }
    }
}