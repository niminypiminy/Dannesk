use crate::channel::{CHANNEL, SignTransaction, SendRLUSDTransaction, SendEuroTransaction};

pub fn update_state_on_success() {
    // Handle SignTransaction State Update (close channel)
    let sign_state = CHANNEL.sign_transaction_rx.borrow().clone();
    if sign_state.send_transaction.is_some() && !sign_state.send_transaction.as_ref().map_or(false, |tx| tx.done) {
        let mut new_sign_state = sign_state.clone();
        new_sign_state.send_transaction = Some(SignTransaction {
            step: 2, // Same step or your relevant final step
            loading: false,
            error: None,
            done: true,
            buffer_id: sign_state.send_transaction.as_ref().and_then(|tx| tx.buffer_id.clone()),
        });
        let _ = CHANNEL.sign_transaction_tx.send(new_sign_state);
    }

    // Handle SendRLUSDTransaction State Update (close channel)
    let send_rlusd_state = CHANNEL.send_rlusd_rx.borrow().clone();
    if send_rlusd_state.send_rlusd.is_some() && !send_rlusd_state.send_rlusd.as_ref().map_or(false, |tx| tx.done) {
        let mut new_send_rlusd_state = send_rlusd_state.clone();
        new_send_rlusd_state.send_rlusd = Some(SendRLUSDTransaction {
            step: 2, // Same step or your relevant final step
            loading: false,
            error: None,
            done: true,
            buffer_id: send_rlusd_state.send_rlusd.as_ref().and_then(|tx| tx.buffer_id.clone()),
        });
        let _ = CHANNEL.send_rlusd_tx.send(new_send_rlusd_state);
    }

    // Handle SendEuroTransaction State Update (close channel)
    let send_euro_state = CHANNEL.send_euro_rx.borrow().clone();
    if send_euro_state.send_euro.is_some() && !send_euro_state.send_euro.as_ref().map_or(false, |tx| tx.done) {
        let mut new_send_euro_state = send_euro_state.clone();
        new_send_euro_state.send_euro = Some(SendEuroTransaction {
            step: 2, // Same step or your relevant final step
            loading: false,
            error: None,
            done: true,
            buffer_id: send_euro_state.send_euro.as_ref().and_then(|tx| tx.buffer_id.clone()),
        });
        let _ = CHANNEL.send_euro_tx.send(new_send_euro_state);
    }
}