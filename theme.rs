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
    --bg-primary: #ffffff;
    --bg-secondary: #f8fafc;
    --bg-card: #ffffff;          
    --bg-faint: #f1f5f9;
    --bg-grid: #f8fafc; /* Visible background for light mode */
    --border: #e2e8f0;
    --input-border: #cbd5e1;
    --text: #0f172a;
    --text-secondary: #64748b;
    --btn: #0f172a;
    --btn-hover: #334155;        
    --btn-active: #020617;
    --accent: #0f172a;           
    --selection: rgba(15, 23, 42, 0.08);
    --input-bg: #ffffff;
    --focus-ring: #94a3b8;
    --status-ok: #059669;
    --status-warn: #dc2626;
}
.theme-root {
    background: var(--bg-primary);
    color: var(--text);
}
.market-value, .monospace-data { font-family: 'JetBrains Mono', monospace; }

.theme-btn {
    background: transparent; /* Force transparency as requested */
    color: var(--text);
    border: 1px solid var(--border);
    padding: 6px 12px;
    font-weight: 500;
    letter-spacing: 0.1em;
}
"#;