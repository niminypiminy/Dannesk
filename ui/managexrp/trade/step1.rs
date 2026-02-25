use dioxus_native::prelude::*;
use crate::context::{XrpContext, GlobalContext};
use crate::utils::market_order_form::MarketOrderForm;

#[component]
pub fn view() -> Element {
    let mut xrp_ctx = use_context::<XrpContext>();
    let global_ctx = use_context::<GlobalContext>();
    
    let search_query = use_signal(String::new);
    let is_searching = use_signal(|| false); 

    let rates_signal = global_ctx.rates;
    let trade_read = xrp_ctx.trade.read();
    
    let Some(inner) = trade_read.send_trade.as_ref() else {
        return rsx! {};
    };

    let selected_base = use_signal(|| inner.base_asset.clone().unwrap_or_default());
    let selected_quote = use_signal(|| inner.quote_asset.clone().unwrap_or_default());
    let mut amount_sig = use_signal(|| inner.amount.clone().unwrap_or_default());
    let mut price_sig = use_signal(|| inner.limit_price.clone().unwrap_or_default());
    let mut active_pct = use_signal(|| inner.fee_percentage);
    let mut flags_sig = use_signal(|| inner.flags.clone().unwrap_or_default());

    let has_selection = !selected_base().is_empty() && !selected_quote().is_empty();

    // --- SCALABLE RATE ENGINE ---
 let market_rate = use_memo(move || {
        let rates_data = rates_signal.read();
        
        let b_raw = selected_base();
        let q_raw = selected_quote();
        if b_raw.is_empty() || q_raw.is_empty() { return 0.0; }

        // Map base asset to market ticker
        let base = match b_raw.as_str() {
            "RLUSD" => "USD",
            "EUROP" => "EUR",
            _ => b_raw.as_str(),
        };

        // Map quote asset to market ticker
        let quote = match q_raw.as_str() {
            "RLUSD" => "USD",
            "EUROP" => "EUR",
            _ => q_raw.as_str(),
        };

        if base == quote { return 1.0; }

        // Helper for lookup - formatting strings is fine here as it's scoped to the memo
        let lookup = |a: &str, b: &str| -> Option<f64> {
            let direct = format!("{}/{}", a, b);
            if let Some(&rate) = rates_data.get(&direct) {
                return Some(rate as f64);
            }
            let inverse = format!("{}/{}", b, a);
            if let Some(&rate) = rates_data.get(&inverse) {
                if rate > 0.0 { return Some(1.0 / rate as f64); }
            }
            None
        };

        // 1. Direct Pair
        if let Some(rate) = lookup(base, quote) {
            return rate;
        }

        // 2. Triangulation via USD
        if base != "USD" && quote != "USD" {
            let base_usd = lookup(base, "USD");
            let quote_usd = lookup(quote, "USD");
            if let (Some(b_r), Some(q_r)) = (base_usd, quote_usd) {
                if q_r > 0.0 { return b_r / q_r; }
            }
        }

        0.0
    });

    let mut update_price = move |amount_str: String, pct: f64| {
        if let Ok(amt) = amount_str.parse::<f64>() {
            let rate = market_rate();
            if rate > 0.0 {
                let calculated = (amt * rate) * (1.0 + (pct / 100.0));
                price_sig.set(format!("{:.4}", calculated));
            }
        }
    };

    rsx! {
        MarketOrderForm {
            search_query,
            is_searching,
            selected_base,
            selected_quote,
            amount_sig,
            price_sig,
            active_pct,
            flags_sig,
            has_selection,
            market_rate: market_rate(),
            
            on_amount_input: move |e: FormEvent| {
                amount_sig.set(e.value());
                update_price(e.value(), active_pct());
            },
            on_price_input: move |e: FormEvent| {
                price_sig.set(e.value());
            },
            on_slippage_select: move |pct: f64| {
                active_pct.set(pct);
                update_price(amount_sig(), pct);
            },
            on_flag_toggle: move |name: String| {
                let flag_val = format!("tf{}", name);
                let mut current = flags_sig.read().clone();
                if current.contains(&flag_val) {
                    current.retain(|f| f != &flag_val);
                } else {
                    let other = if name == "FillOrKill" { "tfImmediateOrCancel" } else { "tfFillOrKill" };
                    current.retain(|f| f != other);
                    current.push(flag_val);
                }
                flags_sig.set(current);
            },
            on_next_click: move |_| {
                xrp_ctx.trade.with_mut(|s| if let Some(ref mut t) = s.send_trade {
                    t.base_asset = Some(selected_base());
                    t.quote_asset = Some(selected_quote());
                    t.amount = Some(amount_sig());
                    t.limit_price = Some(price_sig());
                    t.fee_percentage = active_pct();
                    t.flags = Some(flags_sig());
                    t.step = 2;
                });
            }
        }
    }
}