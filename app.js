/**
 * Counter App - Frontend JavaScript
 * 
 * Connects to Counter API backend and manages UI state
 */

// Configuration
const CONFIG = {
    // API URL configuration
    // For local development: http://localhost:3001/api/counter
    // For production: Set this to your deployed backend URL
    API_BASE_URL: (() => {
        // Auto-detect based on hostname
        if (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1') {
            return 'http://localhost:3001';
        }
        // GitHub Pages deployment - backend URL needs to be configured
        // Update this URL when backend is deployed
        return 'https://your-backend-url.herokuapp.com';
    })(),
    
    // API endpoints
    ENDPOINTS: {
        COUNTER: '/api/counter',
        INCREMENT: '/api/counter/increment',
        DECREMENT: '/api/counter/decrement'
    },
    
    // Request timeout (ms)
    TIMEOUT: 5000
};

// DOM Elements
const elements = {
    counterValue: document.getElementById('counter-value'),
    btnIncrement: document.getElementById('btn-increment'),
    btnDecrement: document.getElementById('btn-decrement'),
    statusIndicator: document.getElementById('status-indicator'),
    statusText: document.getElementById('status-text'),
    loadingOverlay: document.getElementById('loading-overlay'),
    apiStatus: document.getElementById('api-status'),
    apiLink: document.getElementById('api-link')
};

// Application State
const state = {
    count: null,
    isConnected: false,
    isLoading: false
};

/**
 * Fetch with timeout
 */
async function fetchWithTimeout(url, options = {}, timeout = CONFIG.TIMEOUT) {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), timeout);
    
    try {
        const response = await fetch(url, {
            ...options,
            signal: controller.signal
        });
        clearTimeout(timeoutId);
        return response;
    } catch (error) {
        clearTimeout(timeoutId);
        throw error;
    }
}

/**
 * Update the connection status indicator
 */
function updateStatus(status, text) {
    elements.statusIndicator.className = 'status-indicator ' + status;
    elements.statusText.textContent = text;
    
    if (status === 'connected') {
        elements.apiStatus.textContent = 'API: Connected';
        elements.apiLink.href = CONFIG.API_BASE_URL + CONFIG.ENDPOINTS.COUNTER;
        elements.apiLink.textContent = CONFIG.API_BASE_URL;
    } else if (status === 'disconnected') {
        elements.apiStatus.textContent = 'API: Disconnected';
        elements.apiLink.textContent = 'Backend not available';
        elements.apiLink.removeAttribute('href');
    } else {
        elements.apiStatus.textContent = 'API: ';
        elements.apiLink.textContent = 'Checking...';
    }
}

/**
 * Update the counter display
 */
function updateCounterDisplay(value, animate = false) {
    if (animate) {
        elements.counterValue.classList.add('updating');
        setTimeout(() => elements.counterValue.classList.remove('updating'), 150);
    }
    elements.counterValue.textContent = value;
}

/**
 * Set loading state
 */
function setLoading(isLoading) {
    state.isLoading = isLoading;
    if (isLoading) {
        elements.loadingOverlay.classList.add('active');
    } else {
        elements.loadingOverlay.classList.remove('active');
    }
}

/**
 * Enable or disable buttons
 */
function setButtonsEnabled(enabled) {
    elements.btnIncrement.disabled = !enabled;
    elements.btnDecrement.disabled = !enabled;
}

/**
 * Fetch the current counter value from API
 */
async function fetchCounter() {
    try {
        const response = await fetchWithTimeout(
            CONFIG.API_BASE_URL + CONFIG.ENDPOINTS.COUNTER
        );
        
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }
        
        const data = await response.json();
        return data.count;
    } catch (error) {
        console.error('Failed to fetch counter:', error);
        throw error;
    }
}

/**
 * Increment the counter via API
 */
async function incrementCounter() {
    try {
        const response = await fetchWithTimeout(
            CONFIG.API_BASE_URL + CONFIG.ENDPOINTS.INCREMENT,
            { method: 'POST' }
        );
        
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }
        
        const data = await response.json();
        return data.count;
    } catch (error) {
        console.error('Failed to increment counter:', error);
        throw error;
    }
}

/**
 * Decrement the counter via API
 */
async function decrementCounter() {
    try {
        const response = await fetchWithTimeout(
            CONFIG.API_BASE_URL + CONFIG.ENDPOINTS.DECREMENT,
            { method: 'POST' }
        );
        
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }
        
        const data = await response.json();
        return data.count;
    } catch (error) {
        console.error('Failed to decrement counter:', error);
        throw error;
    }
}

/**
 * Handle increment button click
 */
async function handleIncrement() {
    if (state.isLoading || !state.isConnected) return;
    
    setLoading(true);
    setButtonsEnabled(false);
    
    try {
        const newCount = await incrementCounter();
        state.count = newCount;
        updateCounterDisplay(newCount, true);
    } catch (error) {
        showError('Failed to increment counter');
        handleConnectionLost();
    } finally {
        setLoading(false);
        setButtonsEnabled(state.isConnected);
    }
}

/**
 * Handle decrement button click
 */
async function handleDecrement() {
    if (state.isLoading || !state.isConnected) return;
    
    setLoading(true);
    setButtonsEnabled(false);
    
    try {
        const newCount = await decrementCounter();
        state.count = newCount;
        updateCounterDisplay(newCount, true);
    } catch (error) {
        showError('Failed to decrement counter');
        handleConnectionLost();
    } finally {
        setLoading(false);
        setButtonsEnabled(state.isConnected);
    }
}

/**
 * Handle lost connection to API
 */
function handleConnectionLost() {
    state.isConnected = false;
    updateStatus('disconnected', 'Disconnected from server');
    setButtonsEnabled(false);
    
    // Try to reconnect
    setTimeout(checkConnection, 2000);
}

/**
 * Show error message
 */
function showError(message) {
    // Create or update error element
    let errorEl = document.querySelector('.error-message');
    if (!errorEl) {
        errorEl = document.createElement('div');
        errorEl.className = 'error-message';
        elements.counterValue.parentElement.parentElement.appendChild(errorEl);
    }
    errorEl.textContent = message;
    errorEl.style.display = 'block';
    
    // Hide after 3 seconds
    setTimeout(() => {
        errorEl.style.display = 'none';
    }, 3000);
}

/**
 * Check connection to API
 */
async function checkConnection() {
    updateStatus('checking', 'Checking connection...');
    
    try {
        const count = await fetchCounter();
        state.count = count;
        state.isConnected = true;
        updateStatus('connected', 'Connected');
        updateCounterDisplay(count);
        setButtonsEnabled(true);
        
        // Remove any error message
        const errorEl = document.querySelector('.error-message');
        if (errorEl) {
            errorEl.style.display = 'none';
        }
        
        return true;
    } catch (error) {
        state.isConnected = false;
        updateStatus('disconnected', 'Cannot connect to server');
        updateCounterDisplay('-');
        setButtonsEnabled(false);
        
        // Show backend not available message
        showError('Backend server is not available. Please start the server or configure the API URL in app.js');
        
        return false;
    }
}

/**
 * Initialize the application
 */
function init() {
    // Set up event listeners
    elements.btnIncrement.addEventListener('click', handleIncrement);
    elements.btnDecrement.addEventListener('click', handleDecrement);
    
    // Check connection to API
    checkConnection();
    
    // Periodically check connection (every 30 seconds)
    setInterval(() => {
        if (!state.isConnected) {
            checkConnection();
        }
    }, 30000);
}

// Start the application when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}
