# Counter API Testing Guide

## Prerequisites

1. Install dependencies:
   ```bash
   npm install
   ```

2. Start the server:
   ```bash
   npm start
   ```

## Testing with curl

### 1. Get initial counter value

```bash
curl -X GET http://localhost:3001/api/counter
```

**Expected Response:**
```json
{"count":0}
```

---

### 2. Increment the counter

```bash
curl -X POST http://localhost:3001/api/counter/increment
```

**Expected Response:**
```json
{"count":1}
```

---

### 3. Increment again

```bash
curl -X POST http://localhost:3001/api/counter/increment
```

**Expected Response:**
```json
{"count":2}
```

---

### 4. Decrement the counter

```bash
curl -X POST http://localhost:3001/api/counter/decrement
```

**Expected Response:**
```json
{"count":1}
```

---

### 5. Decrement to 0

```bash
curl -X POST http://localhost:3001/api/counter/decrement
```

**Expected Response:**
```json
{"count":0}
```

---

### 6. Verify counter cannot go below 0

```bash
curl -X POST http://localhost:3001/api/counter/decrement
```

**Expected Response:**
```json
{"count":0}
```

---

### 7. Verify CORS headers

```bash
curl -I -X GET http://localhost:3001/api/counter
```

**Expected Headers:**
```
Access-Control-Allow-Origin: *
```

---

## Complete Test Sequence

Run this sequence to verify all functionality:

```bash
# Start fresh (restart server to reset counter)
# Terminal 1: npm start

# Terminal 2: Run tests
curl -X GET http://localhost:3001/api/counter
curl -X POST http://localhost:3001/api/counter/increment
curl -X POST http://localhost:3001/api/counter/increment
curl -X POST http://localhost:3001/api/counter/increment
curl -X GET http://localhost:3001/api/counter
curl -X POST http://localhost:3001/api/counter/decrement
curl -X POST http://localhost:3001/api/counter/decrement
curl -X POST http://localhost:3001/api/counter/decrement
curl -X POST http://localhost:3001/api/counter/decrement  # Should stay at 0
curl -I -X GET http://localhost:3001/api/counter
```

---

## Troubleshooting

- **Port already in use:** Kill any process using port 3001
  ```bash
  lsof -i :3001
  kill -9 <PID>
  ```

- **Connection refused:** Ensure server is running with `npm start`
