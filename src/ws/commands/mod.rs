use tokio_tungstenite::tungstenite::Message;
use crate::ws::connection::ConnectionManager;
use crate::channel::WSCommand;

pub mod import_wallet;
pub mod delete_wallet;
pub mod submit_transaction;
pub mod validation;
pub mod wallet_auth;
pub mod transaction_builder;
pub mod transaction_sender;
pub mod ledger;
pub mod offer_create;
pub mod getcachedbalance;
pub mod payment;
pub mod trustset;
pub mod get_transaction;
pub mod get_trustline_limit;
pub mod get_xrp_balance;
pub mod get_rlusd_balance;
pub mod get_euro_balance;
pub mod bitcoin_import_wallet;
pub mod getbitcoincachedbalance;
pub mod bitcoin_delete_wallet;
pub mod bitcoin_ledger;
pub mod bitcoin_payment;
pub mod bitcoin_submit_transaction;
pub mod bitcoin_transaction_sender;
pub mod bitcoin_validation; 
pub mod bitcoin_auth;
pub mod get_btc_transaction;
pub mod get_bitcoin_balance;
pub mod trustset_euro;
pub mod get_trustline_euro_limit; // New module import



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Command {
    ImportWallet,
    DeleteWallet,
    SubmitTransaction,
    GetLedgerData,
    GetBalance,
    GetTrustlineLimit,
    GetTrustlineEuroLimit, // New command variant
    GetTransaction,
    GetRLUSDBalance,
    GetXRPBalance,
    GetEUROBalance,
    ImportBitcoinWallet, 
    GetBitcoinBalance, 
    DeleteBitcoinWallet,
    SubmitBitcoinTransaction, // Add new command
    GetBitcoinLedgerData,    // Add new command
    GetBTCBalance,
    GetBitcoinTransaction,

  

}

impl Command {
    pub fn from_str(command_name: &str) -> Option<Self> {
        match command_name {
            "import_wallet" => Some(Command::ImportWallet),
            "delete_wallet" => Some(Command::DeleteWallet),
            "submit_transaction" | "submit_transaction_response" => Some(Command::SubmitTransaction),
            "get_ledger_data" => Some(Command::GetLedgerData),
            "get_balance" | "get_cached_balance" => Some(Command::GetBalance),
            "get_trustline_limit" => Some(Command::GetTrustlineLimit),
            "get_trustline_euro_limit" => Some(Command::GetTrustlineEuroLimit), // New mapping
            "get_transaction" => Some(Command::GetTransaction),
            "get_rlusd_balance" => Some(Command::GetRLUSDBalance),
            "get_euro_balance" => Some(Command::GetEUROBalance),
            "xrp_balance" => Some(Command::GetXRPBalance),
            "import_bitcoin_wallet" => Some(Command::ImportBitcoinWallet), 
            "get_bitcoin_cached_balance" => Some(Command::GetBitcoinBalance),
            "delete_bitcoin_wallet" => Some(Command::DeleteBitcoinWallet),
            "bitcoin_submit_transaction" | "submit_bitcoin_transaction" | "submit_bitcoin_transaction_response" => Some(Command::SubmitBitcoinTransaction), // Add "bitcoin_submit_transaction"
            "get_bitcoin_ledger_data" => Some(Command::GetBitcoinLedgerData), // Add new mapping
            "btc_balance" => Some(Command::GetBTCBalance),
"get_bitcoin_transaction" => {
    Some(Command::GetBitcoinTransaction)
}
            _ => None,
        }
    }

