// src/styles/theme.rs

pub const DARK_CSS: &str = r#"
:root, .theme-root {
    --bg-primary: #0a0a0a;
    --bg-secondary: #000000;     
    --bg-card: #141414;
    --bg-faint: #1c1c1c;         
    --bg-grid: rgba(255, 255, 255, 0.02); /* Industrial dark tint */
    --border: #262626;
    --input-border: rgba(255, 255, 255, 0.1);
    --text: #e5e5e5;
    --text-secondary: #737373;
    --btn: #141414;
    --btn-hover: #ffffff;
    --btn-active: #d4d4d4;
    --accent: #ffffff;
    --selection: rgba(255, 255, 255, 0.1);
    --input-bg: #0a0a0a;
    --focus-ring: #525252;
    --status-ok: #10b981;
    --status-warn: #ef4444;
}
.theme-root {
    background: var(--bg-primary);
    color: var(--text);
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}
.market-value, .monospace-data { font-family: 'JetBrains Mono', monospace; }

.theme-btn {
    background: transparent; /* Force transparency as requested */
    color: var(--text);
    border: 1px solid var(--border);
    padding: 6px 12px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.1em;
}
"#;

pub const LIGHT_CSS: &str = r#"
:root, .theme-root {
    /* Backgrounds */
    --bg-primary: #f8fafc;       /* slate-50 */
    --bg-secondary: #ffffff;     /* Pure white for cards to pop against slate-50 */
    --bg-card: #ffffff;          
    --bg-faint: #f1f5f9;         /* slate-100 for subtle offsets */
    --bg-grid: #f8fafc; 

    /* Borders & Dividers */
    --border: #e2e8f0;           /* slate-200 (Matches your Hero's border-slate-200/80) */
    --input-border: #cbd5e1;     /* slate-300 */

    /* Text */
    --text: #020617;             /* slate-950 (Primary Text) */
    --text-secondary: #64748b;   /* slate-500 (Muted labels) */
    --text-accent: #334155;      /* slate-700 (Logo/Sub-headers) */

    /* Interactive */
    --btn: #020617;              /* slate-950 */
    --btn-hover: #334155;        /* slate-700 */
    --btn-active: #0f172a;       /* slate-900 */
    --accent: #020617;           
    
   --selection: #e2e8f0;
    
    --input-bg: #ffffff;
    --focus-ring: #cbd5e1;       /* slate-300 */
    
    /* Status (Kept for functional clarity) */
    --status-ok: #16a34a;        /* green-600 to match your pulse dot */
    --status-warn: #dc2626;      /* red-600 */
}

.theme-root {
    background: var(--bg-primary);
    color: var(--text);
    font-family: 'JetBrains Mono', monospace; /* Ensuring Mono parity with Hero */
}

.market-value, .monospace-data { font-family: 'JetBrains Mono', monospace; }

.theme-btn {
    background: transparent;
    color: var(--text);
    border: 1px solid var(--border);
    padding: 6px 12px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    font-size: 11px; /* Matching your Hero's small caps style */
    transition: all 0.2s ease;
}

"#;