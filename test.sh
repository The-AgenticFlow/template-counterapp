#!/bin/bash

BASE_URL="http://localhost:3001"

echo "=== Counter API Test Suite ==="
echo ""

echo "1. GET initial count (expect 0)"
curl -s "$BASE_URL/api/counter" | jq .
echo ""

echo "2. POST increment (expect 1)"
curl -s -X POST "$BASE_URL/api/counter/increment" | jq .
echo ""

echo "3. POST increment again (expect 2)"
curl -s -X POST "$BASE_URL/api/counter/increment" | jq .
echo ""

echo "4. POST decrement (expect 1)"
curl -s -X POST "$BASE_URL/api/counter/decrement" | jq .
echo ""

echo "5. POST decrement to zero (expect 0)"
curl -s -X POST "$BASE_URL/api/counter/decrement" | jq .
echo ""

echo "6. POST decrement below zero (expect 0, not negative)"
curl -s -X POST "$BASE_URL/api/counter/decrement" | jq .
echo ""

echo "7. GET final state (expect 0)"
curl -s "$BASE_URL/api/counter" | jq .
echo ""

echo "=== All tests complete ==="