  pub async fn execute(
    &self,
    connection: &mut ConnectionManager,
    current_wallet: &mut String,
    bitcoin_current_wallet: &mut String,
    cmd: WSCommand,
) -> Result<(), String> {
    let result = match self {
        Command::ImportWallet => import_wallet::execute(connection, current_wallet, cmd).await,
        Command::DeleteWallet => delete_wallet::execute(connection, current_wallet, cmd).await,
        Command::SubmitTransaction => submit_transaction::execute(connection, current_wallet, cmd).await,
        Command::GetLedgerData => ledger::fetch_ledger_data(connection, cmd.wallet.as_ref().ok_or("Missing wallet")?).await
            .map(|_| ()),
        Command::GetBalance => getcachedbalance::execute(connection, current_wallet, cmd).await,
        Command::GetTrustlineLimit => get_trustline_limit::execute(connection, current_wallet, cmd).await,
        Command::GetTrustlineEuroLimit => get_trustline_euro_limit::execute(connection, current_wallet, cmd).await, // New execution
        Command::GetTransaction => get_transaction::execute(connection, current_wallet, cmd).await,
        Command::GetRLUSDBalance => get_rlusd_balance::execute(connection, current_wallet, cmd).await,
        Command::GetEUROBalance => get_euro_balance::execute(connection, current_wallet, cmd).await,
        Command::GetXRPBalance => get_xrp_balance::execute(connection, current_wallet, cmd).await,
        Command::ImportBitcoinWallet => bitcoin_import_wallet::execute(connection, bitcoin_current_wallet, cmd).await, 
        Command::GetBitcoinBalance => getbitcoincachedbalance::execute(connection, bitcoin_current_wallet, cmd).await, 
        Command::DeleteBitcoinWallet => bitcoin_delete_wallet::execute(connection, bitcoin_current_wallet, cmd).await,
        Command::SubmitBitcoinTransaction => bitcoin_submit_transaction::execute(connection, bitcoin_current_wallet, cmd).await,
        Command::GetBitcoinLedgerData => bitcoin_ledger::fetch_utxo_data(connection, cmd.wallet.as_ref().ok_or("Missing wallet")?).await
            .map(|_| ()), 
        Command::GetBTCBalance => get_bitcoin_balance::execute(connection, bitcoin_current_wallet, cmd).await,
        Command::GetBitcoinTransaction => get_btc_transaction::execute(connection, bitcoin_current_wallet, cmd).await,
    };
    result
}

pub async fn process_response(
    &self,
    message: Message,
    current_wallet: &str,
    bitcoin_current_wallet: &str, 
) -> Result<(), String> {
    match self {
        Command::ImportWallet => import_wallet::process_response(message, current_wallet).await,
        Command::DeleteWallet => delete_wallet::process_response(message, current_wallet).await,
        Command::SubmitTransaction => submit_transaction::process_response(message, current_wallet).await,
        Command::GetLedgerData => ledger::process_response(message, current_wallet).await.map(|_| ()),
        Command::GetBalance => getcachedbalance::process_response(message, current_wallet).await,
        Command::GetTrustlineLimit => get_trustline_limit::process_response(message, current_wallet).await,
        Command::GetTrustlineEuroLimit => get_trustline_euro_limit::process_response(message, current_wallet).await, 
        Command::GetTransaction => get_transaction::process_response(message, current_wallet).await,
        Command::GetRLUSDBalance => get_rlusd_balance::process_response(message, current_wallet).await,
        Command::GetEUROBalance => get_euro_balance::process_response(message, current_wallet).await,
        Command::GetXRPBalance => get_xrp_balance::process_response(message, current_wallet).await,
        Command::ImportBitcoinWallet => bitcoin_import_wallet::process_response(message, bitcoin_current_wallet).await, 
        Command::GetBitcoinBalance => getbitcoincachedbalance::process_response(message, bitcoin_current_wallet).await, 
        Command::DeleteBitcoinWallet => bitcoin_delete_wallet::process_response(message, bitcoin_current_wallet).await,
        Command::SubmitBitcoinTransaction => bitcoin_submit_transaction::process_response(message, bitcoin_current_wallet).await,
        Command::GetBitcoinLedgerData => bitcoin_ledger::process_response(message, bitcoin_current_wallet).await.map(|_| ()), 
        Command::GetBTCBalance => get_bitcoin_balance::process_response(message, bitcoin_current_wallet).await,
        Command::GetBitcoinTransaction => get_btc_transaction::process_response(message, bitcoin_current_wallet).await,
    }
}
}