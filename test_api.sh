#!/bin/bash

# Counter API Test Script
# Tests all endpoints of the Counter API

BASE_URL="http://localhost:3001"

echo "=== Counter API Tests ==="
echo ""

# Check if server is running
if ! curl -s "${BASE_URL}/api/counter" > /dev/null 2>&1; then
    echo "ERROR: Server is not running on ${BASE_URL}"
    echo "Please start the server first with: cargo run"
    exit 1
fi

echo "1. Testing GET /api/counter (should return count: 0)"
RESULT=$(curl -s "${BASE_URL}/api/counter")
echo "   Response: ${RESULT}"
if echo "${RESULT}" | grep -q '"count":0'; then
    echo "   ✓ PASS"
else
    echo "   ✗ FAIL"
fi
echo ""

echo "2. Testing POST /api/counter/increment (should return count: 1)"
RESULT=$(curl -s -X POST "${BASE_URL}/api/counter/increment")
echo "   Response: ${RESULT}"
if echo "${RESULT}" | grep -q '"count":1'; then
    echo "   ✓ PASS"
else
    echo "   ✗ FAIL"
fi
echo ""

echo "3. Testing POST /api/counter/increment again (should return count: 2)"
RESULT=$(curl -s -X POST "${BASE_URL}/api/counter/increment")
echo "   Response: ${RESULT}"
if echo "${RESULT}" | grep -q '"count":2'; then
    echo "   ✓ PASS"
else
    echo "   ✗ FAIL"
fi
echo ""

echo "4. Testing POST /api/counter/decrement (should return count: 1)"
RESULT=$(curl -s -X POST "${BASE_URL}/api/counter/decrement")
echo "   Response: ${RESULT}"
if echo "${RESULT}" | grep -q '"count":1'; then
    echo "   ✓ PASS"
else
    echo "   ✗ FAIL"
fi
echo ""

echo "5. Testing POST /api/counter/decrement again (should return count: 0)"
RESULT=$(curl -s -X POST "${BASE_URL}/api/counter/decrement")
echo "   Response: ${RESULT}"
if echo "${RESULT}" | grep -q '"count":0'; then
    echo "   ✓ PASS"
else
    echo "   ✗ FAIL"
fi
echo ""

echo "6. Testing POST /api/counter/decrement at 0 (should still return count: 0)"
RESULT=$(curl -s -X POST "${BASE_URL}/api/counter/decrement")
echo "   Response: ${RESULT}"
if echo "${RESULT}" | grep -q '"count":0'; then
    echo "   ✓ PASS"
else
    echo "   ✗ FAIL"
fi
echo ""

echo "7. Testing GET /api/counter (should confirm count: 0)"
RESULT=$(curl -s "${BASE_URL}/api/counter")
echo "   Response: ${RESULT}"
if echo "${RESULT}" | grep -q '"count":0'; then
    echo "   ✓ PASS"
else
    echo "   ✗ FAIL"
fi
echo ""

echo "=== All tests completed ==="
