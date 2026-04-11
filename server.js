// ========================================
// Counter API Backend - Express.js Server
// ========================================
// Segment 2: Server Initialization & Middleware Configuration
// ========================================

// Import required modules
const express = require('express');
const cors = require('cors');

// Initialize Express application
const app = express();
const PORT = 3001;

// ========================================
// Middleware Configuration (Order Matters)
// ========================================

// CORS middleware - MUST be first to inject headers on all responses
app.use(cors());

// JSON body parser - enables POST/PUT request body parsing
app.use(express.json());

// Optional: URL-encoded form data parser
app.use(express.urlencoded({ extended: true }));

// ========================================
// Global Error Handler Middleware
// ========================================
// Catches errors from routes and other middleware
app.use((err, req, res, next) => {
  console.error('[ERROR]', err.message);
  res.status(err.status || 500).json({
    error: err.message || 'Internal Server Error'
  });
});

// ========================================
// Segment 3: State Management & REST Endpoints
// ========================================

// Simple in-memory counter state
let counter = 0;

// ========================================
// REST API Endpoints
// ========================================

// GET /api/counter
// Returns current counter value
app.get('/api/counter', (req, res) => {
  res.status(200).json({
    count: counter
  });
});

// POST /api/counter/increment
// Increments counter by 1 and returns new value
app.post('/api/counter/increment', (req, res) => {
  counter++;
  console.log(`[INCREMENT] Counter: ${counter}`);
  res.status(200).json({
    count: counter
  });
});

// POST /api/counter/decrement
// Decrements counter by 1 with floor constraint at 0
// Returns current value (cannot go below 0)
app.post('/api/counter/decrement', (req, res) => {
  if (counter > 0) {
    counter--;
  }
  console.log(`[DECREMENT] Counter: ${counter}`);
  res.status(200).json({
    count: counter
  });
});

// ========================================
// Server Lifecycle - Binding to Port
// ========================================
// Start listening on port 3001 and log confirmation
app.listen(PORT, () => {
  const timestamp = new Date().toISOString();
  console.log(`[${timestamp}] Server running on port ${PORT}`);
  console.log(`[${timestamp}] API Base: http://localhost:${PORT}/api`);
  console.log(`[${timestamp}] CORS enabled for all origins`);
  console.log(`[${timestamp}] Available endpoints:`);
  console.log(`[${timestamp}]   GET  /api/counter`);
  console.log(`[${timestamp}]   POST /api/counter/increment`);
  console.log(`[${timestamp}]   POST /api/counter/decrement`);
});

// Export app for testing purposes (optional)
module.exports = app;
