const express = require('express');
const cors = require('cors');

const app = express();
const PORT = process.env.PORT || 3001;

// Enable CORS for all origins
app.use(cors());

// In-memory counter
let count = 0;

// GET /api/counter - Returns current count
app.get('/api/counter', (req, res) => {
  res.json({ count });
});

// POST /api/counter/increment - Increments count by 1
app.post('/api/counter/increment', (req, res) => {
  count += 1;
  res.json({ count });
});

// POST /api/counter/decrement - Decrements count with floor protection
app.post('/api/counter/decrement', (req, res) => {
  count = Math.max(0, count - 1);
  res.json({ count });
});

// Start server
app.listen(PORT, () => {
  console.log(`Counter API server running on port ${PORT}`);
});
