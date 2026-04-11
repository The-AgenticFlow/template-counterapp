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
// Server Lifecycle - Binding to Port
// ========================================
// Start listening on port 3001 and log confirmation
app.listen(PORT, () => {
  const timestamp = new Date().toISOString();
  console.log(`[${timestamp}] Server running on port ${PORT}`);
  console.log(`[${timestamp}] API Base: http://localhost:${PORT}/api`);
  console.log(`[${timestamp}] CORS enabled for all origins`);
});

// Export app for testing purposes (optional)
module.exports = app;
