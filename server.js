const express = require('express');
const cors = require('cors');

const app = express();
const PORT = 3001;

// Enable CORS for all origins
app.use(cors());

// In-memory counter storage
let counter = 0;

// GET /api/counter - Returns current counter value
app.get('/api/counter', (req, res) => {
  res.json({ count: counter });
});

// POST /api/counter/increment - Increments counter by 1
app.post('/api/counter/increment', (req, res) => {
  counter += 1;
  res.json({ count: counter });
});

// POST /api/counter/decrement - Decrements counter (minimum 0)
app.post('/api/counter/decrement', (req, res) => {
  counter = Math.max(0, counter - 1);
  res.json({ count: counter });
});

// Start server
app.listen(PORT, () => {
  console.log(`Counter API server running on port ${PORT}`);
});
