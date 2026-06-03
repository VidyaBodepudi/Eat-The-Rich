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
        
        // Build DOM safely to avoid interpreting untrusted HTML
        const header = document.createElement('div');
        header.className = 'alert-header';

        const ticker = document.createElement('span');
        ticker.className = 'alert-ticker';
        ticker.textContent = data.ticker;

        const shift = document.createElement('span');
        shift.className = 'alert-shift';
        shift.textContent = `${data.sentiment_shift} Vol`;

        header.appendChild(ticker);
        header.appendChild(shift);

        const body = document.createElement('div');
        body.className = 'alert-body';

        const priceP = document.createElement('p');
        priceP.append('Price Action: ');
        const priceStrong = document.createElement('strong');
        priceStrong.textContent = data.price_action;
        priceP.appendChild(priceStrong);

        const volumeP = document.createElement('p');
        volumeP.append('Source Volume: ');
        const volumeStrong = document.createElement('strong');
        volumeStrong.textContent = `${data.source_volume} mentions`;
        volumeP.appendChild(volumeStrong);

        const noteP = document.createElement('p');
        noteP.style.marginTop = '10px';
        noteP.style.fontSize = '0.8rem';
        noteP.style.color = 'var(--accent-green)';
        noteP.textContent = 'High probability divergence detected.';

        body.appendChild(priceP);
        body.appendChild(volumeP);
        body.appendChild(noteP);

        card.appendChild(header);
        card.appendChild(body);

        // Prepend to container and trigger reflow for animation
        alertsContainer.prepend(card);
        
        // Remove older cards if we exceed 6 to keep UI clean
        if (alertsContainer.children.length > 6) {
            alertsContainer.removeChild(alertsContainer.lastChild);
        }
    }
});
