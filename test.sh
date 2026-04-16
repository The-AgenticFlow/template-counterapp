#!/bin/bash

set -e

BASE_URL="http://localhost:3001"

echo "========================================="
echo "Counter API Test Suite"
echo "========================================="
echo ""

# Test 1: GET /api/counter - Should return count: 0
echo "Test 1: GET /api/counter (initial value should be 0)"
response=$(curl -s "$BASE_URL/api/counter")
echo "Response: $response"
if [[ "$response" == '{"count":0}' ]]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Expected {\"count\":0}, got $response"
    exit 1
fi
echo ""

# Test 2: POST /api/counter/increment - Should increment count
echo "Test 2: POST /api/counter/increment"
response=$(curl -s -X POST "$BASE_URL/api/counter/increment")
echo "Response: $response"
if [[ "$response" == '{"count":1}' ]]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Expected {\"count\":1}, got $response"
    exit 1
fi
echo ""

# Test 3: POST /api/counter/increment again
echo "Test 3: POST /api/counter/increment (again)"
response=$(curl -s -X POST "$BASE_URL/api/counter/increment")
echo "Response: $response"
if [[ "$response" == '{"count":2}' ]]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Expected {\"count\":2}, got $response"
    exit 1
fi
echo ""

# Test 4: POST /api/counter/decrement - Should decrement count
echo "Test 4: POST /api/counter/decrement"
response=$(curl -s -X POST "$BASE_URL/api/counter/decrement")
echo "Response: $response"
if [[ "$response" == '{"count":1}' ]]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Expected {\"count\":1}, got $response"
    exit 1
fi
echo ""

# Test 5: POST /api/counter/decrement to zero
echo "Test 5: POST /api/counter/decrement (back to 0)"
response=$(curl -s -X POST "$BASE_URL/api/counter/decrement")
echo "Response: $response"
if [[ "$response" == '{"count":0}' ]]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Expected {\"count\":0}, got $response"
    exit 1
fi
echo ""

# Test 6: POST /api/counter/decrement - Should not go below 0
echo "Test 6: POST /api/counter/decrement (should stay at 0, not go negative)"
response=$(curl -s -X POST "$BASE_URL/api/counter/decrement")
echo "Response: $response"
if [[ "$response" == '{"count":0}' ]]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Expected {\"count\":0}, got $response"
    exit 1
fi
echo ""

# Test 7: GET /api/counter - Verify final state
echo "Test 7: GET /api/counter (verify final state)"
response=$(curl -s "$BASE_URL/api/counter")
echo "Response: $response"
if [[ "$response" == '{"count":0}' ]]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Expected {\"count\":0}, got $response"
    exit 1
fi
echo ""

echo "========================================="
echo "All tests passed! ✓"
echo "========================================="