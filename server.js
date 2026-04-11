// ============================================================
// Counter API Backend - Express.js Server Implementation
// Task: T-005
// Date: 2026-04-11
// ============================================================

// SEGMENT 2: Server Initialization & Middleware Configuration
// Lines 1-51

// Import required modules
const express = require('express');
const cors = require('cors');

// Initialize Express application
const app = express();
const PORT = 3001;

// ============================================================
// Middleware Configuration (Proper Ordering)
// ============================================================

// 1. CORS Middleware (MUST be first to inject headers on all responses)
app.use(cors({
  origin: '*',
  credentials: false,
  methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
  allowedHeaders: ['Content-Type', 'Authorization']
}));

// 2. JSON Body Parser Middleware (for parsing application/json requests)
app.use(express.json());

// 3. URL-encoded Form Data Parser Middleware (for form submissions)
app.use(express.urlencoded({ extended: true }));

// ============================================================
// Global Error Handler Middleware
// ============================================================

// Error handling middleware (catches errors in route handlers)
app.use((err, req, res, next) => {
  console.error(`[ERROR] ${new Date().toISOString()} - ${err.message}`);
  
  // Return error response to client
  res.status(err.status || 500).json({
    error: err.message || 'Internal Server Error',
    status: err.status || 500,
    timestamp: new Date().toISOString()
  });
});

// ============================================================
// SEGMENT 3: State Management & REST Endpoint Implementation
// Lines 52-83
// ============================================================

// In-memory counter state (simple counter variable)
// Acts as single source of truth for counter value
let count = 0;

// ============================================================
// REST API Endpoints
// ============================================================

// GET /api/counter
// Returns the current counter value in JSON format
app.get('/api/counter', (req, res) => {
  console.log(`[GET] /api/counter - Current count: ${count}`);
  res.status(200).json({ count });
});

// POST /api/counter/increment
// Increments the counter by 1 and returns the new value
app.post('/api/counter/increment', (req, res) => {
  count++;
  console.log(`[POST] /api/counter/increment - New count: ${count}`);
  res.status(200).json({ count });
});

// POST /api/counter/decrement
// Decrements the counter by 1 with floor constraint at 0
// If count is already 0, it stays at 0 (floor enforcement)
app.post('/api/counter/decrement', (req, res) => {
  if (count > 0) {
    count--;
  }
  // If count === 0, decrement does nothing (floor constraint)
  console.log(`[POST] /api/counter/decrement - New count: ${count}`);
  res.status(200).json({ count });
});

// ============================================================
// Server Lifecycle & Logging
// ============================================================

// Start the server and listen on the specified PORT
const server = app.listen(PORT, () => {
  console.log(`[${new Date().toISOString()}] Server running on port ${PORT}`);
  console.log(`[${new Date().toISOString()}] API Base: http://localhost:${PORT}/api`);
  console.log(`[${new Date().toISOString()}] CORS enabled for all origins`);
  console.log(`[${new Date().toISOString()}] Available endpoints:`);
  console.log(`[${new Date().toISOString()}]   - GET  /api/counter`);
  console.log(`[${new Date().toISOString()}]   - POST /api/counter/increment`);
  console.log(`[${new Date().toISOString()}]   - POST /api/counter/decrement`);
});

// Graceful shutdown handling
process.on('SIGTERM', () => {
  console.log(`[${new Date().toISOString()}] SIGTERM received, closing server gracefully`);
  server.close(() => {
    console.log(`[${new Date().toISOString()}] Server closed`);
    process.exit(0);
  });
});

module.exports = app;
