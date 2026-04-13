document.addEventListener('DOMContentLoaded', () => {
    const connStatus = document.getElementById('conn-status');
    const alertsGrid = document.getElementById('alerts-grid');
    const template = document.getElementById('alert-template');

    // Connect to the Rust Axum WebSocket backend (Phase 2 core)
    let ws;
    
    function connect() {
        console.log("Attempting to connect to Arrow IPC WebSocket...");
        // This is the URL configured in eat-the-rich-core/src/api.rs
        ws = new WebSocket('ws://127.0.0.1:3000/ws');

        ws.onopen = () => {
            connStatus.textContent = 'Hardware Link Active 🟢';
            connStatus.classList.add('live');
        };

        ws.onmessage = (event) => {
            try {
                // The backend sends mocked Zero-overhead anomaly payloads
                const anomaly = JSON.parse(event.data);
                
                // Remove the "radar scan" placeholder if it exists
                const radar = document.querySelector('.radar-scan');
                if (radar) radar.remove();

                injectAlert(anomaly);
            } catch (err) {
                console.error("Payload decoding error:", err);
            }
        };

        ws.onclose = () => {
            connStatus.textContent = 'Connection Lost 🔴';
            connStatus.classList.remove('live');
            // Attempt auto-reconnect
            setTimeout(connect, 3000);
        };
    }

    function injectAlert(anomaly) {
        // Clone the HTML template
        const clone = template.content.cloneNode(true);
        const card = clone.querySelector('.alert-card');

        // Populate dynamic data
        clone.querySelector('.ticker-symbol').textContent = `$${anomaly.ticker}`;
        clone.querySelector('.param-hype').textContent = anomaly.sentiment_shift;
        clone.querySelector('.metric-row > div:nth-child(2) .metric-value').textContent = 
            anomaly.source_volume.toLocaleString();
        
        // Define Visual Flash Triggers
        if (anomaly.type === 'T1_ANOMALY' || anomaly.type === 'SHORT_TARGET') {
            card.classList.add('flash-short');
            clone.querySelector('.signal-badge').textContent = '⚠️ IRRATIONAL EXUBERANCE (SHORT)';
        } else if (anomaly.type === 'BUY_TARGET') {
            card.classList.add('flash-long');
            clone.querySelector('.signal-badge').textContent = '✅ UNWARRANTED PANIC (BUY)';
        }

        // Add to the top of the grid
        alertsGrid.insertBefore(clone, alertsGrid.firstChild);

        // Keep only top 10 elements to prevent DOM bloat during high-frequency streams
        if (alertsGrid.children.length > 10) {
            alertsGrid.removeChild(alertsGrid.lastChild);
        }
    }

    // Initialize WebSockets
    connect();
});
