pub const LOGO_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="64" height="64">
  <rect x="2" y="2" width="60" height="60" rx="4" fill="#0F2937" stroke="#008B9E" stroke-width="2.5"/>
  <circle cx="24" cy="24" r="10" fill="#F59E0B"/>
  <path d="M24 9V12M24 36V39M9 24H12M36 24H39M13.4 13.4L15.5 15.5M32.5 32.5L34.6 34.6M34.6 13.4L32.5 15.5M15.5 32.5L13.4 34.6" stroke="#F59E0B" stroke-width="2.2" stroke-linecap="square"/>
  <path d="M10 42H46C49.5 42 52 39.5 52 36.5C52 33.5 49.5 31 46.5 31C44 31 42 32.8 41.5 35" fill="none" stroke="#06B6D4" stroke-width="3" stroke-linecap="square"/>
  <path d="M10 49H50" fill="none" stroke="#008B9E" stroke-width="3" stroke-linecap="square"/>
  <path d="M10 55H38C41 55 43 57 43 59" fill="none" stroke="#38BDF8" stroke-width="2.5" stroke-linecap="square"/>
</svg>"##;

pub const WEBMANIFEST_JSON: &str = r##"{
  "name": "WS2300 Weather Telemetry",
  "short_name": "WS2300",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#F4FBFB",
  "theme_color": "#008B9E",
  "icons": [
    {
      "src": "/favicon.svg",
      "sizes": "any",
      "type": "image/svg+xml"
    }
  ]
}"##;

