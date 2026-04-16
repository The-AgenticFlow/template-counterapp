#!/bin/bash

# Test script for Counter Frontend

echo "=== Counter Frontend Validation Tests ==="
echo ""

FRONTEND_PORT=3000
BACKEND_PORT=3001

# Test 1: Check that index.html exists and is valid HTML
echo "Test 1: Checking index.html exists and is valid..."
if [ ! -f "index.html" ]; then
    echo "❌ FAIL: index.html not found"
    exit 1
fi

# Check for essential HTML elements
if ! grep -q '<!DOCTYPE html>' index.html; then
    echo "❌ FAIL: Missing DOCTYPE declaration"
    exit 1
fi

if ! grep -q 'id="counter"' index.html; then
    echo "❌ FAIL: Missing counter display element"
    exit 1
fi

if ! grep -q 'id="decrementBtn"' index.html; then
    echo "❌ FAIL: Missing decrement button"
    exit 1
fi

if ! grep -q 'id="incrementBtn"' index.html; then
    echo "❌ FAIL: Missing increment button"
    exit 1
fi

if ! grep -q 'id="loading"' index.html; then
    echo "❌ FAIL: Missing loading indicator"
    exit 1
fi

if ! grep -q 'id="error"' index.html; then
    echo "❌ FAIL: Missing error display element"
    exit 1
fi

echo "✅ PASS: index.html is valid and contains all required elements"
echo ""

# Test 2: Check for JavaScript functions
echo "Test 2: Checking JavaScript functions..."
if ! grep -q 'function increment()' index.html; then
    echo "❌ FAIL: Missing increment function"
    exit 1
fi

if ! grep -q 'function decrement()' index.html; then
    echo "❌ FAIL: Missing decrement function"
    exit 1
fi

if ! grep -q 'function fetchCounter()' index.html; then
    echo "❌ FAIL: Missing fetchCounter function"
    exit 1
fi

echo "✅ PASS: All required JavaScript functions present"
echo ""

# Test 3: Check API endpoint references
echo "Test 3: Checking API endpoint references..."
if ! grep -q 'localhost:3001' index.html; then
    echo "❌ FAIL: Missing backend API URL reference"
    exit 1
fi

if ! grep -q '/api/counter' index.html; then
    echo "❌ FAIL: Missing API endpoint reference"
    exit 1
fi

if ! grep -q '/increment' index.html; then
    echo "❌ FAIL: Missing increment endpoint reference"
    exit 1
fi

if ! grep -q '/decrement' index.html; then
    echo "❌ FAIL: Missing decrement endpoint reference"
    exit 1
fi

echo "✅ PASS: All API endpoints correctly referenced"
echo ""

# Test 4: Check for CSS styling
echo "Test 4: Checking CSS styling..."
if ! grep -q '<style>' index.html; then
    echo "❌ FAIL: Missing CSS styles"
    exit 1
fi

if ! grep -q '\.decrement-btn' index.html; then
    echo "❌ FAIL: Missing decrement button styling"
    exit 1
fi

if ! grep -q '\.increment-btn' index.html; then
    echo "❌ FAIL: Missing increment button styling"
    exit 1
fi

echo "✅ PASS: CSS styling present"
echo ""

# Test 5: Check README exists
echo "Test 5: Checking README..."
if [ ! -f "README.md" ]; then
    echo "❌ FAIL: README.md not found"
    exit 1
fi

echo "✅ PASS: README.md exists"
echo ""

echo "=== All Tests Passed ==="
echo ""
echo "Manual Testing Checklist:"
echo "1. Start the backend API on port 3001"
echo "2. Start this frontend server: python3 -m http.server 3000"
echo "3. Open http://localhost:3000 in browser"
echo "4. Verify:"
echo "   [ ] Counter displays initial value from API"
echo "   [ ] Increment button (+) increases the count"
echo "   [ ] Decrement button (−) decreases the count (min 0)"
echo "   [ ] Loading spinner shows during API calls"
echo "   [ ] Error message shows if backend is unavailable"
echo "   [ ] Buttons are disabled during loading"
