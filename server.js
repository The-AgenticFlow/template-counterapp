/**
 * Counter API Backend
 * Simple REST API for a counter application
 * 
 * Endpoints:
 * - GET /api/counter - Get current count
 * - POST /api/counter/increment - Increment count
 * - POST /api/counter/decrement - Decrement count (min: 0)
 * 
 * Port: 3001
 */

const express = require('express');
const cors = require('cors');

const app = express();
const PORT = process.env.PORT || 3001;

// Enable CORS for all origins (allows frontend access)
app.use(cors());

// In-memory counter storage
let count = 0;

/**
 * GET /api/counter
 * Returns the current counter value
 */
app.get('/api/counter', (req, res) => {
    res.status(200).json({ count });
});

/**
 * POST /api/counter/increment
 * Increments the counter by 1
 */
app.post('/api/counter/increment', (req, res) => {
    count = count + 1;
    res.status(200).json({ count });
});

/**
 * POST /api/counter/decrement
 * Decrements the counter by 1, but never below 0
 */
app.post('/api/counter/decrement', (req, res) => {
    count = Math.max(0, count - 1);
    res.status(200).json({ count });
});

// Start server
app.listen(PORT, () => {
    console.log(`Counter API server running on port ${PORT}`);
    console.log(`Endpoints:`);
    console.log(`  GET  http://localhost:${PORT}/api/counter`);
    console.log(`  POST http://localhost:${PORT}/api/counter/increment`);
    console.log(`  POST http://localhost:${PORT}/api/counter/decrement`);
});
