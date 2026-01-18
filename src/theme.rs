// src/styles/theme.rs

pub const DARK_CSS: &str = r#"
    :root, .theme-root {
        --bg-primary: #1e1e1e;
        --bg-secondary: #2d2d2d;
        --bg-card: #2a2a2a;
        --bg-faint: #353535;
        --border: #444444;
        --text: #fffef9; /* Inherited WHITE text for general UI */
        --text-secondary: #aaaaaa;
        --btn: #3a3a3c;
        --btn-hover: #555557;
        --btn-active: #64646f;
        --accent: #64646f;
        --selection: #505050;

        /* Input Variables: Set to TRUE GREY */
        --input-bg: #444444; 
        --focus-ring: #64646f;
    }
    .theme-root {
        background: var(--bg-primary);
        color: var(--text);
    }
    .theme-bg-primary   { background: var(--bg-primary); }
    .theme-bg-secondary { background: var(--bg-secondary); }
    .theme-bg-card      { background: var(--bg-card); }
    --theme-border       { border-color: var(--border); }
    .theme-text         { color: var(--text); }
    .theme-text-secondary { color: var(--text-secondary); }
    .theme-btn {
        background: var(--btn);
        color: var(--text);
        border: 1px solid var(--border); /* Base border, overridden by inline style */
        /* Explicitly defining padding/font/transition here for consistency */
        padding: 6px 12px;
        font-weight: 500;
        transition: all 0.15s ease;
    }
    .theme-btn:hover { background: var(--btn-hover); }
    .theme-btn:active { background: var(--btn-active); }

    .theme-input-bg     { background: var(--input-bg); }
    .focus\:theme-focus-ring:focus {
        --tw-ring-color: var(--focus-ring) !important;
    }

    /* Placeholder styles for inputs */
    input::placeholder {
        color: var(--text-secondary);
        opacity: 1;
    }
"#;

pub const LIGHT_CSS: &str = r#"
    :root, .theme-root {
        /* Clean white + soft warm gray palette — professional, calm, modern */
        --bg-primary: #fdfdfb;           /* Slightly off-white, warm */
        --bg-secondary: #f5f5f3;         /* Very light warm gray */
        --bg-card: #ffffff;
        --bg-faint: #eeeeec;
        --border: #d4d4d0;               /* Soft warm gray border */
        --text: #1a1a1a;                 /* Almost black, high contrast */
        --text-secondary: #666666;
        --btn: #3a3a3c;                  /* SAME dark gray as dark theme buttons */
        --btn-hover: #555557;
        --btn-active: #64646f;
        --accent: #5a5a66;
        --selection: #e0e0e0;

        /* 🌟 NEW: Input Specific Variables */
        --input-bg: #ffffff; 
        --focus-ring: #888;
    }
    .theme-root {
        background: var(--bg-primary);
        color: var(--text);
    }
    .theme-bg-primary   { background: var(--bg-primary); }
    .theme-bg-secondary { background: var(--bg-secondary); }
    .theme-bg-card      { background: var(--bg-card); border: 1px solid var(--border); }
    .theme-border       { border-color: var(--border); }
    .theme-text         { color: var(--text); }
    .theme-text-secondary { color: var(--text-secondary); }

    /* Buttons: dark neutral gray — consistent with dark mode */
    .theme-btn {
        background: var(--btn);
        color: #fffef9;
        border: 1px solid #555; /* Base border, overridden by inline style */
        /* Explicitly defining padding/font/transition here for consistency */
        padding: 6px 12px;
        font-weight: 500;
        transition: all 0.15s ease;
    }
    .theme-btn:hover {
        background: var(--btn-hover);
        border-color: #666;
        transform: translateY(-1px);
        box-shadow: 0 2px 4px rgba(0,0,0,0.05);
    }
    .theme-btn:active {
        background: var(--btn-active);
        transform: translateY(0);
    }

    /* 🌟 NEW: Input Specific Classes */
    .theme-input-bg     { background: var(--input-bg); }
    .focus\:theme-focus-ring:focus {
        outline: none !important; /* Override default outline */
        --tw-ring-color: var(--focus-ring) !important;
    }

    /* Placeholder styles for inputs */
    input::placeholder {
        color: var(--text-secondary);
        opacity: 1;
    }
"#;