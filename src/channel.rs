// channel.rs (updated with settings_dropdown channels)
use tokio::sync::watch;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing; // Add this line!

pub static CHANNEL: Lazy<Channel> = Lazy::new(Channel::new);

//global tabs and modals struct

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    Balance,
    XRP,
    BTC,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressState {
    pub progress: f32,   
    pub message: String,  
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)] // Add Default here
pub enum SettingsView {
    #[default] // Mark Name as the starting point
    Name,
    Security,
    Network,
}

#[derive(Debug, Clone, Default)]
pub struct SettingsState {
    pub view_type: SettingsView,
    pub last_view: Option<SettingsView>, // None means settings is closed
}

//xrp, euro and rlusd token related structs

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransactionStatus {
    Success,
    Failed,
    Pending,
    Cancelled,
}

#[derive(Debug, Clone, Default)]
pub struct TransactionState {
    pub transactions: HashMap<String, TransactionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub tx_id: String,          // Maps to "hash"
    pub status: TransactionStatus, // Maps to "status"
    pub execution_price: String, // Maps to "price"
    pub order_type: String,     // Maps to "tx_type"
    pub timestamp: String,      // Maps to "timestamp"
    pub amount: String,         // Maps to "amount" (formatted with currency)
    pub currency: String,       // Maps to "currency"
    pub fee: String,           // Maps to "fee"
    pub flags: Option<String>, // Changed from Option<Vec<String>> to Option<String>
    pub receiver: String,      // Maps to "receiver"
    pub sender: String,        // Maps to "sender"
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignTransaction {
    pub step: u8,
    pub error: Option<String>,
    pub recipient: Option<String>,
    pub amount: Option<String>,
    pub asset: String, // "XRP", "RLUSD", or "EURO"
}

#[derive(Debug, Clone, Default)]
pub struct SignTransactionState {
    pub send_transaction: Option<SignTransaction>,
}

#[derive(Debug, Clone)]
pub struct WSCommand {
    pub command: String,
    pub wallet: Option<String>,
    pub recipient: Option<String>,
    pub amount: Option<String>,
    pub passphrase: Option<Zeroizing<String>>,
    pub trustline_limit: Option<String>,
    pub fee: Option<String>,
    pub tx_type: Option<String>,
    pub taker_pays: Option<(String, String)>,
    pub taker_gets: Option<(String, String)>,
    pub seed: Option<Zeroizing<String>>,
    pub flags: Option<Vec<String>>,
    pub wallet_type: Option<String>, 
    pub bip39: Option<Zeroizing<String>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum ActiveView {
    #[default]
    XRP,
    RLUSD,
    EURO,
    Receive,
    Transactions,
    Import,
    Create,
    Send,
    Trade,
}

#[derive(Debug, Clone, Default)]
pub struct XRPModalState {
    pub view_type: ActiveView,
    pub last_view: Option<ActiveView>,
}

#[derive(Debug, Clone, Default)]
pub struct XRPWalletProcessState {
    pub import_wallet: Option<XRPImport>,
    pub create_wallet: Option<XRPImport>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct XRPImport {
    pub step: u8,
    pub seed: Option<Zeroizing<String>>, 
    pub error: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Trade {
    pub step: u8,
    pub base_asset: Option<String>,  //buy asset
    pub quote_asset: Option<String>, //sell asset
    pub amount: Option<String>,
    pub limit_price: Option<String>,
    pub fee_percentage: f64, // <--- Add this
    pub flags: Option<Vec<String>>,
    pub error: Option<String>,
    pub asset: String, // "XRP", "RLUSD", or "EURO"
 
}

#[derive(Debug, Clone, Default)]
pub struct SignTradeState {
    pub send_trade: Option<Trade>,
}

//bitcoin related structs

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BTCImport {
    pub step: u8,
    pub seed: Option<Zeroizing<String>>, 
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BTCModalState {
    pub view_type: BTCActiveView,
    pub last_view: Option<BTCActiveView>,

}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BTCSignTransaction {
    pub step: u8,
    pub error: Option<String>,
    pub recipient: Option<String>,
    pub amount: Option<String>,
    pub asset: String, 
    pub fee: String, 
}

#[derive(Debug, Clone, Default)]
pub struct BTCSignTransactionState {
    pub send_transaction: Option<BTCSignTransaction>,
}

#[derive(Debug, Clone, Default)]
pub struct BTCWalletProcessState {
    pub import_wallet: Option<BTCImport>,
    pub create_wallet: Option<BTCImport>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum BTCActiveView {
    #[default]
    BTC,
    Receive,
    Transactions,
    Import,   // Added
    Create,   // Added
    Send,     // Added
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BitcoinTransactionStatus {
    Success,
    Failed,
    Pending,
    Cancelled,
}

#[derive(Debug, Clone, Default)]
pub struct BTCTransactionState {
    pub transactions: HashMap<String, BTCTransactionData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BTCTransactionData {
    pub txid: String,          // Transaction ID (txid)
    pub status: BitcoinTransactionStatus, // Pending, Success, Failed, or Cancelled
    pub amount: String,         // Amount transferred to non-wallet addresses (in satoshis)
    pub fees: String,            // Fee in satoshis
    pub receiver_addresses: Vec<String>, // List of recipient addresses
    pub sender_addresses: Vec<String>,   // List of sender addresses
    pub timestamp: String,      // ISO 8601 timestamp
}

//channels

pub struct Channel {
    //global related channels
    pub rates_tx: watch::Sender<HashMap<String, f32>>,
    pub rates_rx: watch::Receiver<HashMap<String, f32>>,
    pub selected_tab_tx: watch::Sender<Tab>,
    pub selected_tab_rx: watch::Receiver<Tab>,
    pub theme_user_tx: watch::Sender<(bool, String, bool)>,
    pub theme_user_rx: watch::Receiver<(bool, String, bool)>,
    pub progress_tx: watch::Sender<Option<ProgressState>>,
    pub progress_rx: watch::Receiver<Option<ProgressState>>,
    pub version_tx: watch::Sender<Option<String>>,
    pub version_rx: watch::Receiver<Option<String>>,
    pub exchange_ws_status_tx: watch::Sender<bool>,
    pub exchange_ws_status_rx: watch::Receiver<bool>,
    pub crypto_ws_status_tx: watch::Sender<bool>,
    pub crypto_ws_status_rx: watch::Receiver<bool>, 
    pub settings_modal_tx: watch::Sender<SettingsState>,
    pub settings_modal_rx: watch::Receiver<SettingsState>,

    //rlusd channels
    pub rlusd_tx: watch::Sender<(f64, bool, Option<f64>)>,
    pub rlusd_rx: watch::Receiver<(f64, bool, Option<f64>)>,
    
    //xrp channels
    pub wallet_balance_tx: watch::Sender<(f64, Option<String>, bool)>,
    pub wallet_balance_rx: watch::Receiver<(f64, Option<String>, bool)>,
    pub xrp_modal_tx: watch::Sender<XRPModalState>,
    pub xrp_modal_rx: watch::Receiver<XRPModalState>,
    pub sign_transaction_tx: watch::Sender<SignTransactionState>,
    pub sign_transaction_rx: watch::Receiver<SignTransactionState>,
    pub transactions_tx: watch::Sender<TransactionState>,
    pub transactions_rx: watch::Receiver<TransactionState>,
    pub xrp_wallet_process_tx: watch::Sender<XRPWalletProcessState>,
    pub xrp_wallet_process_rx: watch::Receiver<XRPWalletProcessState>,
    pub trade_tx: watch::Sender<SignTradeState>,
    pub trade_rx: watch::Receiver<SignTradeState>,
  

    //euro channels
    pub euro_tx: watch::Sender<(f64, bool, Option<f64>)>,
    pub euro_rx: watch::Receiver<(f64, bool, Option<f64>)>,
    
    //bitcoin related channels
    pub btc_modal_tx: watch::Sender<BTCModalState>,
    pub btc_modal_rx: watch::Receiver<BTCModalState>,
    pub bitcoin_wallet_tx: watch::Sender<(f64, Option<String>, bool)>, 
    pub bitcoin_wallet_rx: watch::Receiver<(f64, Option<String>, bool)>,
    pub btc_transactions_rx: watch::Receiver<BTCTransactionState>,
    pub btc_transactions_tx: watch::Sender<BTCTransactionState>,
    pub btc_wallet_process_tx: watch::Sender<BTCWalletProcessState>,
    pub btc_wallet_process_rx: watch::Receiver<BTCWalletProcessState>,
    pub btc_sign_transaction_tx: watch::Sender<BTCSignTransactionState>,
    pub btc_sign_transaction_rx: watch::Receiver<BTCSignTransactionState>,
}

impl Channel {
    pub fn new() -> Self {

        //global related
        let (theme_user_tx, theme_user_rx) = watch::channel((true, "anonymous".to_string(), false));
        let (rates_tx, rates_rx) = watch::channel(HashMap::new());
        let (selected_tab_tx, selected_tab_rx) = watch::channel(Tab::Balance);
        let (progress_tx, progress_rx) = watch::channel(None);
        let (version_tx, version_rx) = watch::channel(None);
        let (exchange_ws_status_tx, exchange_ws_status_rx) = watch::channel(false);
        let (crypto_ws_status_tx, crypto_ws_status_rx) = watch::channel(false);
        let (settings_modal_tx, settings_modal_rx) = watch::channel(SettingsState::default());

        

        //rlusd related
        let (rlusd_tx, rlusd_rx) = watch::channel((0.0, false, None));

        //xrp related
        let (wallet_balance_tx, wallet_balance_rx) = watch::channel((0.0, None, false));
        let (xrp_modal_tx, xrp_modal_rx) = watch::channel(XRPModalState::default());
        let (sign_transaction_tx, sign_transaction_rx) = watch::channel(SignTransactionState::default());
        let (xrp_wallet_process_tx, xrp_wallet_process_rx) = watch::channel(XRPWalletProcessState::default());
        let (transactions_tx, transactions_rx) = watch::channel(TransactionState::default());
        let (trade_tx, trade_rx) = watch::channel(SignTradeState::default());

     
        //euro related
        let (euro_tx, euro_rx) = watch::channel((0.0, false, None));

       //btc related 
        let (bitcoin_wallet_tx, bitcoin_wallet_rx) = watch::channel((0.0, None, false)); 
        let (btc_modal_tx, btc_modal_rx) = watch::channel(BTCModalState::default());
        let (btc_transactions_tx, btc_transactions_rx) = watch::channel(BTCTransactionState::default());
        let (btc_sign_transaction_tx, btc_sign_transaction_rx) = watch::channel(BTCSignTransactionState::default());
        let (btc_wallet_process_tx, btc_wallet_process_rx) = watch::channel(BTCWalletProcessState::default());


        Channel {

            //global
            theme_user_tx,
            theme_user_rx,
            rates_tx,
            rates_rx,
            selected_tab_tx,
            selected_tab_rx,
            progress_tx,
            progress_rx,
            version_tx,
            version_rx,
            exchange_ws_status_tx,
            exchange_ws_status_rx,
            crypto_ws_status_tx,
            crypto_ws_status_rx,
            settings_modal_tx,
            settings_modal_rx,
          
            //xrp, euro and rlusd related
            rlusd_tx,
            rlusd_rx,
            wallet_balance_tx,
            wallet_balance_rx,
            xrp_modal_tx,
            xrp_modal_rx,
            xrp_wallet_process_tx,
            xrp_wallet_process_rx,
            sign_transaction_tx,
            sign_transaction_rx,
            euro_tx,
            euro_rx,
            transactions_tx,
            transactions_rx,
            trade_tx,
            trade_rx,
              
            //bitcoin related
            bitcoin_wallet_tx,
            bitcoin_wallet_rx,
            btc_modal_tx,
            btc_modal_rx,
            btc_transactions_rx,
            btc_transactions_tx,
            btc_wallet_process_tx,
            btc_wallet_process_rx,
            btc_sign_transaction_tx,
            btc_sign_transaction_rx,
            
        }
    }
}

