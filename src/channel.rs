use tokio::sync::watch;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static CHANNEL: Lazy<Channel> = Lazy::new(Channel::new);

//global tabs and modals struct

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    Balance,
    XRP,
    BTC,
}

// Startup data structure
#[derive(Serialize, Deserialize, Clone)]
pub struct StartupData {
    pub private_key: Vec<u8>, // ED25519 seed (68 bytes)
    pub public_key: Vec<u8>,  // ED25519 public key (32 bytes)
}

#[derive(Debug, Clone, Default)]
pub struct ModalState {
    pub settings: bool,
    pub exchange: bool,
    pub name: bool,
    pub websocket: bool, 
}


#[derive(Debug, Clone, PartialEq)]
pub struct ProgressState {
    pub progress: f32,   
    pub message: String,  
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
    pub loading: bool,
    pub error: Option<String>,
    pub done: bool,
    pub buffer_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SignTransactionState {
    pub send_transaction: Option<SignTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendRLUSDTransaction {
    pub step: u8,
    pub loading: bool,
    pub error: Option<String>,
    pub done: bool,
    pub buffer_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SendRLUSDTransactionState {
    pub send_rlusd: Option<SendRLUSDTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendEuroTransaction {
    pub step: u8,
    pub loading: bool,
    pub error: Option<String>,
    pub done: bool,
    pub buffer_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SendEuroTransactionState {
    pub send_euro: Option<SendEuroTransaction>,
}

#[derive(Debug, Clone)]
pub struct WSCommand {
    pub command: String,
    pub wallet: Option<String>,
    pub recipient: Option<String>,
    pub amount: Option<String>,
    pub passphrase: Option<String>,
    pub trustline_limit: Option<String>,
    pub tx_type: Option<String>,
    pub taker_pays: Option<(String, String)>,
    pub taker_gets: Option<(String, String)>,
    pub seed: Option<String>,
    pub flags: Option<Vec<String>>,
    pub wallet_type: Option<String>, // New field: "XRP", "RLUSD", or None

}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ActiveView {
    #[default]
    XRP,
    RLUSD,
    EURO,
    TrustLine,
    Receive,
    ReceiveRLUSD,
    Transactions,
    Trade,
    Enable,
    ReceiveEURO,
    TrustLineEURO,
    EnableEURO,
    InfoEuro,
    InfoRLUSD, 
}

#[derive(Debug, Clone, Default)]
pub struct XRPModalState {
    pub import_wallet: Option<XRPImport>,
    pub create_wallet: Option<XRPImport>,
    pub view_type: ActiveView,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct XRPImport {
    pub step: u8,
    pub loading: bool,
    pub seed: Option<String>,
    pub error: Option<String>,
    pub done: bool,
    pub buffer_id: Option<String>,
}

//bitcoin related structs

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BTCImport {
    pub step: u8,
    pub loading: bool,
    pub seed: Option<String>,
    pub error: Option<String>,
    pub done: bool,
    pub buffer_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BTCModalState {
    pub import_wallet: Option<BTCImport>,
    pub create_wallet: Option<BTCImport>,
    pub view_type: BTCActiveView,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum BTCActiveView {
    #[default]
    BTC,
    Receive,
    Transactions,
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
    pub modal_tx: watch::Sender<ModalState>,
    pub modal_rx: watch::Receiver<ModalState>,
    pub theme_user_tx: watch::Sender<(bool, String, bool)>,
    pub theme_user_rx: watch::Receiver<(bool, String, bool)>,
    pub progress_tx: watch::Sender<Option<ProgressState>>,
    pub progress_rx: watch::Receiver<Option<ProgressState>>,
    pub startup_tx: watch::Sender<Option<StartupData>>,
    pub startup_rx: watch::Receiver<Option<StartupData>>,
    pub version_tx: watch::Sender<Option<String>>,
    pub version_rx: watch::Receiver<Option<String>>,
    pub exchange_ws_status_tx: watch::Sender<bool>,
    pub exchange_ws_status_rx: watch::Receiver<bool>,
    pub crypto_ws_status_tx: watch::Sender<bool>,
    pub crypto_ws_status_rx: watch::Receiver<bool>,
    pub update_url_tx: watch::Sender<Option<String>>, // New channel for update URL
    pub update_url_rx: watch::Receiver<Option<String>>, // New channel for update URL
    
 

    //rlusd channels
    pub rlusd_tx: watch::Sender<(f64, bool, Option<f64>)>,
    pub rlusd_rx: watch::Receiver<(f64, bool, Option<f64>)>,
    pub send_rlusd_tx: watch::Sender<SendRLUSDTransactionState>,
    pub send_rlusd_rx: watch::Receiver<SendRLUSDTransactionState>,

    //xrp channels
    pub wallet_balance_tx: watch::Sender<(f64, Option<String>, bool, bool)>,
    pub wallet_balance_rx: watch::Receiver<(f64, Option<String>, bool, bool)>,
    pub xrp_modal_tx: watch::Sender<XRPModalState>,
    pub xrp_modal_rx: watch::Receiver<XRPModalState>,
    pub sign_transaction_tx: watch::Sender<SignTransactionState>,
    pub sign_transaction_rx: watch::Receiver<SignTransactionState>,
    pub transactions_tx: watch::Sender<TransactionState>,
    pub transactions_rx: watch::Receiver<TransactionState>,

    //euro channels
    pub euro_tx: watch::Sender<(f64, bool, Option<f64>)>,
    pub euro_rx: watch::Receiver<(f64, bool, Option<f64>)>,
    pub send_euro_tx: watch::Sender<SendEuroTransactionState>,
    pub send_euro_rx: watch::Receiver<SendEuroTransactionState>,

    //bitcoin related channels
    pub btc_modal_tx: watch::Sender<BTCModalState>,
    pub btc_modal_rx: watch::Receiver<BTCModalState>,
    pub bitcoin_wallet_tx: watch::Sender<(f64, Option<String>, bool)>, 
    pub bitcoin_wallet_rx: watch::Receiver<(f64, Option<String>, bool)>,
    pub btc_transactions_rx: watch::Receiver<BTCTransactionState>,
    pub btc_transactions_tx: watch::Sender<BTCTransactionState>,
}

impl Channel {
    pub fn new() -> Self {

        //global related
        let (theme_user_tx, theme_user_rx) = watch::channel((true, "anonymous".to_string(), false));
        let (rates_tx, rates_rx) = watch::channel(HashMap::new());
        let (selected_tab_tx, selected_tab_rx) = watch::channel(Tab::Balance);
        let (modal_tx, modal_rx) = watch::channel(ModalState::default());
        let (progress_tx, progress_rx) = watch::channel(None);
        let (startup_tx, startup_rx) = watch::channel(None);
        let (version_tx, version_rx) = watch::channel(None);
        let (exchange_ws_status_tx, exchange_ws_status_rx) = watch::channel(false);
        let (crypto_ws_status_tx, crypto_ws_status_rx) = watch::channel(false);
        let (update_url_tx, update_url_rx) = watch::channel(None); // Initialize new channel




        //rlusd related

        let (rlusd_tx, rlusd_rx) = watch::channel((0.0, false, None));
        let (send_rlusd_tx, send_rlusd_rx) = watch::channel(SendRLUSDTransactionState::default());

        //xrp related
        let (wallet_balance_tx, wallet_balance_rx) = watch::channel((0.0, None, false, false));
        let (xrp_modal_tx, xrp_modal_rx) = watch::channel(XRPModalState::default());
        let (sign_transaction_tx, sign_transaction_rx) = watch::channel(SignTransactionState::default());
        let (transactions_tx, transactions_rx) = watch::channel(TransactionState::default());
     
        //euro related
        let (send_euro_tx, send_euro_rx) = watch::channel(SendEuroTransactionState::default());
        let (euro_tx, euro_rx) = watch::channel((0.0, false, None));

       //btc related 
        let (bitcoin_wallet_tx, bitcoin_wallet_rx) = watch::channel((0.0, None, false)); 
        let (btc_modal_tx, btc_modal_rx) = watch::channel(BTCModalState::default());
        let (btc_transactions_tx, btc_transactions_rx) = watch::channel(BTCTransactionState::default());



        Channel {

            //global
            theme_user_tx,
            theme_user_rx,
            rates_tx,
            rates_rx,
            selected_tab_tx,
            selected_tab_rx,
            modal_tx,
            modal_rx,
            progress_tx,
            progress_rx,
            startup_tx,
            startup_rx,
            version_tx,
            version_rx,
            exchange_ws_status_tx,
            exchange_ws_status_rx,
            crypto_ws_status_tx,
            crypto_ws_status_rx,
            update_url_tx, 
            update_url_rx, 
         


            //xrp, euro and rlusd related
            rlusd_tx,
            rlusd_rx,
            wallet_balance_tx,
            wallet_balance_rx,
            xrp_modal_tx,
            xrp_modal_rx,
            sign_transaction_tx,
            sign_transaction_rx,
            send_rlusd_tx,
            send_rlusd_rx,
            send_euro_tx,
            send_euro_rx,
            euro_tx,
            euro_rx,
            transactions_tx,
            transactions_rx,
              
            //bitcoin related
            bitcoin_wallet_tx,
            bitcoin_wallet_rx,
            btc_modal_tx,
            btc_modal_rx,
            btc_transactions_rx,
            btc_transactions_tx,
            
        }
    }
}

