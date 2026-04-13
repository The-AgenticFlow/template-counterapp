#!/bin/bash
# Segment 3: Testing and Verification
# Verify all acceptance criteria from CONTRACT.md

echo "========================================"
echo "Segment 3: Testing and Verification"
echo "========================================"
echo ""

# Start server in background
echo "Starting Counter API server..."
node server.js &
SERVER_PID=$!
sleep 2

# Cleanup function
cleanup() {
    echo ""
    echo "Stopping server..."
    kill $SERVER_PID 2>/dev/null
    wait $SERVER_PID 2>/dev/null
}
trap cleanup EXIT

echo ""
echo "========================================"
echo "Acceptance Criteria Tests"
echo "========================================"
echo ""

# AC1: GET /api/counter returns {"count": 0}
echo "AC1: GET /api/counter returns {\"count\": 0}"
echo "  Command: curl http://localhost:3001/api/counter"
RESULT=$(curl -s http://localhost:3001/api/counter)
echo "  Result: $RESULT"
if [ "$RESULT" == '{"count":0}' ]; then
    echo "  Status: ✓ PASS"
else
    echo "  Status: ✗ FAIL"
    exit 1
fi
echo ""

# AC2: POST /api/counter/increment returns {"count": N+1}
echo "AC2: POST /api/counter/increment returns {\"count\": N+1}"
echo "  Incrementing from 0..."
echo "  Command: curl -X POST http://localhost:3001/api/counter/increment"
RESULT=$(curl -s -X POST http://localhost:3001/api/counter/increment)
echo "  Result: $RESULT"
if [ "$RESULT" == '{"count":1}' ]; then
    echo "  Status: ✓ PASS (0 -> 1)"
else
    echo "  Status: ✗ FAIL"
    exit 1
fi
echo ""

# Increment again to verify N+1 behavior
echo "  Incrementing from 1..."
RESULT=$(curl -s -X POST http://localhost:3001/api/counter/increment)
echo "  Result: $RESULT"
if [ "$RESULT" == '{"count":2}' ]; then
    echo "  Status: ✓ PASS (1 -> 2)"
else
    echo "  Status: ✗ FAIL"
    exit 1
fi
echo ""

# AC3: POST /api/counter/decrement returns {"count": max(0, N-1)}
echo "AC3: POST /api/counter/decrement returns {\"count\": max(0, N-1)}"
echo "  Decrementing from 2..."
echo "  Command: curl -X POST http://localhost:3001/api/counter/decrement"
RESULT=$(curl -s -X POST http://localhost:3001/api/counter/decrement)
echo "  Result: $RESULT"
if [ "$RESULT" == '{"count":1}' ]; then
    echo "  Status: ✓ PASS (2 -> 1)"
else
    echo "  Status: ✗ FAIL"
    exit 1
fi
echo ""

# Test floor protection (decrement at 0)
echo "  Testing floor protection..."
# Decrement to 0
curl -s -X POST http://localhost:3001/api/counter/decrement > /dev/null
echo "  Decrementing at 0 (should stay at 0)..."
RESULT=$(curl -s -X POST http://localhost:3001/api/counter/decrement)
echo "  Result: $RESULT"
if [ "$RESULT" == '{"count":0}' ]; then
    echo "  Status: ✓ PASS (floor protection works - stays at 0)"
else
    echo "  Status: ✗ FAIL (floor protection failed)"
    exit 1
fi
echo ""

# AC4: CORS enabled for frontend access
echo "AC4: CORS enabled for frontend access"
echo "  Command: curl -I -H 'Origin: http://example.com' http://localhost:3001/api/counter"
CORS_HEADER=$(curl -s -I -H "Origin: http://example.com" http://localhost:3001/api/counter 2>/dev/null | grep -i "access-control-allow-origin" || true)
echo "  Result: $CORS_HEADER"
if [ -n "$CORS_HEADER" ]; then
    echo "  Status: ✓ PASS (CORS header present)"
else
    echo "  Status: ✗ FAIL (CORS header missing)"
    exit 1
fi
echo ""

# AC5: Port 3001
echo "AC5: Server running on port 3001"
if lsof -i :3001 > /dev/null 2>&1; then
    echo "  Status: ✓ PASS (server listening on port 3001)"
else
    echo "  Status: ✗ FAIL (server not on port 3001)"
    exit 1
fi
echo ""

# AC6: package.json with start script
echo "AC6: package.json with start script"
if [ -f "package.json" ] && grep -q '"start"' package.json; then
    echo "  Status: ✓ PASS (package.json exists with start script)"
else
    echo "  Status: ✗ FAIL"
    exit 1
fi
echo ""

# AC7: server.js created
echo "AC7: server.js created"
if [ -f "server.js" ]; then
    echo "  Status: ✓ PASS (server.js exists)"
else
    echo "  Status: ✗ FAIL"
    exit 1
fi
echo ""

echo "========================================"
echo "All Acceptance Criteria Tests PASSED!"
echo "========================================"
