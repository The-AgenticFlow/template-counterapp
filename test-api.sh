#!/bin/bash
# Test Segment 2: API Server Implementation

echo "Testing Counter API server..."
echo ""

# Start server in background
echo "Starting server..."
node server.js &
SERVER_PID=$!
sleep 2

# Function to cleanup on exit
cleanup() {
    echo ""
    echo "Stopping server..."
    kill $SERVER_PID 2>/dev/null
    wait $SERVER_PID 2>/dev/null
}
trap cleanup EXIT

# Test 1: GET /api/counter should return {"count": 0}
echo "Test 1: GET /api/counter (initial value)"
RESULT=$(curl -s http://localhost:3001/api/counter)
if [ "$RESULT" == '{"count":0}' ]; then
    echo "✓ GET /api/counter returns {\"count\": 0}"
else
    echo "✗ GET /api/counter returned: $RESULT (expected {\"count\":0})"
    exit 1
fi

# Test 2: POST /api/counter/increment should return {"count": 1}
echo "Test 2: POST /api/counter/increment"
RESULT=$(curl -s -X POST http://localhost:3001/api/counter/increment)
if [ "$RESULT" == '{"count":1}' ]; then
    echo "✓ POST /api/counter/increment returns {\"count\": 1}"
else
    echo "✗ POST /api/counter/increment returned: $RESULT (expected {\"count\":1})"
    exit 1
fi

# Test 3: POST /api/counter/increment again should return {"count": 2}
echo "Test 3: POST /api/counter/increment (second time)"
RESULT=$(curl -s -X POST http://localhost:3001/api/counter/increment)
if [ "$RESULT" == '{"count":2}' ]; then
    echo "✓ POST /api/counter/increment returns {\"count\": 2}"
else
    echo "✗ POST /api/counter/increment returned: $RESULT (expected {\"count\":2})"
    exit 1
fi

# Test 4: GET /api/counter should now return {"count": 2}
echo "Test 4: GET /api/counter (after increments)"
RESULT=$(curl -s http://localhost:3001/api/counter)
if [ "$RESULT" == '{"count":2}' ]; then
    echo "✓ GET /api/counter returns {\"count\": 2}"
else
    echo "✗ GET /api/counter returned: $RESULT (expected {\"count\":2})"
    exit 1
fi

# Test 5: POST /api/counter/decrement should return {"count": 1}
echo "Test 5: POST /api/counter/decrement"
RESULT=$(curl -s -X POST http://localhost:3001/api/counter/decrement)
if [ "$RESULT" == '{"count":1}' ]; then
    echo "✓ POST /api/counter/decrement returns {\"count\": 1}"
else
    echo "✗ POST /api/counter/decrement returned: $RESULT (expected {\"count\":1})"
    exit 1
fi

# Test 6: Decrement again to 0
echo "Test 6: POST /api/counter/decrement (to 0)"
RESULT=$(curl -s -X POST http://localhost:3001/api/counter/decrement)
if [ "$RESULT" == '{"count":0}' ]; then
    echo "✓ POST /api/counter/decrement returns {\"count\": 0}"
else
    echo "✗ POST /api/counter/decrement returned: $RESULT (expected {\"count\":0})"
    exit 1
fi

# Test 7: Decrement at 0 should stay at 0 (floor protection)
echo "Test 7: POST /api/counter/decrement (floor protection - should stay at 0)"
RESULT=$(curl -s -X POST http://localhost:3001/api/counter/decrement)
if [ "$RESULT" == '{"count":0}' ]; then
    echo "✓ Floor protection works - count stays at 0"
else
    echo "✗ Floor protection failed - returned: $RESULT (expected {\"count\":0})"
    exit 1
fi

# Test 8: CORS headers present
echo "Test 8: CORS headers check"
HEADERS=$(curl -s -I -H "Origin: http://example.com" http://localhost:3001/api/counter 2>/dev/null | grep -i "access-control-allow-origin" || true)
if [ -n "$HEADERS" ]; then
    echo "✓ CORS headers present: $HEADERS"
else
    echo "✗ CORS headers missing"
    exit 1
fi

echo ""
echo "All Segment 2 tests passed!"
