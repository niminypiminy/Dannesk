// src/context.rs
use dioxus::prelude::*;
use std::collections::HashMap;
use tokio::sync::watch; // watch::Receiver
use tokio::sync::mpsc; // mpsc::Sender
use crate::channel::{CHANNEL, Tab, ProgressState, SignTradeState, XRPWalletProcessState, 
    BTCWalletProcessState, TransactionState, BTCSignTransactionState, SignTransactionState, XRPModalState, SettingsState, BTCModalState, BTCTransactionState, WSCommand};

// --- Define Context Structs (Bundles of Signals) ---

#[derive(Clone)]
pub struct GlobalContext {
    pub theme_user: Signal<(bool, String, bool)>,
    pub rates: Signal<HashMap<String, f32>>,
    pub selected_tab: Signal<Tab>,
    pub progress: Signal<Option<ProgressState>>,
    pub version: Signal<Option<String>>,
    pub exchange_ws_status: Signal<bool>,
    pub crypto_ws_status: Signal<bool>,
    pub ws_tx: mpsc::Sender<WSCommand>, 
    pub settings_modal: Signal<SettingsState>,

}

#[derive(Clone, Copy)]
pub struct XrpContext {
    pub wallet_balance: Signal<(f64, Option<String>, bool)>,
    pub xrp_modal: Signal<XRPModalState>,
    pub sign_transaction: Signal<SignTransactionState>,
    pub wallet_process: Signal<XRPWalletProcessState>, // Replaces the old fields
    pub transactions: Signal<TransactionState>,
    pub trade: Signal<SignTradeState>,
}

#[derive(Clone, Copy)]
pub struct RlusdContext {
    pub rlusd: Signal<(f64, bool, Option<f64>)>,
}

#[derive(Clone, Copy)]
pub struct EuroContext {
    pub euro: Signal<(f64, bool, Option<f64>)>,
}

#[derive(Clone, Copy)]
pub struct BtcContext {
    pub bitcoin_wallet: Signal<(f64, Option<String>, bool)>,
    pub btc_modal: Signal<BTCModalState>,
    pub btc_transactions: Signal<BTCTransactionState>,
    pub btc_wallet_process: Signal<BTCWalletProcessState>, // Replaces the old fields
    pub btc_sign_transaction: Signal<BTCSignTransactionState>,


}

// Call this in App component. 
pub fn setup_contexts(ws_tx: mpsc::Sender<WSCommand>) {
    // Global Context
    let global = GlobalContext {
        theme_user: use_signal(|| CHANNEL.theme_user_rx.borrow().clone()),
        rates: use_signal(|| CHANNEL.rates_rx.borrow().clone()),
        selected_tab: use_signal(|| CHANNEL.selected_tab_rx.borrow().clone()),
        progress: use_signal(|| CHANNEL.progress_rx.borrow().clone()),
        version: use_signal(|| CHANNEL.version_rx.borrow().clone()),
        exchange_ws_status: use_signal(|| CHANNEL.exchange_ws_status_rx.borrow().clone()),
        crypto_ws_status: use_signal(|| CHANNEL.crypto_ws_status_rx.borrow().clone()),
        ws_tx, // NEW: Add here (no clone needed, as it's passed by value)
        settings_modal: use_signal(|| CHANNEL.settings_modal_rx.borrow().clone()),

    };
use_context_provider(|| global.clone());


    // Spawn coroutines for global channels
    subscribe_to_channel(global.theme_user, CHANNEL.theme_user_rx.clone());
    subscribe_to_channel(global.rates, CHANNEL.rates_rx.clone());
    subscribe_to_channel(global.selected_tab, CHANNEL.selected_tab_rx.clone());
    subscribe_to_channel(global.progress, CHANNEL.progress_rx.clone());
    subscribe_to_channel(global.version, CHANNEL.version_rx.clone());
    subscribe_to_channel(global.exchange_ws_status, CHANNEL.exchange_ws_status_rx.clone());
    subscribe_to_channel(global.crypto_ws_status, CHANNEL.crypto_ws_status_rx.clone());
    subscribe_to_channel(global.settings_modal, CHANNEL.settings_modal_rx.clone());


    // XRP Context 
    let xrp = XrpContext {
        wallet_balance: use_signal(|| CHANNEL.wallet_balance_rx.borrow().clone()),
        xrp_modal: use_signal(|| CHANNEL.xrp_modal_rx.borrow().clone()),
        sign_transaction: use_signal(|| CHANNEL.sign_transaction_rx.borrow().clone()),
        wallet_process: use_signal(|| CHANNEL.xrp_wallet_process_rx.borrow().clone()), // NEW
        transactions: use_signal(|| CHANNEL.transactions_rx.borrow().clone()),
        trade: use_signal(|| CHANNEL.trade_rx.borrow().clone()),

    };
    use_context_provider(|| xrp);

    subscribe_to_channel(xrp.wallet_balance, CHANNEL.wallet_balance_rx.clone());
    subscribe_to_channel(xrp.xrp_modal, CHANNEL.xrp_modal_rx.clone());
    subscribe_to_channel(xrp.sign_transaction, CHANNEL.sign_transaction_rx.clone());
    subscribe_to_channel(xrp.wallet_process, CHANNEL.xrp_wallet_process_rx.clone()); // NEW
    subscribe_to_channel(xrp.transactions, CHANNEL.transactions_rx.clone());
    subscribe_to_channel(xrp.trade, CHANNEL.trade_rx.clone());


    // RLUSD Context
    let rlusd = RlusdContext {
        rlusd: use_signal(|| CHANNEL.rlusd_rx.borrow().clone()),
    };
    use_context_provider(|| rlusd);

    subscribe_to_channel(rlusd.rlusd, CHANNEL.rlusd_rx.clone());

    // Euro Context
    let euro = EuroContext {
        euro: use_signal(|| CHANNEL.euro_rx.borrow().clone()),
    };
    use_context_provider(|| euro);

    subscribe_to_channel(euro.euro, CHANNEL.euro_rx.clone());

    // BTC Context
    let btc = BtcContext {
        bitcoin_wallet: use_signal(|| CHANNEL.bitcoin_wallet_rx.borrow().clone()),
        btc_modal: use_signal(|| CHANNEL.btc_modal_rx.borrow().clone()),
        btc_transactions: use_signal(|| CHANNEL.btc_transactions_rx.borrow().clone()),
        btc_wallet_process: use_signal(|| CHANNEL.btc_wallet_process_rx.borrow().clone()), 
        btc_sign_transaction: use_signal(|| CHANNEL.btc_sign_transaction_rx.borrow().clone()),


    };
    use_context_provider(|| btc);

    subscribe_to_channel(btc.bitcoin_wallet, CHANNEL.bitcoin_wallet_rx.clone());
    subscribe_to_channel(btc.btc_modal, CHANNEL.btc_modal_rx.clone());
    subscribe_to_channel(btc.btc_transactions, CHANNEL.btc_transactions_rx.clone());
    subscribe_to_channel(btc.btc_wallet_process, CHANNEL.btc_wallet_process_rx.clone());
    subscribe_to_channel(btc.btc_sign_transaction, CHANNEL.btc_sign_transaction_rx.clone());
 

}

// Generic subscription coroutine (reuse for all channels)
fn subscribe_to_channel<T: Clone + 'static>(mut signal: Signal<T>, rx: watch::Receiver<T>) {
    use_coroutine(move |_: UnboundedReceiver<()>| {
        let mut rx = rx.clone();
        async move {
            while rx.changed().await.is_ok() {
                signal.set(rx.borrow().clone());
            }
        }
    });
}