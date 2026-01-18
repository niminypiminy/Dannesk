use dioxus::prelude::*;
use crate::context::XrpContext;
use crate::channel::XRPWalletProcessState;
use arboard::Clipboard;

#[component]
pub fn view() -> Element {
    let xrp_ctx = use_context::<XrpContext>();
    let mut wallet_process = xrp_ctx.wallet_process;
    
    let modal_state = wallet_process.peek();

    let seed_wrapper = modal_state.create_wallet
        .as_ref()
        .and_then(|s| s.seed.as_ref());
        
    let words: Vec<&str> = seed_wrapper
        .map(|s| s.split_whitespace().collect())
        .unwrap_or_default();

    let on_next_click = move |_| {
        wallet_process.with_mut(|state: &mut XRPWalletProcessState| {
            if let Some(ref mut create) = state.create_wallet {
                create.step = 2;
            }
        });
    };

    let on_copy_click = move |_| {
        if let Ok(mut ctx) = Clipboard::new() {
            if let Some(ref create) = wallet_process.peek().create_wallet {
                if let Some(ref seed) = create.seed {
                    let _ = ctx.set_text(seed.to_string());
                }
            }
        }
    };

     rsx! {
        style { {r#"
    .create-step1-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
    }
    .title {
        font-size: 1.5rem;
        font-weight: bold;
        margin-top: 1.5rem;
        margin-bottom: 1rem;
        color: var(--text); /* Use theme text color */
    }
    .instruction {
        color: var(--text-secondary); /* Use theme secondary text */
        font-size: 1rem;
        margin-bottom: 20px;
        text-align: center;
        max-width: 400px;
    }
    .words-grid {
        display: grid;
        grid-template-columns: repeat(4, 1fr);
        gap: 0.5rem;
        padding: 1.2rem;
    }
    .word-chip {
        /* Use the card background and border from your theme */
        background-color: var(--bg-card);
        border: 1px solid var(--border);
        padding: 0.5rem 1rem;
        border-radius: 0.5rem;
        display: flex;
        align-items: center;
    }
    .word-index {
        color: var(--text-secondary);
        font-size: 0.8rem; /* Slightly smaller for better hierarchy */
        margin-right: 0.5rem;
        user-select: none;
    }
    .word-text {
        font-weight: 500;
        font-family: monospace;
        /* This will be white in dark mode and dark gray in light mode */
        color: var(--text); 
        font-size: 1rem;
    }
    .button-row {
        display: flex;
        flex-direction: row;
        margin-top: 30px;
    }
    .action-btn {
        width: 8.75rem; 
        height: 2.25rem; 
        /* Use your theme button variables */
        background-color: var(--btn); 
        color: #fffef9; /* Keep text light for the dark buttons */
        border: none; 
        border-radius: 1.375rem; 
        font-size: 1rem; 
        display: flex; 
        cursor: pointer; 
        justify-content: center; 
        align-items: center;
        margin-left: 1rem;
        margin-right: 1rem;
        transition: background-color 0.2s;
    }
    .action-btn:hover {
        background-color: var(--btn-hover);
    }
"#} }

        div { class: "create-step1-container",
            div { class: "title", "Write Down Your Recovery Phrase" }
            div { class: "instruction", 
                "Please write down these 24 words in the correct order and store them safely. "
            }

            div { class: "words-grid",
                for (i, word) in words.iter().enumerate() {
                    div { 
                        key: "{i}",
                        class: "word-chip",
                        span { class: "word-index", "{i + 1}" }
                        span { class: "word-text", "{word}" }
                    }
                }
            }

            div { class: "button-row",
                button { 
                    class: "action-btn",
                    onclick: on_copy_click,
                    "Copy"
                }
                button { 
                    class: "action-btn",
                    onclick: on_next_click,
                    "Continue"
                }
            }
        }
    }
}