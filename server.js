// ============================================================================
// Counter API Backend - Express.js Server
// ============================================================================

const express = require('express');
const cors = require('cors');

// Initialize Express app
const app = express();
const PORT = 3001;

// ============================================================================
// SEGMENT 2: Server Initialization & Middleware Configuration
// ============================================================================

// Middleware: CORS (must be configured first to inject headers on all responses)
app.use(cors());

// Middleware: JSON body parser for POST/PUT requests
app.use(express.json());

// Middleware: URL-encoded form data parser
app.use(express.urlencoded({ extended: true }));

// Middleware: Global error handler
app.use((err, req, res, next) => {
  console.error(`[ERROR] ${new Date().toISOString()} - ${err.message}`);
  res.status(500).json({ error: 'Internal server error' });
});

// ============================================================================
// SEGMENT 3: State Management & REST Endpoint Implementation
// ============================================================================

// In-memory counter state (single source of truth)
let counter = 0;

// GET /api/counter - Return current counter value
app.get('/api/counter', (req, res) => {
  console.log(`[GET] /api/counter - Count: ${counter}`);
  res.json({ count: counter });
});

// POST /api/counter/increment - Increment counter by 1
app.post('/api/counter/increment', (req, res) => {
  counter += 1;
  console.log(`[POST] /api/counter/increment - New Count: ${counter}`);
  res.json({ count: counter });
});

// POST /api/counter/decrement - Decrement counter by 1 (with floor constraint at 0)
app.post('/api/counter/decrement', (req, res) => {
  if (counter > 0) {
    counter -= 1;
  }
  console.log(`[POST] /api/counter/decrement - New Count: ${counter}`);
  res.json({ count: counter });
});

// ============================================================================
// Server Lifecycle
// ============================================================================

app.listen(PORT, () => {
  console.log(`[STARTUP] ${new Date().toISOString()}`);
  console.log(`[SERVER] Counter API server listening on port ${PORT}`);
  console.log(`[CORS] Cross-Origin Resource Sharing enabled`);
  console.log(`[ENDPOINTS] Available endpoints:`);
  console.log(`  • GET    /api/counter`);
  console.log(`  • POST   /api/counter/increment`);
  console.log(`  • POST   /api/counter/decrement`);
});
