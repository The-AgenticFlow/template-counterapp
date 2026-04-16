# Counter App Frontend

A vanilla HTML/CSS/JavaScript frontend for the Counter API.

## Prerequisites

- The Counter API backend must be running on port 3001
- A modern web browser
- Python 3 or Node.js (for serving the frontend on port 3000)

## Running the Frontend

### Option 1: Python (recommended)
```bash
# Start the server on port 3000
python3 -m http.server 3000
```

Then open http://localhost:3000 in your browser.

### Option 2: Node.js
```bash
# Install serve globally (if not already installed)
npm install -g serve

# Start the server on port 3000
serve -l 3000
```

Then open http://localhost:3000 in your browser.

### Option 3: Open directly in browser
You can also open `index.html` directly in your browser, but you may encounter CORS issues depending on your browser settings.

## Features

- **Display current count**: Shows the counter value fetched from the API
- **Increment button (+)**: Increases the counter by 1
- **Decrement button (−)**: Decreases the counter by 1 (minimum 0)
- **Loading state**: Shows a spinner during API calls
- **Error handling**: Displays error messages if the API is unreachable

## API Endpoints

The frontend connects to:
- `GET http://localhost:3001/api/counter` - Get current count
- `POST http://localhost:3001/api/counter/increment` - Increment count
- `POST http://localhost:3001/api/counter/decrement` - Decrement count

## Testing

See `test_frontend.sh` for validation tests.

## Port Configuration

- Frontend: Port 3000
- Backend API: Port 3001
