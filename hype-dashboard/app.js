document.addEventListener('DOMContentLoaded', () => {
    const alertsContainer = document.getElementById('alerts-container');
    
    // Connect to the Rust backend serving the WebSocket
    const ws = new WebSocket('ws://localhost:3000/ws');

    ws.onopen = () => {
        console.log('🔗 Connected to Eat The Rich Oracle Stream');
        // Clear placeholder
        alertsContainer.innerHTML = '';
    };

    ws.onmessage = (event) => {
        try {
            const data = JSON.parse(event.data);
            
            if (data.type === 'T1_ANOMALY') {
                injectAlertCard(data);
            }
        } catch (e) {
            console.error("Error parsing websocket message:", e);
        }
    };

    ws.onclose = () => {
        console.warn('⚠️ Disconnected from Oracle Stream');
        alertsContainer.innerHTML = `
            <div class="alert-card glass-panel placeholder" style="border-left-color: var(--accent-red)">
                <div class="loading-pulse" style="color: var(--accent-red)">Connection lost. Retrying...</div>
            </div>
        `;
    };

    function injectAlertCard(data) {
        // Create the card
        const card = document.createElement('div');
        card.className = 'alert-card glass-panel';
        
        // Ensure newest alerts are at the top
        card.innerHTML = `
            <div class="alert-header">
                <span class="alert-ticker">${data.ticker}</span>
                <span class="alert-shift">${data.sentiment_shift} Vol</span>
            </div>
            <div class="alert-body">
                <p>Price Action: <strong>${data.price_action}</strong></p>
                <p>Source Volume: <strong>${data.source_volume} mentions</strong></p>
                <p style="margin-top: 10px; font-size: 0.8rem; color: var(--accent-green)">High probability divergence detected.</p>
            </div>
        `;

        // Prepend to container and trigger reflow for animation
        alertsContainer.prepend(card);
        
        // Remove older cards if we exceed 6 to keep UI clean
        if (alertsContainer.children.length > 6) {
            alertsContainer.removeChild(alertsContainer.lastChild);
        }
    }
});