pub const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>WS2300 Weather Telemetry</title>
  <link rel="icon" type="image/svg+xml" href="/favicon.svg">
  <link rel="apple-touch-icon" href="/favicon.svg">
  <link rel="manifest" href="/site.webmanifest">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Chakra+Petch:wght@500;600;700&family=JetBrains+Mono:wght@400;500;600&family=Space+Grotesk:wght@400;500;600;700&display=swap" rel="stylesheet">
  <style>
    :root {
      --bg-page: #F2FBFD;
      --bg-surface: #FFFFFF;
      --bg-subtle: #E4F6F9;
      --bg-header: #FFFFFF;
      --border-main: #9ED6E0;
      --border-strong: #008B9E;
      --accent-primary: #008B9E;
      --accent-hover: #006F7E;
      --accent-cyan: #06B6D4;
      --accent-sky: #0284C7;
      --accent-solar: #D97706;
      --text-main: #0F2937;
      --text-muted: #436475;
      --status-ok: #0D9488;
      --status-stop: #475569;
      --status-err: #DC2626;
      --radius-sharp: 3px;
      --font-display: 'Chakra Petch', 'Space Grotesk', sans-serif;
      --font-body: 'Space Grotesk', system-ui, sans-serif;
      --font-mono: 'JetBrains Mono', monospace;
    }

    * {
      box-sizing: border-box;
      margin: 0;
      padding: 0;
    }

    body {
      background-color: var(--bg-page);
      background-image:
        linear-gradient(to right, rgba(0, 139, 158, 0.04) 1px, transparent 1px),
        linear-gradient(to bottom, rgba(0, 139, 158, 0.04) 1px, transparent 1px);
      background-size: 28px 28px;
      color: var(--text-main);
      font-family: var(--font-body);
      line-height: 1.5;
      min-height: 100vh;
    }

    /* Header */
    .top-header {
      background: var(--bg-header);
      border-bottom: 2px solid var(--border-strong);
      padding: 0.75rem 1.5rem;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 1rem;
      position: sticky;
      top: 0;
      z-index: 30;
      box-shadow: 0 2px 10px rgba(0, 139, 158, 0.07);
    }

    .brand-group {
      display: flex;
      align-items: center;
      gap: 0.85rem;
      text-decoration: none;
      color: inherit;
    }

    .brand-logo {
      width: 40px;
      height: 40px;
      flex-shrink: 0;
    }

    .brand-title {
      font-family: var(--font-display);
      font-size: 1.35rem;
      font-weight: 700;
      letter-spacing: 0.06em;
      text-transform: uppercase;
      color: var(--text-main);
    }

    .brand-subtitle {
      font-family: var(--font-mono);
      font-size: 0.72rem;
      color: var(--accent-primary);
      letter-spacing: 0.08em;
      text-transform: uppercase;
      display: block;
    }

    .header-controls {
      display: flex;
      align-items: center;
      gap: 0.65rem;
      flex-wrap: wrap;
    }

    /* Badges */
    .badge {
      display: inline-flex;
      align-items: center;
      gap: 0.45rem;
      font-family: var(--font-display);
      font-size: 0.82rem;
      font-weight: 700;
      letter-spacing: 0.06em;
      text-transform: uppercase;
      padding: 0.35rem 0.75rem;
      border-radius: var(--radius-sharp);
      border: 1px solid transparent;
    }

    .badge-running {
      background: #CCFBF1;
      color: #0F766E;
      border-color: #14B8A6;
    }

    .badge-stopped {
      background: #E2E8F0;
      color: #334155;
      border-color: #94A3B8;
    }

    .badge-alert {
      background: #FEE2E2;
      color: #B91C1C;
      border-color: #F87171;
    }

    .badge-clamped {
      background: #0F2937;
      color: #38BDF8;
      border: 1px solid #0284C7;
      font-size: 0.7rem;
      padding: 0.15rem 0.5rem;
    }

    /* Serial Comms Indicator */
    .serial-indicator {
      display: inline-flex;
      align-items: center;
      gap: 0.55rem;
      padding: 0.35rem 0.65rem;
      background: var(--bg-surface);
      border: 1px solid var(--border-main);
      border-radius: var(--radius-sharp);
      font-family: var(--font-display);
      font-size: 0.76rem;
      font-weight: 700;
      letter-spacing: 0.05em;
      text-transform: uppercase;
      color: var(--text-main);
      user-select: none;
    }

    .serial-led {
      width: 14px;
      height: 14px;
      border-radius: 50%;
      display: inline-block;
      flex-shrink: 0;
      position: relative;
      box-sizing: border-box;
      transition: background 0.25s ease, box-shadow 0.25s ease, border-color 0.25s ease;
    }

    /* Specular curved reflection */
    .serial-led::after {
      content: '';
      position: absolute;
      top: 2px;
      left: 2.5px;
      width: 5px;
      height: 3px;
      background: rgba(255, 255, 255, 0.75);
      border-radius: 50% 50% 40% 40%;
      transform: rotate(-25deg);
      pointer-events: none;
    }

    /* Red when not connected or error */
    .serial-led.led-red {
      background: radial-gradient(circle at 35% 35%, #F87171 0%, #DC2626 55%, #7F1D1D 100%);
      border: 1.5px solid #EF4444;
      box-shadow: 0 0 7px rgba(239, 68, 68, 0.8), inset 0 1px 2px rgba(255, 255, 255, 0.4);
    }

    /* Dark green when serial connected */
    .serial-led.led-dark-green {
      background: radial-gradient(circle at 35% 35%, #059669 0%, #047857 55%, #022c22 100%);
      border: 1.5px solid #10B981;
      box-shadow: 0 0 4px rgba(16, 185, 129, 0.45), inset 0 1px 2px rgba(255, 255, 255, 0.35);
    }

    /* Light green when bits fly / data received */
    .serial-led.led-flash {
      background: radial-gradient(circle at 35% 35%, #6EE7B7 0%, #10B981 45%, #065F46 100%);
      border: 1.5px solid #34D399;
      box-shadow: 0 0 12px #34D399, 0 0 4px #ECFDF5, inset 0 1px 3px rgba(255, 255, 255, 0.75);
    }

    /* OIDC User Badge */
    .user-badge {
      display: inline-flex;
      align-items: center;
      gap: 0.45rem;
      padding: 0.35rem 0.65rem;
      background: var(--bg-subtle);
      border: 1px solid var(--border-main);
      border-radius: var(--radius-sharp);
      font-family: var(--font-display);
      font-size: 0.8rem;
      font-weight: 600;
      color: var(--text-main);
    }

    .user-icon {
      width: 16px;
      height: 16px;
      color: var(--accent-primary);
      flex-shrink: 0;
    }

    .user-name {
      max-width: 140px;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .logout-btn {
      color: var(--text-muted);
      text-decoration: none;
      font-size: 0.82rem;
      font-weight: 700;
      padding: 0 0.2rem;
      border-radius: 2px;
      margin-left: 0.2rem;
      transition: color 0.15s ease;
    }

    .logout-btn:hover {
      color: #EF4444;
    }

    /* Buttons */
    .btn {
      font-family: var(--font-display);
      font-size: 0.9rem;
      font-weight: 600;
      letter-spacing: 0.05em;
      text-transform: uppercase;
      padding: 0.5rem 1rem;
      border-radius: var(--radius-sharp);
      border: 1px solid var(--border-strong);
      cursor: pointer;
      transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease;
      display: inline-flex;
      align-items: center;
      gap: 0.45rem;
      min-height: 40px;
    }

    .btn:focus-visible,
    input:focus-visible,
    select:focus-visible {
      outline: 3px solid var(--accent-cyan);
      outline-offset: 2px;
    }

    .btn-primary {
      background: var(--accent-primary);
      color: #FFFFFF;
      border-color: var(--accent-primary);
    }

    .btn-primary:hover {
      background: var(--accent-hover);
    }

    .btn-danger {
      background: #DC2626;
      color: #FFFFFF;
      border-color: #B91C1C;
    }

    .btn-danger:hover {
      background: #B91C1C;
    }

    .btn-dark {
      background: var(--text-main);
      color: #FFFFFF;
      border-color: var(--text-main);
    }

    .btn-dark:hover {
      background: #1E3A4C;
    }

    .btn-outline {
      background: var(--bg-surface);
      color: var(--accent-primary);
      border-color: var(--border-main);
    }

    .btn-outline:hover {
      background: var(--bg-subtle);
      border-color: var(--accent-primary);
    }

    .hamburger-btn {
      background: var(--bg-subtle);
      color: var(--text-main);
      border: 1px solid var(--border-strong);
      padding: 0.45rem 0.7rem;
      border-radius: var(--radius-sharp);
      cursor: pointer;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      min-height: 40px;
      min-width: 44px;
    }

    .hamburger-btn:hover {
      background: var(--accent-primary);
      color: #FFFFFF;
    }

    /* Single-entry Drawer */
    .drawer-backdrop {
      position: fixed;
      inset: 0;
      background: rgba(15, 41, 55, 0.42);
      z-index: 90;
      opacity: 0;
      pointer-events: none;
      transition: opacity 0.2s ease;
    }

    .drawer-backdrop.open {
      opacity: 1;
      pointer-events: auto;
    }

    .nav-drawer {
      position: fixed;
      top: 0;
      right: 0;
      width: min(18rem, 80vw);
      height: 100svh;
      background: var(--bg-surface);
      border-left: 2px solid var(--border-strong);
      z-index: 100;
      transform: translateX(100%);
      transition: transform 0.22s cubic-bezier(0.16, 1, 0.3, 1);
      display: flex;
      flex-direction: column;
      box-shadow: -8px 0 24px rgba(15, 41, 55, 0.14);
    }

    .nav-drawer.open {
      transform: translateX(0);
    }

    .drawer-header {
      padding: 1rem 1.25rem;
      background: var(--bg-subtle);
      border-bottom: 1px solid var(--border-main);
      display: flex;
      align-items: center;
      justify-content: space-between;
    }

    .drawer-title {
      font-family: var(--font-display);
      font-weight: 700;
      font-size: 1.05rem;
      letter-spacing: 0.06em;
      text-transform: uppercase;
      color: var(--text-main);
    }

    .drawer-nav {
      padding: 1rem;
      overflow-y: auto;
      flex: 1;
    }

    .drawer-nav ul {
      list-style: none;
    }

    .nav-item-btn {
      width: 100%;
      text-align: left;
      background: transparent;
      border: 1px solid transparent;
      border-radius: var(--radius-sharp);
      padding: 0.75rem 1rem;
      font-family: var(--font-display);
      font-size: 1.02rem;
      font-weight: 600;
      color: var(--text-main);
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 0.45rem;
    }

    .nav-item-btn:hover,
    .nav-item-btn.active {
      background: var(--bg-subtle);
      border-color: var(--border-main);
      color: var(--accent-primary);
    }

    /* Main Container */
    .main-container {
      max-width: 1320px;
      margin: 0 auto;
      padding: 1.5rem;
    }

    /* Timestamp & Status Bar */
    .timestamp-bar {
      background: var(--bg-surface);
      border: 1px solid var(--border-strong);
      border-left: 5px solid var(--accent-primary);
      border-radius: var(--radius-sharp);
      padding: 0.85rem 1.25rem;
      margin-bottom: 1.5rem;
      display: flex;
      flex-wrap: wrap;
      align-items: center;
      justify-content: space-between;
      gap: 1rem;
    }

    .ts-item {
      font-size: 0.92rem;
      color: var(--text-muted);
    }

    .ts-value {
      font-family: var(--font-mono);
      font-weight: 600;
      color: var(--text-main);
      margin-left: 0.35rem;
    }

    /* Alert Banner */
    .error-banner {
      background: #FEF2F2;
      border: 1px solid #EF4444;
      border-left: 5px solid #DC2626;
      border-radius: var(--radius-sharp);
      padding: 0.75rem 1.15rem;
      margin-bottom: 1.25rem;
      color: #991B1B;
      font-size: 0.9rem;
      display: none;
      align-items: center;
      justify-content: space-between;
      gap: 1rem;
    }

    /* Dashboard Grid */
    .telemetry-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(340px, 1fr));
      gap: 1.25rem;
    }

    .card {
      background: var(--bg-surface);
      border: 1px solid var(--border-main);
      border-top: 3px solid var(--accent-primary);
      border-radius: var(--radius-sharp);
      padding: 1.25rem;
      box-shadow: 0 2px 6px rgba(0, 139, 158, 0.05);
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      gap: 1rem;
    }

    .card-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      border-bottom: 1px solid var(--bg-subtle);
      padding-bottom: 0.65rem;
      gap: 0.5rem;
    }

    .card-title-group {
      display: flex;
      align-items: center;
      gap: 0.55rem;
    }

    .card-icon {
      width: 24px;
      height: 24px;
      color: var(--accent-primary);
      flex-shrink: 0;
    }

    .card-title {
      font-family: var(--font-display);
      font-size: 1.08rem;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      color: var(--text-main);
    }

    .metric-row {
      display: flex;
      align-items: flex-end;
      justify-content: space-between;
      gap: 0.75rem;
      padding: 0.35rem 0;
      min-height: 54px;
    }

    .metric-row-sm {
      display: flex;
      align-items: flex-end;
      justify-content: space-between;
      gap: 0.75rem;
      padding: 0.35rem 0;
      min-height: 44px;
    }

    .metric-label {
      font-size: 0.88rem;
      color: var(--text-muted);
      font-weight: 500;
    }

    .metric-latest {
      font-family: var(--font-display);
      font-size: 2rem;
      font-weight: 700;
      font-variant-numeric: tabular-nums;
      color: var(--text-main);
      line-height: 1.1;
    }

    .metric-latest-sm {
      font-family: var(--font-display);
      font-size: 1.45rem;
      font-weight: 700;
      font-variant-numeric: tabular-nums;
      color: var(--text-main);
    }

    .metric-unit {
      font-size: 0.95rem;
      font-weight: 600;
      color: var(--accent-primary);
      margin-left: 0.2rem;
    }

    .metric-prev {
      font-family: var(--font-mono);
      font-size: 0.8rem;
      font-variant-numeric: tabular-nums;
      color: var(--text-muted);
      background: var(--bg-subtle);
      padding: 0.2rem 0.5rem;
      border-radius: var(--radius-sharp);
      border: 1px solid var(--border-main);
    }

    .compass-wrap {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 1rem;
    }

    .compass-svg {
      width: 84px;
      height: 84px;
      flex-shrink: 0;
    }

    /* Views */
    .view-section {
      display: none;
    }

    .view-section.active {
      display: block;
    }

    /* Settings View with Header Actions & Tabs */
    .settings-header-bar {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 1.25rem;
      flex-wrap: wrap;
      gap: 1rem;
      background: var(--bg-surface);
      border: 1px solid var(--border-main);
      border-left: 5px solid var(--accent-primary);
      border-radius: var(--radius-sharp);
      padding: 1rem 1.25rem;
    }

    .settings-title-group h1 {
      font-family: var(--font-display);
      font-size: 1.4rem;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      color: var(--text-main);
    }

    .settings-title-group p {
      color: var(--text-muted);
      font-size: 0.88rem;
    }

    .settings-top-actions {
      display: flex;
      align-items: center;
      gap: 0.65rem;
      flex-wrap: wrap;
    }

    /* Tabs Navigation */
    .settings-tabs-nav {
      display: flex;
      gap: 0.35rem;
      margin-bottom: 1.25rem;
      border-bottom: 2px solid var(--border-main);
      overflow-x: auto;
      scrollbar-width: none;
    }

    .tab-btn {
      font-family: var(--font-display);
      font-size: 0.92rem;
      font-weight: 600;
      letter-spacing: 0.04em;
      text-transform: uppercase;
      padding: 0.65rem 1rem;
      background: transparent;
      border: 1px solid transparent;
      border-bottom: none;
      border-top-left-radius: var(--radius-sharp);
      border-top-right-radius: var(--radius-sharp);
      color: var(--text-muted);
      cursor: pointer;
      white-space: nowrap;
      transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease;
      margin-bottom: -2px;
    }

    .tab-btn:hover {
      background: var(--bg-subtle);
      color: var(--text-main);
    }

    .tab-btn.active {
      background: var(--bg-surface);
      border-color: var(--border-main) var(--border-main) transparent var(--border-main);
      border-top: 3px solid var(--accent-primary);
      border-bottom: 2px solid var(--bg-surface);
      color: var(--accent-primary);
      font-weight: 700;
    }

    /* Tab Panels */
    .tab-panel {
      display: none;
      background: var(--bg-surface);
      border: 1px solid var(--border-main);
      border-radius: var(--radius-sharp);
      padding: 1.5rem;
      box-shadow: 0 2px 6px rgba(0, 139, 158, 0.05);
      margin-bottom: 1.5rem;
    }

    .tab-panel.active {
      display: block;
    }

    .form-group {
      margin-bottom: 1rem;
    }

    .form-row-2 {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 1rem;
    }

    label.field-label {
      display: block;
      font-size: 0.86rem;
      font-weight: 600;
      color: var(--text-main);
      margin-bottom: 0.35rem;
    }

    input[type="text"],
    input[type="password"],
    input[type="number"],
    input[type="url"],
    input[type="email"],
    select {
      width: 100%;
      font-family: var(--font-mono);
      font-size: 0.92rem;
      padding: 0.55rem 0.75rem;
      min-height: 42px;
      background: #F9FEFF;
      border: 1px solid var(--border-main);
      border-radius: var(--radius-sharp);
      color: var(--text-main);
    }

    .checkbox-row {
      display: flex;
      align-items: center;
      gap: 0.6rem;
      margin: 0.65rem 0;
      cursor: pointer;
    }

    .checkbox-row input[type="checkbox"] {
      width: 18px;
      height: 18px;
      accent-color: var(--accent-primary);
      cursor: pointer;
    }

    .bounds-table {
      width: 100%;
      border-collapse: collapse;
      margin-top: 0.75rem;
      font-size: 0.9rem;
    }

    .bounds-table th,
    .bounds-table td {
      border: 1px solid var(--border-main);
      padding: 0.55rem 0.75rem;
      text-align: left;
    }

    .bounds-table th {
      background: var(--bg-subtle);
      font-family: var(--font-display);
      text-transform: uppercase;
      font-size: 0.8rem;
      letter-spacing: 0.04em;
    }

    .bounds-table input[type="number"] {
      min-height: 36px;
      padding: 0.3rem 0.5rem;
    }

    .toast {
      position: fixed;
      bottom: 1.5rem;
      right: 1.5rem;
      background: var(--text-main);
      color: #FFFFFF;
      border-left: 4px solid var(--accent-cyan);
      padding: 0.85rem 1.25rem;
      border-radius: var(--radius-sharp);
      font-family: var(--font-body);
      font-size: 0.9rem;
      box-shadow: 0 6px 20px rgba(0, 0, 0, 0.18);
      z-index: 120;
      display: none;
    }

    @media (max-width: 640px) {
      .form-row-2 {
        grid-template-columns: 1fr;
      }
      .telemetry-grid {
        grid-template-columns: 1fr;
      }
    }
  </style>
</head>
<body>

  <header class="top-header">
    <a href="#dashboard" class="brand-group" id="brand-link">
      <svg class="brand-logo" aria-hidden="true" viewBox="0 0 64 64">
        <rect x="2" y="2" width="60" height="60" rx="4" fill="#0F2937" stroke="#008B9E" stroke-width="2.5"/>
        <circle cx="24" cy="24" r="10" fill="#F59E0B"/>
        <path d="M24 9V12M24 36V39M9 24H12M36 24H39M13.4 13.4L15.5 15.5M32.5 32.5L34.6 34.6M34.6 13.4L32.5 15.5M15.5 32.5L13.4 34.6" stroke="#F59E0B" stroke-width="2.2" stroke-linecap="square"/>
        <path d="M10 42H46C49.5 42 52 39.5 52 36.5C52 33.5 49.5 31 46.5 31C44 31 42 32.8 41.5 35" fill="none" stroke="#06B6D4" stroke-width="3" stroke-linecap="square"/>
        <path d="M10 49H50" fill="none" stroke="#008B9E" stroke-width="3" stroke-linecap="square"/>
        <path d="M10 55H38C41 55 43 57 43 59" fill="none" stroke="#38BDF8" stroke-width="2.5" stroke-linecap="square"/>
      </svg>
      <div>
        <span class="brand-title">WS2300</span>
        <span class="brand-subtitle" id="header-port-label">/dev/ttyUSB0 · 2400 BAUD</span>
      </div>
    </a>

    <div class="header-controls">
      <div class="serial-indicator" id="serial-indicator" title="Serial Data Port Status">
        <span>SERIAL DATA</span>
        <span class="serial-led led-red" id="serial-led" aria-hidden="true"></span>
      </div>

      <span id="service-status-badge" class="badge badge-running">
        <span id="service-status-text">SERVICE RUNNING</span>
      </span>

      <button type="button" id="btn-toggle-service" class="btn btn-danger" style="display: none;">Stop</button>

      <div id="oidc-user-badge" class="user-badge" style="display: none;" title="Authenticated via OIDC">
        <svg class="user-icon" aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
          <circle cx="12" cy="7" r="4"/>
        </svg>
        <span id="oidc-user-name" class="user-name"></span>
        <a href="/auth/logout" class="logout-btn" title="Log out">✕</a>
      </div>

      <button type="button" id="drawer-open-btn" class="hamburger-btn" aria-label="Open Navigation Menu" aria-expanded="false" aria-controls="nav-drawer">
        <svg aria-hidden="true" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="square">
          <path d="M3 6h18M3 12h18M3 18h18"/>
        </svg>
      </button>
    </div>
  </header>

  <div id="drawer-backdrop" class="drawer-backdrop"></div>
  <aside id="nav-drawer" class="nav-drawer" aria-label="Site Navigation" tabindex="-1">
    <div class="drawer-header">
      <span class="drawer-title">Navigation</span>
      <button type="button" id="drawer-close-btn" class="btn btn-outline" style="min-height:34px;padding:0.25rem 0.65rem;">Close ✕</button>
    </div>
    <nav class="drawer-nav">
      <ul>
        <li>
          <button type="button" class="nav-item-btn active" id="nav-btn-dashboard">
            <span>Dashboard</span>
            <span>▸</span>
          </button>
        </li>
        <li id="drawer-settings-item">
          <button type="button" class="nav-item-btn" id="nav-btn-settings">
            <span>Settings</span>
            <span>▸</span>
          </button>
        </li>
        <li id="drawer-auth-item" style="display: none; margin-top: 1rem; border-top: 1px solid var(--border-main); padding-top: 0.5rem;">
          <a id="drawer-auth-link" href="/auth/login" class="nav-item-btn" style="text-decoration: none;">
            <span id="drawer-auth-label">Log In</span>
            <span id="drawer-auth-icon">→</span>
          </a>
        </li>
      </ul>
    </nav>
  </aside>

  <main class="main-container" id="main-content">
    <div id="error-banner" class="error-banner" role="alert" aria-live="polite">
      <span id="error-banner-text"></span>
    </div>

    <!-- DASHBOARD VIEW -->
    <section id="view-dashboard" class="view-section active">
      <div class="timestamp-bar">
        <div class="ts-item">
          Latest Reading:
          <span class="ts-value" id="ts-latest">Waiting for station poll...</span>
        </div>
        <div class="ts-item">
          Previous Reading:
          <span class="ts-value" id="ts-previous">—</span>
        </div>
        <div class="ts-item">
          MQTT Status:
          <span class="ts-value" id="mqtt-status-label">Disabled</span>
        </div>
      </div>

      <div class="telemetry-grid">
        <!-- Tile 1: Indoor (FIRST) -->
        <article class="card" id="card-indoor">
          <div class="card-header">
            <div class="card-title-group">
              <svg class="card-icon" aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M3 10.5L12 3l9 7.5V21H3V10.5z"/>
                <path d="M9 21V13h6v8"/>
              </svg>
              <h2 class="card-title">Indoor</h2>
            </div>
            <span id="clamp-indoor" class="badge badge-clamped" style="display:none;">Out-of-bounds clamped to previous valid</span>
          </div>

          <div>
            <div class="metric-row">
              <div>
                <div class="metric-label">Temperature</div>
                <div class="metric-latest"><span id="val-temp-in">—</span><span class="metric-unit">°C</span></div>
              </div>
              <div class="metric-prev">Prev: <span id="prev-temp-in">—</span> °C</div>
            </div>

            <div class="metric-row-sm" style="margin-top:0.4rem;">
              <div>
                <div class="metric-label">Humidity</div>
                <div class="metric-latest-sm"><span id="val-hum-in">—</span><span class="metric-unit">%</span></div>
              </div>
              <div class="metric-prev">Prev: <span id="prev-hum-in">—</span> %</div>
            </div>

            <div class="metric-row-sm" style="margin-top:0.4rem;visibility:hidden;" aria-hidden="true">
              <div>
                <div class="metric-label">Placeholder</div>
                <div class="metric-latest-sm">0.0<span class="metric-unit">°C</span></div>
              </div>
              <div class="metric-prev">Prev: 0.0 °C</div>
            </div>
          </div>
        </article>

        <!-- Tile 2: Outdoor (SECOND) -->
        <article class="card" id="card-outdoor">
          <div class="card-header">
            <div class="card-title-group">
              <svg class="card-icon" aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 2L6 8h3l-4 5h3l-5 6h9v3h2v-3h9l-5-6h3l-4-5h3z"/>
              </svg>
              <h2 class="card-title">Outdoor</h2>
            </div>
            <span id="clamp-outdoor" class="badge badge-clamped" style="display:none;">Out-of-bounds clamped to previous valid</span>
          </div>

          <div>
            <div class="metric-row">
              <div>
                <div class="metric-label">Temperature</div>
                <div class="metric-latest"><span id="val-temp-out">—</span><span class="metric-unit">°C</span></div>
              </div>
              <div class="metric-prev">Prev: <span id="prev-temp-out">—</span> °C</div>
            </div>

            <div class="metric-row-sm" style="margin-top:0.4rem;">
              <div>
                <div class="metric-label">Humidity</div>
                <div class="metric-latest-sm"><span id="val-hum-out">—</span><span class="metric-unit">%</span></div>
              </div>
              <div class="metric-prev">Prev: <span id="prev-hum-out">—</span> %</div>
            </div>

            <div class="metric-row-sm" style="margin-top:0.4rem;">
              <div>
                <div class="metric-label">Dewpoint</div>
                <div class="metric-latest-sm"><span id="val-dewpoint">—</span><span class="metric-unit">°C</span></div>
              </div>
              <div class="metric-prev">Prev: <span id="prev-dewpoint">—</span> °C</div>
            </div>
          </div>
        </article>

        <!-- Tile 3: Pressure & Forecast -->
        <article class="card" id="card-pressure">
          <div class="card-header">
            <div class="card-title-group">
              <svg class="card-icon" aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="9"/>
                <path d="M12 12l4-4M12 7v1M17 12h-1M7 12h1"/>
              </svg>
              <h2 class="card-title">Pressure &amp; Forecast</h2>
            </div>
            <span id="clamp-pressure" class="badge badge-clamped" style="display:none;">Out-of-bounds clamped to previous valid</span>
          </div>

          <div>
            <div class="metric-row">
              <div>
                <div class="metric-label">Pressure</div>
                <div class="metric-latest"><span id="val-pressure">—</span><span class="metric-unit">hPa</span></div>
              </div>
              <div class="metric-prev">Prev: <span id="prev-pressure">—</span> hPa</div>
            </div>

            <div class="metric-row" style="margin-top:0.4rem;">
              <div>
                <div class="metric-label">Tendency</div>
                <div class="metric-latest-sm" id="val-tendency">—</div>
              </div>
              <div class="metric-prev">Prev: <span id="prev-tendency">—</span></div>
            </div>

            <div class="metric-row">
              <div>
                <div class="metric-label">Forecast</div>
                <div class="metric-latest-sm" id="val-forecast">—</div>
              </div>
              <div class="metric-prev">Prev: <span id="prev-forecast">—</span></div>
            </div>
          </div>
        </article>

        <!-- Tile 4: Wind (TELEMETRY removed) -->
        <article class="card" id="card-wind">
          <div class="card-header">
            <div class="card-title-group">
              <svg class="card-icon" aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M9.59 4.59A2 2 0 1 1 11 8H2m10.59 11.41A2 2 0 1 0 14 16H2m15.73-8.27A2.5 2.5 0 1 1 19.5 12H2"/>
              </svg>
              <h2 class="card-title">Wind</h2>
            </div>
            <span id="clamp-wind" class="badge badge-clamped" style="display:none;">Out-of-bounds clamped to previous valid</span>
          </div>

          <div class="compass-wrap">
            <div style="flex:1;">
              <div class="metric-row">
                <div>
                  <div class="metric-label">Speed</div>
                  <div class="metric-latest"><span id="val-wind-speed">—</span><span class="metric-unit">m/s</span></div>
                </div>
                <div class="metric-prev">Prev: <span id="prev-wind-speed">—</span> m/s</div>
              </div>

              <div class="metric-row">
                <div>
                  <div class="metric-label">Direction</div>
                  <div class="metric-latest-sm"><span id="val-wind-dir-str">—</span> (<span id="val-wind-dir-deg">—</span>°)</div>
                </div>
                <div class="metric-prev">Prev: <span id="prev-wind-dir-str">—</span></div>
              </div>

              <div class="metric-row">
                <div>
                  <div class="metric-label">Wind Chill</div>
                  <div class="metric-latest-sm"><span id="val-wind-chill">—</span><span class="metric-unit">°C</span></div>
                </div>
                <div class="metric-prev">Prev: <span id="prev-wind-chill">—</span> °C</div>
              </div>
            </div>

            <svg class="compass-svg" aria-hidden="true" viewBox="0 0 100 100">
              <circle cx="50" cy="50" r="42" fill="#F2FBFD" stroke="#008B9E" stroke-width="2.5"/>
              <text x="50" y="19" text-anchor="middle" font-size="10" font-weight="700" fill="#0F2937">N</text>
              <text x="84" y="53" text-anchor="middle" font-size="10" font-weight="700" fill="#0F2937">E</text>
              <text x="50" y="88" text-anchor="middle" font-size="10" font-weight="700" fill="#0F2937">S</text>
              <text x="16" y="53" text-anchor="middle" font-size="10" font-weight="700" fill="#0F2937">W</text>
              <g id="compass-needle" transform="rotate(0 50 50)">
                <polygon points="50,20 56,50 50,45 44,50" fill="#008B9E"/>
                <polygon points="50,80 56,50 50,55 44,50" fill="#94A3B8"/>
              </g>
            </svg>
          </div>
        </article>

        <!-- Tile 5: Rainfall (ACCUMULATION removed) -->
        <article class="card" id="card-rain">
          <div class="card-header">
            <div class="card-title-group">
              <svg class="card-icon" aria-hidden="true" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M20 16.2A4.5 4.5 0 0 0 17.5 8h-1.8A7 7 0 1 0 4 14.9"/>
                <path d="M8 19v3M12 17v5M16 19v3"/>
              </svg>
              <h2 class="card-title">Rainfall</h2>
            </div>
            <span id="clamp-rain" class="badge badge-clamped" style="display:none;">Out-of-bounds clamped to previous valid</span>
          </div>

          <div>
            <div class="metric-row">
              <div>
                <div class="metric-label">Last 1 Hour</div>
                <div class="metric-latest"><span id="val-rain-1h">—</span><span class="metric-unit">mm</span></div>
              </div>
              <div class="metric-prev">Prev: <span id="prev-rain-1h">—</span> mm</div>
            </div>

            <div class="metric-row">
              <div>
                <div class="metric-label">Last 24 Hours</div>
                <div class="metric-latest-sm"><span id="val-rain-24h">—</span><span class="metric-unit">mm</span></div>
              </div>
              <div class="metric-prev">Prev: <span id="prev-rain-24h">—</span> mm</div>
            </div>

            <div class="metric-row">
              <div>
                <div class="metric-label">Total Cumulative</div>
                <div class="metric-latest-sm"><span id="val-rain-total">—</span><span class="metric-unit">mm</span></div>
              </div>
              <div class="metric-prev">Prev: <span id="prev-rain-total">—</span> mm</div>
            </div>
          </div>
        </article>
      </div>
    </section>

    <!-- SETTINGS VIEW WITH TOP TABS & TOP ACTIONS -->
    <section id="view-settings" class="view-section">
      <form id="settings-form" action="/api/config" method="POST">
        <!-- Top Title & Action Bar -->
        <div class="settings-header-bar">
          <div class="settings-title-group">
            <h1>Settings</h1>
            <p>Persisted atomically to <code>config.yaml</code></p>
          </div>

          <div class="settings-top-actions">
            <button type="button" class="btn btn-outline" id="btn-reload-cfg">Reload from Disk</button>
            <button type="submit" class="btn btn-primary" id="btn-save-cfg">Save to config.yaml</button>
            <button type="button" class="btn btn-outline" id="btn-back-dashboard">← Back to Dashboard</button>
          </div>
        </div>

        <!-- Horizontal Settings Tabs -->
        <div class="settings-tabs-nav" role="tablist">
          <button type="button" class="tab-btn active" role="tab" aria-selected="true" data-tab-target="tab-serial">Serial Port &amp; Metrics</button>
          <button type="button" class="tab-btn" role="tab" aria-selected="false" data-tab-target="tab-mqtt">MQTT Server &amp; Topic</button>
          <button type="button" class="tab-btn" role="tab" aria-selected="false" data-tab-target="tab-alerts">Stale Data Alerts</button>
          <button type="button" class="tab-btn" role="tab" aria-selected="false" data-tab-target="tab-oidc">OIDC Authentication</button>
          <button type="button" class="tab-btn" role="tab" aria-selected="false" data-tab-target="tab-bounds">Out-of-Bounds Filters</button>
        </div>

        <!-- Tab 1: Serial Port & Metrics -->
        <div class="tab-panel active" id="tab-serial" role="tabpanel">
          <div class="form-group">
            <label class="field-label" for="cfg-serial-port">Serial Port</label>
            <div style="display:flex;gap:0.5rem;">
              <input type="text" id="cfg-serial-port" name="serial_port" list="serial-ports-datalist" autocorrect="off" autocapitalize="off" spellcheck="false" required>
              <datalist id="serial-ports-datalist"></datalist>
              <button type="button" class="btn btn-outline" id="btn-scan-ports">Scan Ports</button>
            </div>
          </div>

          <div class="form-row-2">
            <div class="form-group">
              <label class="field-label" for="cfg-poll-interval">Polling Interval (seconds)</label>
              <input type="number" id="cfg-poll-interval" name="poll_interval" min="2" max="86400" required>
            </div>
            <div class="form-group">
              <label class="field-label" for="cfg-bind-addr">Web UI Bind Address</label>
              <input type="text" id="cfg-bind-addr" name="bind_address" autocorrect="off" autocapitalize="off" spellcheck="false" required>
            </div>
          </div>

          <label class="checkbox-row" for="cfg-auto-start">
            <input type="checkbox" id="cfg-auto-start" name="auto_start">
            <span>Auto-start polling service on boot</span>
          </label>

          <label class="checkbox-row" for="cfg-demo-mode">
            <input type="checkbox" id="cfg-demo-mode" name="demo_mode">
            <span>Simulate WS2300 readings (Demo Mode when hardware disconnected)</span>
          </label>

          <hr style="border:0;border-top:1px solid var(--border-main);margin:1.25rem 0;">

          <div style="font-family:var(--font-display);font-weight:700;font-size:0.95rem;text-transform:uppercase;color:var(--accent-primary);margin-bottom:0.45rem;">
            Metric Inclusion (UI &amp; MQTT)
          </div>

          <label class="checkbox-row" for="cfg-include-wind">
            <input type="checkbox" id="cfg-include-wind" name="include_wind">
            <span>Include <strong>WIND</strong> metrics on Dashboard and send to MQTT</span>
          </label>

          <label class="checkbox-row" for="cfg-include-rain">
            <input type="checkbox" id="cfg-include-rain" name="include_rain">
            <span>Include <strong>RAIN</strong> metrics on Dashboard and send to MQTT</span>
          </label>
        </div>

        <!-- Tab 2: MQTT -->
        <div class="tab-panel" id="tab-mqtt" role="tabpanel">
          <label class="checkbox-row" for="cfg-mqtt-enabled">
            <input type="checkbox" id="cfg-mqtt-enabled" name="mqtt_enabled">
            <span>Enable MQTT Publishing</span>
          </label>

          <div class="form-row-2">
            <div class="form-group">
              <label class="field-label" for="cfg-mqtt-host">Broker Host / IP</label>
              <input type="text" id="cfg-mqtt-host" name="mqtt_host" autocorrect="off" autocapitalize="off" spellcheck="false">
            </div>
            <div class="form-group">
              <label class="field-label" for="cfg-mqtt-port">Broker Port</label>
              <input type="number" id="cfg-mqtt-port" name="mqtt_port" min="1" max="65535">
            </div>
          </div>

          <div class="form-row-2">
            <div class="form-group">
              <label class="field-label" for="cfg-mqtt-user">Username (optional)</label>
              <input type="text" id="cfg-mqtt-user" name="mqtt_user" autocomplete="username" autocorrect="off" autocapitalize="off" spellcheck="false">
            </div>
            <div class="form-group">
              <label class="field-label" for="cfg-mqtt-pass">Password (optional)</label>
              <input type="password" id="cfg-mqtt-pass" name="mqtt_pass" autocomplete="current-password">
            </div>
          </div>

          <div class="form-row-2">
            <div class="form-group">
              <label class="field-label" for="cfg-mqtt-topic">MQTT Topic</label>
              <input type="text" id="cfg-mqtt-topic" name="mqtt_topic" autocorrect="off" autocapitalize="off" spellcheck="false">
            </div>
            <div class="form-group">
              <label class="field-label" for="cfg-mqtt-client">Client ID</label>
              <input type="text" id="cfg-mqtt-client" name="mqtt_client" autocorrect="off" autocapitalize="off" spellcheck="false">
            </div>
          </div>

          <label class="checkbox-row" for="cfg-mqtt-retain">
            <input type="checkbox" id="cfg-mqtt-retain" name="mqtt_retain">
            <span>Retain latest published MQTT message</span>
          </label>
        </div>

        <!-- Tab 3: Stale Data Alerts -->
        <div class="tab-panel" id="tab-alerts" role="tabpanel">
          <div style="display:flex;align-items:center;justify-content:space-between;flex-wrap:wrap;gap:0.5rem;margin-bottom:0.75rem;">
            <label class="checkbox-row" for="cfg-alerts-enabled">
              <input type="checkbox" id="cfg-alerts-enabled" name="alerts_enabled">
              <span>Enable Stale Telemetry Watchdog</span>
            </label>
            <button type="button" class="btn btn-outline" id="btn-test-alert" style="min-height:34px;padding:0.25rem 0.75rem;font-size:0.82rem;">Send Test Alert</button>
          </div>

          <div class="form-group">
            <label class="field-label" for="cfg-alert-minutes">Alert when no values received for (minutes)</label>
            <input type="number" id="cfg-alert-minutes" name="alert_minutes" min="1" max="10080">
          </div>

          <hr style="border:0;border-top:1px solid var(--border-main);margin:1.25rem 0;">

          <label class="checkbox-row" for="cfg-email-enabled">
            <input type="checkbox" id="cfg-email-enabled" name="email_enabled">
            <span><strong>Email (SMTP)</strong> Notifications</span>
          </label>
          <div class="form-row-2">
            <div class="form-group">
              <label class="field-label" for="cfg-smtp-host">SMTP Host</label>
              <input type="text" id="cfg-smtp-host" name="smtp_host" autocorrect="off" autocapitalize="off" spellcheck="false">
            </div>
            <div class="form-group">
              <label class="field-label" for="cfg-smtp-security">Security / Encryption</label>
              <select id="cfg-smtp-security" name="smtp_security">
                <option value="starttls">STARTTLS (Port 587)</option>
                <option value="tls">TLS / SSL (Port 465)</option>
                <option value="plain">Plain / None (Port 25)</option>
              </select>
            </div>
          </div>
          <div class="form-row-2">
            <div class="form-group">
              <label class="field-label" for="cfg-smtp-port">SMTP Port</label>
              <input type="number" id="cfg-smtp-port" name="smtp_port">
            </div>
            <div class="form-group">
              <label class="field-label" for="cfg-smtp-user">SMTP Username</label>
              <input type="text" id="cfg-smtp-user" name="smtp_user" autocorrect="off" autocapitalize="off" spellcheck="false">
            </div>
          </div>
          <div class="form-row-2">
            <div class="form-group">
              <label class="field-label" for="cfg-smtp-pass">SMTP Password</label>
              <input type="password" id="cfg-smtp-pass" name="smtp_pass">
            </div>
            <div class="form-group">
              <label class="field-label" for="cfg-smtp-from">From Email</label>
              <input type="email" id="cfg-smtp-from" name="smtp_from" autocorrect="off" autocapitalize="off" spellcheck="false">
            </div>
          </div>
          <div class="form-group">
            <label class="field-label" for="cfg-smtp-to">To Email</label>
            <input type="email" id="cfg-smtp-to" name="smtp_to" autocorrect="off" autocapitalize="off" spellcheck="false">
          </div>

          <hr style="border:0;border-top:1px solid var(--border-main);margin:1.25rem 0;">

          <label class="checkbox-row" for="cfg-tg-enabled">
            <input type="checkbox" id="cfg-tg-enabled" name="tg_enabled">
            <span><strong>Telegram Bot</strong> Notifications</span>
          </label>
          <div class="form-row-2">
            <div class="form-group">
              <label class="field-label" for="cfg-tg-token">Telegram Bot Token</label>
              <input type="password" id="cfg-tg-token" name="tg_token" autocorrect="off" autocapitalize="off" spellcheck="false">
            </div>
            <div class="form-group">
              <label class="field-label" for="cfg-tg-chat">Telegram Chat ID</label>
              <input type="text" id="cfg-tg-chat" name="tg_chat" autocorrect="off" autocapitalize="off" spellcheck="false">
            </div>
          </div>

          <hr style="border:0;border-top:1px solid var(--border-main);margin:1.25rem 0;">

          <label class="checkbox-row" for="cfg-webhook-enabled">
            <input type="checkbox" id="cfg-webhook-enabled" name="webhook_enabled">
            <span><strong>Webhook / Apprise / Ntfy / Slack</strong> URL</span>
          </label>
          <div class="form-group">
            <label class="field-label" for="cfg-webhook-url">Target Webhook URL</label>
            <input type="url" id="cfg-webhook-url" name="webhook_url" placeholder="https://ntfy.sh/ws2300-alerts" autocorrect="off" autocapitalize="off" spellcheck="false">
          </div>
        </div>

        <!-- Tab 4: OIDC -->
        <div class="tab-panel" id="tab-oidc" role="tabpanel">
          <p style="font-size:0.88rem;color:var(--text-muted);margin-bottom:1rem;">
            Identity authentication only — no complex permissions or role management required.
          </p>

          <label class="checkbox-row" for="cfg-oidc-enabled">
            <input type="checkbox" id="cfg-oidc-enabled" name="oidc_enabled">
            <span>Require OIDC Login to access UI &amp; API</span>
          </label>

          <div class="form-group">
            <label class="field-label" for="cfg-oidc-issuer">Issuer URL</label>
            <input type="url" id="cfg-oidc-issuer" name="oidc_issuer" autocorrect="off" autocapitalize="off" spellcheck="false">
          </div>

          <div class="form-row-2">
            <div class="form-group">
              <label class="field-label" for="cfg-oidc-client-id">Client ID</label>
              <input type="text" id="cfg-oidc-client-id" name="oidc_client_id" autocorrect="off" autocapitalize="off" spellcheck="false">
            </div>
            <div class="form-group">
              <label class="field-label" for="cfg-oidc-secret">Client Secret</label>
              <input type="password" id="cfg-oidc-secret" name="oidc_secret">
            </div>
          </div>

          <div class="form-group">
            <label class="field-label" for="cfg-oidc-redirect">Redirect Callback URL</label>
            <input type="url" id="cfg-oidc-redirect" name="oidc_redirect" autocorrect="off" autocapitalize="off" spellcheck="false">
          </div>
        </div>

        <!-- Tab 5: Out-of-Bounds Filters -->
        <div class="tab-panel" id="tab-bounds" role="tabpanel">
          <label class="checkbox-row" for="cfg-bounds-replace">
            <input type="checkbox" id="cfg-bounds-replace" name="bounds_replace">
            <span>Replace out-of-bounds sensor values with previous valid reading during processing</span>
          </label>

          <div style="overflow-x:auto;">
            <table class="bounds-table">
              <thead>
                <tr>
                  <th>Metric</th>
                  <th>Minimum Valid Value</th>
                  <th>Maximum Valid Value</th>
                  <th>Unit</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <td>Indoor Temperature</td>
                  <td><input type="number" step="0.1" id="b-ti-min" aria-label="Indoor Temperature Min"></td>
                  <td><input type="number" step="0.1" id="b-ti-max" aria-label="Indoor Temperature Max"></td>
                  <td>°C</td>
                </tr>
                <tr>
                  <td>Outdoor Temperature</td>
                  <td><input type="number" step="0.1" id="b-to-min" aria-label="Outdoor Temperature Min"></td>
                  <td><input type="number" step="0.1" id="b-to-max" aria-label="Outdoor Temperature Max"></td>
                  <td>°C</td>
                </tr>
                <tr>
                  <td>Dewpoint</td>
                  <td><input type="number" step="0.1" id="b-dp-min" aria-label="Dewpoint Min"></td>
                  <td><input type="number" step="0.1" id="b-dp-max" aria-label="Dewpoint Max"></td>
                  <td>°C</td>
                </tr>
                <tr>
                  <td>Indoor Humidity</td>
                  <td><input type="number" step="1" id="b-hi-min" aria-label="Indoor Humidity Min"></td>
                  <td><input type="number" step="1" id="b-hi-max" aria-label="Indoor Humidity Max"></td>
                  <td>%</td>
                </tr>
                <tr>
                  <td>Outdoor Humidity</td>
                  <td><input type="number" step="1" id="b-ho-min" aria-label="Outdoor Humidity Min"></td>
                  <td><input type="number" step="1" id="b-ho-max" aria-label="Outdoor Humidity Max"></td>
                  <td>%</td>
                </tr>
                <tr>
                  <td>Barometric Pressure</td>
                  <td><input type="number" step="0.1" id="b-pr-min" aria-label="Pressure Min"></td>
                  <td><input type="number" step="0.1" id="b-pr-max" aria-label="Pressure Max"></td>
                  <td>hPa</td>
                </tr>
                <tr>
                  <td>Wind Speed</td>
                  <td><input type="number" step="0.1" id="b-ws-min" aria-label="Wind Speed Min"></td>
                  <td><input type="number" step="0.1" id="b-ws-max" aria-label="Wind Speed Max"></td>
                  <td>m/s</td>
                </tr>
                <tr>
                  <td>Wind Direction</td>
                  <td><input type="number" step="0.1" id="b-wd-min" aria-label="Wind Direction Min"></td>
                  <td><input type="number" step="0.1" id="b-wd-max" aria-label="Wind Direction Max"></td>
                  <td>deg (°)</td>
                </tr>
                <tr>
                  <td>Wind Chill</td>
                  <td><input type="number" step="0.1" id="b-wc-min" aria-label="Wind Chill Min"></td>
                  <td><input type="number" step="0.1" id="b-wc-max" aria-label="Wind Chill Max"></td>
                  <td>°C</td>
                </tr>
                <tr>
                  <td>Rain 1h</td>
                  <td><input type="number" step="0.1" id="b-r1-min" aria-label="Rain 1h Min"></td>
                  <td><input type="number" step="0.1" id="b-r1-max" aria-label="Rain 1h Max"></td>
                  <td>mm</td>
                </tr>
                <tr>
                  <td>Rain 24h</td>
                  <td><input type="number" step="0.1" id="b-r24-min" aria-label="Rain 24h Min"></td>
                  <td><input type="number" step="0.1" id="b-r24-max" aria-label="Rain 24h Max"></td>
                  <td>mm</td>
                </tr>
                <tr>
                  <td>Rain Total</td>
                  <td><input type="number" step="0.1" id="b-rt-min" aria-label="Rain Total Min"></td>
                  <td><input type="number" step="0.1" id="b-rt-max" aria-label="Rain Total Max"></td>
                  <td>mm</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </form>
    </section>
  </main>

  <div id="toast" class="toast" role="status" aria-live="polite"></div>

  <script>
    const drawer = document.getElementById('nav-drawer');
    const backdrop = document.getElementById('drawer-backdrop');
    const openBtn = document.getElementById('drawer-open-btn');
    const closeBtn = document.getElementById('drawer-close-btn');
    const mainContent = document.getElementById('main-content');
    const viewDashboard = document.getElementById('view-dashboard');
    const viewSettings = document.getElementById('view-settings');
    const navBtnDashboard = document.getElementById('nav-btn-dashboard');
    const navBtnSettings = document.getElementById('nav-btn-settings');
    let currentConfig = null;

    function showToast(msg) {
      const t = document.getElementById('toast');
      t.textContent = msg;
      t.style.display = 'block';
      clearTimeout(window._toastTimer);
      window._toastTimer = setTimeout(() => { t.style.display = 'none'; }, 3500);
    }

    function openDrawer() {
      drawer.classList.add('open');
      backdrop.classList.add('open');
      openBtn.setAttribute('aria-expanded', 'true');
      mainContent.inert = true;
      drawer.focus();
    }

    function closeDrawer() {
      drawer.classList.remove('open');
      backdrop.classList.remove('open');
      openBtn.setAttribute('aria-expanded', 'false');
      mainContent.inert = false;
    }

    openBtn.addEventListener('click', openDrawer);
    closeBtn.addEventListener('click', closeDrawer);
    backdrop.addEventListener('click', closeDrawer);
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape') closeDrawer();
    });

    function switchView(viewName) {
      if (viewName === 'settings') {
        if (!isUserAuthorized) {
          switchView('dashboard');
          return;
        }
        viewDashboard.classList.remove('active');
        viewSettings.classList.add('active');
        navBtnDashboard.classList.remove('active');
        navBtnSettings.classList.add('active');
      } else {
        viewSettings.classList.remove('active');
        viewDashboard.classList.add('active');
        navBtnSettings.classList.remove('active');
        navBtnDashboard.classList.add('active');
      }
      closeDrawer();
    }

    navBtnDashboard.addEventListener('click', () => switchView('dashboard'));
    navBtnSettings.addEventListener('click', () => switchView('settings'));
    document.getElementById('brand-link').addEventListener('click', (e) => {
      e.preventDefault();
      switchView('dashboard');
    });
    document.getElementById('btn-back-dashboard').addEventListener('click', () => switchView('dashboard'));

    // Settings Tabs Handling
    document.querySelectorAll('.tab-btn').forEach((tab) => {
      tab.addEventListener('click', () => {
        document.querySelectorAll('.tab-btn').forEach((t) => {
          t.classList.remove('active');
          t.setAttribute('aria-selected', 'false');
        });
        document.querySelectorAll('.tab-panel').forEach((p) => p.classList.remove('active'));

        tab.classList.add('active');
        tab.setAttribute('aria-selected', 'true');
        const targetId = tab.getAttribute('data-tab-target');
        const panel = document.getElementById(targetId);
        if (panel) panel.classList.add('active');
      });
    });

    function fmtNum(v, decimals = 1) {
      if (v === null || v === undefined) return '—';
      return Number(v).toFixed(decimals);
    }

    function fmtTs(iso) {
      if (!iso) return '—';
      const d = new Date(iso);
      return d.toLocaleString();
    }

    let lastSeenReadingTs = null;
    let serialFlashTimer = null;

    function renderSnapshot(snap) {
      const serialLed = document.getElementById('serial-led');
      const serialIndicator = document.getElementById('serial-indicator');
      const isConnected = snap.serial_connected || (snap.running && !snap.last_error);

      if (!isConnected) {
        if (serialFlashTimer) {
          clearTimeout(serialFlashTimer);
          serialFlashTimer = null;
        }
        serialLed.className = 'serial-led led-red';
        serialIndicator.title = snap.last_error ? `Serial Error: ${snap.last_error}` : 'Serial Port Disconnected (Service Stopped)';
      } else {
        const currentTs = snap.latest ? snap.latest.timestamp : null;
        const isNewReading = currentTs && currentTs !== lastSeenReadingTs;

        if (isNewReading) {
          lastSeenReadingTs = currentTs;
          // Pulse light green when bits fly / new data arrives
          serialLed.className = 'serial-led led-flash';
          serialIndicator.title = `Serial Port Active (Reading received at ${fmtTs(currentTs)})`;
          if (serialFlashTimer) clearTimeout(serialFlashTimer);
          serialFlashTimer = setTimeout(() => {
            if (serialLed.classList.contains('led-flash')) {
              serialLed.className = 'serial-led led-dark-green';
              serialIndicator.title = 'Serial Port Connected (Idle, waiting for next poll)';
            }
          }, 850);
        } else if (!serialLed.classList.contains('led-flash')) {
          serialLed.className = 'serial-led led-dark-green';
          serialIndicator.title = 'Serial Port Connected (Idle, waiting for next poll)';
        }
      }

      const badge = document.getElementById('service-status-badge');
      const statusText = document.getElementById('service-status-text');
      const toggleBtn = document.getElementById('btn-toggle-service');

      if (snap.alert_active) {
        badge.className = 'badge badge-alert';
        statusText.textContent = 'STALE DATA ALERT';
      } else if (snap.running) {
        badge.className = 'badge badge-running';
        statusText.textContent = snap.demo_mode ? 'RUNNING (DEMO)' : 'SERVICE RUNNING';
      } else {
        badge.className = 'badge badge-stopped';
        statusText.textContent = 'SERVICE STOPPED';
      }

      currentServiceRunning = !!snap.running;
      updateServiceToggleButton(snap.running);

      document.getElementById('header-port-label').textContent =
        `${snap.serial_port} · Every ${snap.poll_interval_seconds}s`;

      const errBanner = document.getElementById('error-banner');
      const errText = document.getElementById('error-banner-text');
      if (snap.last_error) {
        errBanner.style.display = 'flex';
        errText.textContent = `Serial Status Warning: ${snap.last_error}`;
      } else {
        errBanner.style.display = 'none';
      }

      document.getElementById('mqtt-status-label').textContent =
        snap.mqtt_connected ? 'Connected & Published' : 'Standby / Disabled';

      document.getElementById('card-wind').style.display = snap.include_wind ? 'flex' : 'none';
      document.getElementById('card-rain').style.display = snap.include_rain ? 'flex' : 'none';

      const latest = snap.latest;
      const prev = snap.previous;

      document.getElementById('ts-latest').textContent = latest ? fmtTs(latest.timestamp) : 'Waiting for station poll...';
      document.getElementById('ts-previous').textContent = prev ? fmtTs(prev.timestamp) : '—';

      if (latest && latest.data) {
        const d = latest.data;
        const clamped = latest.clamped_fields || [];

        // Indoor tile values
        document.getElementById('val-temp-in').textContent = fmtNum(d.temperature_indoor, 1);
        document.getElementById('val-hum-in').textContent = d.humidity_indoor ?? '—';

        // Outdoor tile values (including outdoor humidity and dewpoint)
        document.getElementById('val-temp-out').textContent = fmtNum(d.temperature_outdoor, 1);
        document.getElementById('val-hum-out').textContent = d.humidity_outdoor ?? '—';
        document.getElementById('val-dewpoint').textContent = fmtNum(d.dewpoint, 1);

        // Pressure & Forecast
        document.getElementById('val-pressure').textContent = fmtNum(d.pressure, 1);
        document.getElementById('val-tendency').textContent = d.tendency || '—';
        document.getElementById('val-forecast').textContent = d.forecast || '—';

        if (snap.include_wind) {
          document.getElementById('val-wind-speed').textContent = fmtNum(d.wind_speed, 1);
          document.getElementById('val-wind-dir-deg').textContent = fmtNum(d.wind_dir, 1);
          document.getElementById('val-wind-dir-str').textContent = d.wind_direction || '—';
          document.getElementById('val-wind-chill').textContent = fmtNum(d.wind_chill, 1);
          const deg = Number(d.wind_dir || 0);
          document.getElementById('compass-needle').setAttribute('transform', `rotate(${deg} 50 50)`);
        }

        if (snap.include_rain) {
          document.getElementById('val-rain-1h').textContent = fmtNum(d.rain_1h, 1);
          document.getElementById('val-rain-24h').textContent = fmtNum(d.rain_24h, 1);
          document.getElementById('val-rain-total').textContent = fmtNum(d.rain_total, 1);
        }

        document.getElementById('clamp-indoor').style.display =
          (clamped.includes('temperature_indoor') || clamped.includes('humidity_indoor')) ? 'inline-flex' : 'none';
        document.getElementById('clamp-outdoor').style.display =
          (clamped.includes('temperature_outdoor') || clamped.includes('humidity_outdoor') || clamped.includes('dewpoint')) ? 'inline-flex' : 'none';
        document.getElementById('clamp-pressure').style.display =
          clamped.includes('pressure') ? 'inline-flex' : 'none';
        document.getElementById('clamp-wind').style.display =
          (clamped.includes('wind_speed') || clamped.includes('wind_dir') || clamped.includes('wind_chill')) ? 'inline-flex' : 'none';
        document.getElementById('clamp-rain').style.display =
          (clamped.includes('rain_1h') || clamped.includes('rain_24h') || clamped.includes('rain_total')) ? 'inline-flex' : 'none';
      }

      if (prev && prev.data) {
        const p = prev.data;
        document.getElementById('prev-temp-in').textContent = fmtNum(p.temperature_indoor, 1);
        document.getElementById('prev-hum-in').textContent = p.humidity_indoor ?? '—';
        document.getElementById('prev-temp-out').textContent = fmtNum(p.temperature_outdoor, 1);
        document.getElementById('prev-hum-out').textContent = p.humidity_outdoor ?? '—';
        document.getElementById('prev-dewpoint').textContent = fmtNum(p.dewpoint, 1);
        document.getElementById('prev-pressure').textContent = fmtNum(p.pressure, 1);
        document.getElementById('prev-tendency').textContent = p.tendency || '—';
        document.getElementById('prev-forecast').textContent = p.forecast || '—';

        if (snap.include_wind) {
          document.getElementById('prev-wind-speed').textContent = fmtNum(p.wind_speed, 1);
          document.getElementById('prev-wind-dir-str').textContent = p.wind_direction || '—';
          document.getElementById('prev-wind-chill').textContent = fmtNum(p.wind_chill, 1);
        }
        if (snap.include_rain) {
          document.getElementById('prev-rain-1h').textContent = fmtNum(p.rain_1h, 1);
          document.getElementById('prev-rain-24h').textContent = fmtNum(p.rain_24h, 1);
          document.getElementById('prev-rain-total').textContent = fmtNum(p.rain_total, 1);
        }
      }
    }

    async function loadConfigIntoForm() {
      const res = await fetch('/api/config');
      if (res.status === 401) {
        window.location.href = '/auth/login';
        return;
      }
      const cfg = await res.json();
      currentConfig = cfg;

      document.getElementById('cfg-serial-port').value = cfg.serial.port;
      document.getElementById('cfg-poll-interval').value = cfg.service.poll_interval_seconds;
      document.getElementById('cfg-bind-addr').value = cfg.service.bind_address;
      document.getElementById('cfg-auto-start').checked = cfg.service.auto_start;
      document.getElementById('cfg-demo-mode').checked = cfg.service.demo_mode;

      document.getElementById('cfg-include-wind').checked = cfg.metrics.include_wind;
      document.getElementById('cfg-include-rain').checked = cfg.metrics.include_rain;

      document.getElementById('cfg-mqtt-enabled').checked = cfg.mqtt.enabled;
      document.getElementById('cfg-mqtt-host').value = cfg.mqtt.broker_host;
      document.getElementById('cfg-mqtt-port').value = cfg.mqtt.broker_port;
      document.getElementById('cfg-mqtt-user').value = cfg.mqtt.username;
      document.getElementById('cfg-mqtt-pass').value = cfg.mqtt.password;
      document.getElementById('cfg-mqtt-topic').value = cfg.mqtt.topic;
      document.getElementById('cfg-mqtt-client').value = cfg.mqtt.client_id;
      document.getElementById('cfg-mqtt-retain').checked = cfg.mqtt.retain;

      document.getElementById('cfg-alerts-enabled').checked = cfg.alerts.enabled;
      document.getElementById('cfg-alert-minutes').value = cfg.alerts.stale_timeout_minutes;
      document.getElementById('cfg-email-enabled').checked = cfg.alerts.email.enabled;
      document.getElementById('cfg-smtp-host').value = cfg.alerts.email.smtp_host;
      document.getElementById('cfg-smtp-security').value = cfg.alerts.email.smtp_security || 'starttls';
      document.getElementById('cfg-smtp-port').value = cfg.alerts.email.smtp_port;
      document.getElementById('cfg-smtp-user').value = cfg.alerts.email.smtp_username;
      document.getElementById('cfg-smtp-pass').value = cfg.alerts.email.smtp_password;
      document.getElementById('cfg-smtp-from').value = cfg.alerts.email.from_address;
      document.getElementById('cfg-smtp-to').value = cfg.alerts.email.to_address;

      document.getElementById('cfg-tg-enabled').checked = cfg.alerts.telegram.enabled;
      document.getElementById('cfg-tg-token').value = cfg.alerts.telegram.bot_token;
      document.getElementById('cfg-tg-chat').value = cfg.alerts.telegram.chat_id;

      document.getElementById('cfg-webhook-enabled').checked = cfg.alerts.webhook.enabled;
      document.getElementById('cfg-webhook-url').value = cfg.alerts.webhook.url;

      document.getElementById('cfg-oidc-enabled').checked = cfg.oidc.enabled;
      document.getElementById('cfg-oidc-issuer').value = cfg.oidc.issuer_url;
      document.getElementById('cfg-oidc-client-id').value = cfg.oidc.client_id;
      document.getElementById('cfg-oidc-secret').value = cfg.oidc.client_secret;
      document.getElementById('cfg-oidc-redirect').value = cfg.oidc.redirect_url;

      const b = cfg.bounds;
      document.getElementById('cfg-bounds-replace').checked = b.replace_with_previous;
      document.getElementById('b-ti-min').value = b.temperature_indoor.min;
      document.getElementById('b-ti-max').value = b.temperature_indoor.max;
      document.getElementById('b-to-min').value = b.temperature_outdoor.min;
      document.getElementById('b-to-max').value = b.temperature_outdoor.max;
      document.getElementById('b-dp-min').value = b.dewpoint.min;
      document.getElementById('b-dp-max').value = b.dewpoint.max;
      document.getElementById('b-hi-min').value = b.humidity_indoor.min;
      document.getElementById('b-hi-max').value = b.humidity_indoor.max;
      document.getElementById('b-ho-min').value = b.humidity_outdoor.min;
      document.getElementById('b-ho-max').value = b.humidity_outdoor.max;
      document.getElementById('b-pr-min').value = b.pressure.min;
      document.getElementById('b-pr-max').value = b.pressure.max;
      document.getElementById('b-ws-min').value = b.wind_speed.min;
      document.getElementById('b-ws-max').value = b.wind_speed.max;
      document.getElementById('b-wd-min').value = b.wind_dir.min;
      document.getElementById('b-wd-max').value = b.wind_dir.max;
      document.getElementById('b-wc-min').value = b.wind_chill.min;
      document.getElementById('b-wc-max').value = b.wind_chill.max;
      document.getElementById('b-r1-min').value = b.rain_1h.min;
      document.getElementById('b-r1-max').value = b.rain_1h.max;
      document.getElementById('b-r24-min').value = b.rain_24h.min;
      document.getElementById('b-r24-max').value = b.rain_24h.max;
      document.getElementById('b-rt-min').value = b.rain_total.min;
      document.getElementById('b-rt-max').value = b.rain_total.max;
    }

    async function scanSerialPorts(showFeedback = false) {
      const res = await fetch('/api/serial-ports');
      if (!res.ok) return;
      const ports = await res.json();
      const dl = document.getElementById('serial-ports-datalist');
      dl.innerHTML = '';
      ports.forEach((p) => {
        const opt = document.createElement('option');
        opt.value = p;
        dl.appendChild(opt);
      });
      if (showFeedback) {
        showToast(`Detected ${ports.length} serial port candidate(s)`);
      }
    }

    document.getElementById('btn-scan-ports').addEventListener('click', () => scanSerialPorts(true));
    document.getElementById('cfg-smtp-security').addEventListener('change', (e) => {
      const portInput = document.getElementById('cfg-smtp-port');
      const val = e.target.value;
      if (val === 'starttls') {
        portInput.value = 587;
      } else if (val === 'tls') {
        portInput.value = 465;
      } else if (val === 'plain') {
        portInput.value = 25;
      }
    });
    document.getElementById('btn-reload-cfg').addEventListener('click', async () => {
      await loadConfigIntoForm();
      showToast('Configuration reloaded from config.yaml');
    });

    document.getElementById('settings-form').addEventListener('submit', async (e) => {
      e.preventDefault();
      if (!currentConfig) currentConfig = {};

      const payload = {
        service: {
          auto_start: document.getElementById('cfg-auto-start').checked,
          poll_interval_seconds: Number(document.getElementById('cfg-poll-interval').value),
          bind_address: document.getElementById('cfg-bind-addr').value.trim(),
          demo_mode: document.getElementById('cfg-demo-mode').checked
        },
        serial: {
          port: document.getElementById('cfg-serial-port').value.trim()
        },
        metrics: {
          include_wind: document.getElementById('cfg-include-wind').checked,
          include_rain: document.getElementById('cfg-include-rain').checked
        },
        mqtt: {
          enabled: document.getElementById('cfg-mqtt-enabled').checked,
          broker_host: document.getElementById('cfg-mqtt-host').value.trim(),
          broker_port: Number(document.getElementById('cfg-mqtt-port').value),
          username: document.getElementById('cfg-mqtt-user').value.trim(),
          password: document.getElementById('cfg-mqtt-pass').value,
          topic: document.getElementById('cfg-mqtt-topic').value.trim(),
          client_id: document.getElementById('cfg-mqtt-client').value.trim(),
          retain: document.getElementById('cfg-mqtt-retain').checked
        },
        alerts: {
          enabled: document.getElementById('cfg-alerts-enabled').checked,
          stale_timeout_minutes: Number(document.getElementById('cfg-alert-minutes').value),
          email: {
            enabled: document.getElementById('cfg-email-enabled').checked,
            smtp_host: document.getElementById('cfg-smtp-host').value.trim(),
            smtp_security: document.getElementById('cfg-smtp-security').value,
            smtp_port: Number(document.getElementById('cfg-smtp-port').value),
            smtp_username: document.getElementById('cfg-smtp-user').value.trim(),
            smtp_password: document.getElementById('cfg-smtp-pass').value,
            from_address: document.getElementById('cfg-smtp-from').value.trim(),
            to_address: document.getElementById('cfg-smtp-to').value.trim()
          },
          telegram: {
            enabled: document.getElementById('cfg-tg-enabled').checked,
            bot_token: document.getElementById('cfg-tg-token').value.trim(),
            chat_id: document.getElementById('cfg-tg-chat').value.trim()
          },
          webhook: {
            enabled: document.getElementById('cfg-webhook-enabled').checked,
            url: document.getElementById('cfg-webhook-url').value.trim()
          }
        },
        oidc: {
          enabled: document.getElementById('cfg-oidc-enabled').checked,
          issuer_url: document.getElementById('cfg-oidc-issuer').value.trim(),
          authorization_endpoint: currentConfig.oidc?.authorization_endpoint || '',
          token_endpoint: currentConfig.oidc?.token_endpoint || '',
          userinfo_endpoint: currentConfig.oidc?.userinfo_endpoint || '',
          client_id: document.getElementById('cfg-oidc-client-id').value.trim(),
          client_secret: document.getElementById('cfg-oidc-secret').value,
          redirect_url: document.getElementById('cfg-oidc-redirect').value.trim(),
          session_secret: currentConfig.oidc?.session_secret || 'change-me-session-secret-ws2300-key'
        },
        bounds: {
          replace_with_previous: document.getElementById('cfg-bounds-replace').checked,
          temperature_indoor: { min: Number(document.getElementById('b-ti-min').value), max: Number(document.getElementById('b-ti-max').value) },
          temperature_outdoor: { min: Number(document.getElementById('b-to-min').value), max: Number(document.getElementById('b-to-max').value) },
          dewpoint: { min: Number(document.getElementById('b-dp-min').value), max: Number(document.getElementById('b-dp-max').value) },
          humidity_indoor: { min: Number(document.getElementById('b-hi-min').value), max: Number(document.getElementById('b-hi-max').value) },
          humidity_outdoor: { min: Number(document.getElementById('b-ho-min').value), max: Number(document.getElementById('b-ho-max').value) },
          pressure: { min: Number(document.getElementById('b-pr-min').value), max: Number(document.getElementById('b-pr-max').value) },
          wind_speed: { min: Number(document.getElementById('b-ws-min').value), max: Number(document.getElementById('b-ws-max').value) },
          wind_dir: { min: Number(document.getElementById('b-wd-min').value), max: Number(document.getElementById('b-wd-max').value) },
          wind_chill: { min: Number(document.getElementById('b-wc-min').value), max: Number(document.getElementById('b-wc-max').value) },
          rain_1h: { min: Number(document.getElementById('b-r1-min').value), max: Number(document.getElementById('b-r1-max').value) },
          rain_24h: { min: Number(document.getElementById('b-r24-min').value), max: Number(document.getElementById('b-r24-max').value) },
          rain_total: { min: Number(document.getElementById('b-rt-min').value), max: Number(document.getElementById('b-rt-max').value) }
        }
      };

      const btn = document.getElementById('btn-save-cfg');
      btn.disabled = true;
      try {
        const res = await fetch('/api/config', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        });
        const out = await res.json();
        if (res.ok) {
          showToast('Saved configuration to config.yaml');
          const snapRes = await fetch('/api/status');
          if (snapRes.ok) renderSnapshot(await snapRes.json());
        } else {
          showToast(`Error: ${out.error || 'Failed to save config'}`);
        }
      } finally {
        btn.disabled = false;
      }
    });

    let currentServiceRunning = false;
    let isUserAuthorized = false;

    function updateServiceToggleButton(snapRunning) {
      const toggleBtn = document.getElementById('btn-toggle-service');
      if (!toggleBtn) return;
      if (!isUserAuthorized) {
        toggleBtn.style.display = 'none';
        return;
      }
      toggleBtn.style.display = 'inline-block';
      if (snapRunning) {
        toggleBtn.textContent = 'Stop';
        toggleBtn.className = 'btn btn-danger';
      } else {
        toggleBtn.textContent = 'Start';
        toggleBtn.className = 'btn btn-primary';
      }
    }

    document.getElementById('btn-toggle-service').addEventListener('click', async () => {
      const toggleBtn = document.getElementById('btn-toggle-service');
      toggleBtn.disabled = true;
      try {
        const willStop = currentServiceRunning;
        const endpoint = willStop ? '/api/service/stop' : '/api/service/start';
        const res = await fetch(endpoint, { method: 'POST' });
        if (res.ok) {
          const snap = await res.json();
          renderSnapshot(snap);
          showToast(willStop ? 'WS2300 Polling Service Stopped' : 'WS2300 Polling Service Started');
        }
      } finally {
        toggleBtn.disabled = false;
      }
    });

    document.getElementById('btn-test-alert').addEventListener('click', async () => {
      const res = await fetch('/api/alerts/test', { method: 'POST' });
      const data = await res.json();
      if (res.ok) {
        showToast(`Test Alert Dispatched: ${(data.results || []).join(', ')}`);
      } else {
        showToast(`Alert Test Failed: ${data.error}`);
      }
    });

    async function checkCurrentUser() {
      try {
        const res = await fetch('/api/auth/me');
        if (!res.ok) return;
        const data = await res.json();
        const badge = document.getElementById('oidc-user-badge');
        const nameEl = document.getElementById('oidc-user-name');
        const settingsNavItem = document.getElementById('drawer-settings-item');
        const drawerAuthItem = document.getElementById('drawer-auth-item');
        const drawerAuthLink = document.getElementById('drawer-auth-link');
        const drawerAuthLabel = document.getElementById('drawer-auth-label');
        const drawerAuthIcon = document.getElementById('drawer-auth-icon');

        if (!data.oidc_enabled) {
          // OIDC disabled: no user badge, no login/logout button, settings accessible to all
          isUserAuthorized = true;
          badge.style.display = 'none';
          if (settingsNavItem) settingsNavItem.style.display = 'block';
          if (drawerAuthItem) drawerAuthItem.style.display = 'none';
        } else if (data.authenticated && data.username) {
          // OIDC enabled & user logged in
          isUserAuthorized = true;
          nameEl.textContent = data.username;
          badge.style.display = 'inline-flex';
          if (settingsNavItem) settingsNavItem.style.display = 'block';
          if (drawerAuthItem) {
            drawerAuthItem.style.display = 'block';
            drawerAuthLink.href = '/auth/logout';
            drawerAuthLink.style.color = '#DC2626';
            drawerAuthLabel.textContent = 'Log Out';
            drawerAuthIcon.textContent = '⎋';
          }
        } else {
          // OIDC enabled & user NOT logged in: hide settings, hide user badge, show Login
          isUserAuthorized = false;
          badge.style.display = 'none';
          if (settingsNavItem) settingsNavItem.style.display = 'none';
          if (drawerAuthItem) {
            drawerAuthItem.style.display = 'block';
            drawerAuthLink.href = '/auth/login';
            drawerAuthLink.style.color = 'inherit';
            drawerAuthLabel.textContent = 'Log In';
            drawerAuthIcon.textContent = '→';
          }
          // If on settings view, redirect to dashboard
          if (viewSettings && viewSettings.classList.contains('active')) {
            switchView('dashboard');
          }
        }
        updateServiceToggleButton(currentServiceRunning);
      } catch (_) {}
    }

    async function init() {
      const statusRes = await fetch('/api/status');
      if (statusRes.ok) {
        renderSnapshot(await statusRes.json());
      }
      await checkCurrentUser();

      // Only load sensitive configuration form and serial ports if user is authorized
      if (isUserAuthorized) {
        await loadConfigIntoForm();
        await scanSerialPorts();
      }

      const es = new EventSource('/api/events');
      es.onmessage = (evt) => {
        try {
          const snap = JSON.parse(evt.data);
          renderSnapshot(snap);
        } catch (_) {}
      };
    }

    init();
  </script>
</body>
</html>
"##;
