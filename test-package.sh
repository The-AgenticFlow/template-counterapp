#!/bin/bash
# Test Segment 1: Verify package.json exists and has correct dependencies

echo "Testing package.json setup..."

# Test 1: package.json exists
if [ -f "package.json" ]; then
    echo "✓ package.json exists"
else
    echo "✗ package.json not found"
    exit 1
fi

# Test 2: express dependency is listed
if grep -q '"express"' package.json; then
    echo "✓ express dependency listed"
else
    echo "✗ express dependency missing"
    exit 1
fi

# Test 3: cors dependency is listed
if grep -q '"cors"' package.json; then
    echo "✓ cors dependency listed"
else
    echo "✗ cors dependency missing"
    exit 1
fi

# Test 4: start script exists
if grep -q '"start"' package.json; then
    echo "✓ start script defined"
else
    echo "✗ start script missing"
    exit 1
fi

# Test 5: node_modules exists (dependencies installed)
if [ -d "node_modules" ]; then
    echo "✓ node_modules exists (dependencies installed)"
else
    echo "✗ node_modules not found"
    exit 1
fi

# Test 6: express is installed
if [ -d "node_modules/express" ]; then
    echo "✓ express module installed"
else
    echo "✗ express module not installed"
    exit 1
fi

# Test 7: cors is installed
if [ -d "node_modules/cors" ]; then
    echo "✓ cors module installed"
else
    echo "✗ cors module not installed"
    exit 1
fi

echo ""
echo "All Segment 1 tests passed!"
