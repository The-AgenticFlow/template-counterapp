#!/bin/bash

# Counter API Integration Tests
# This script tests all endpoints of the Rust Axum Counter API

set -e

BASE_URL="http://localhost:3001"

echo "================================"
echo "Counter API Integration Tests"
echo "================================"
echo ""

# Check if server is running
echo "Checking if server is running on port 3001..."
if ! curl -s "$BASE_URL/api/counter" > /dev/null 2>&1; then
    echo "ERROR: Server is not running on port 3001"
    echo "Please start the server first with: cargo run"
    exit 1
fi
echo "Server is running. Starting tests..."
echo ""

# Test 1: GET initial count (expect 0)
echo "Test 1: GET /api/counter - Initial count should be 0"
RESPONSE=$(curl -s "$BASE_URL/api/counter")
echo "Response: $RESPONSE"
if echo "$RESPONSE" | grep -q '"count":0'; then
    echo "✅ PASS: Initial count is 0"
else
    echo "❌ FAIL: Expected count to be 0"
    exit 1
fi
echo ""

# Test 2: POST increment (expect 1)
echo "Test 2: POST /api/counter/increment - Should return 1"
RESPONSE=$(curl -s -X POST "$BASE_URL/api/counter/increment")
echo "Response: $RESPONSE"
if echo "$RESPONSE" | grep -q '"count":1'; then
    echo "✅ PASS: Count incremented to 1"
else
    echo "❌ FAIL: Expected count to be 1"
    exit 1
fi
echo ""

# Test 3: POST increment again (expect 2)
echo "Test 3: POST /api/counter/increment - Should return 2"
RESPONSE=$(curl -s -X POST "$BASE_URL/api/counter/increment")
echo "Response: $RESPONSE"
if echo "$RESPONSE" | grep -q '"count":2'; then
    echo "✅ PASS: Count incremented to 2"
else
    echo "❌ FAIL: Expected count to be 2"
    exit 1
fi
echo ""

# Test 4: POST decrement (expect 1)
echo "Test 4: POST /api/counter/decrement - Should return 1"
RESPONSE=$(curl -s -X POST "$BASE_URL/api/counter/decrement")
echo "Response: $RESPONSE"
if echo "$RESPONSE" | grep -q '"count":1'; then
    echo "✅ PASS: Count decremented to 1"
else
    echo "❌ FAIL: Expected count to be 1"
    exit 1
fi
echo ""

# Test 5: POST decrement to zero (expect 0)
echo "Test 5: POST /api/counter/decrement - Should return 0"
RESPONSE=$(curl -s -X POST "$BASE_URL/api/counter/decrement")
echo "Response: $RESPONSE"
if echo "$RESPONSE" | grep -q '"count":0'; then
    echo "✅ PASS: Count decremented to 0"
else
    echo "❌ FAIL: Expected count to be 0"
    exit 1
fi
echo ""

# Test 6: POST decrement below zero (expect 0, not negative)
echo "Test 6: POST /api/counter/decrement - Should stay at 0 (no negative)"
RESPONSE=$(curl -s -X POST "$BASE_URL/api/counter/decrement")
echo "Response: $RESPONSE"
if echo "$RESPONSE" | grep -q '"count":0'; then
    echo "✅ PASS: Count stays at 0 (no negative values)"
else
    echo "❌ FAIL: Expected count to remain 0"
    exit 1
fi
echo ""

# Test 7: GET final state (expect 0)
echo "Test 7: GET /api/counter - Final count should be 0"
RESPONSE=$(curl -s "$BASE_URL/api/counter")
echo "Response: $RESPONSE"
if echo "$RESPONSE" | grep -q '"count":0'; then
    echo "✅ PASS: Final count is 0"
else
    echo "❌ FAIL: Expected final count to be 0"
    exit 1
fi
echo ""

echo "================================"
echo "All 7 tests passed! ✅"
echo "================================"
