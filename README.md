# Counter App

A simple counter application with a vanilla HTML/JS frontend and Rust Axum backend.

## Architecture

- **Frontend**: Single HTML file with embedded CSS and JavaScript (port 3000)
- **Backend**: Rust Axum API server (port 3001)

## Running the Application

### Prerequisites

1. Backend API running on port 3001 (see backend implementation)
2. A static file server for the frontend

### Option 1: Python HTTP Server

```bash
# From this directory
python3 -m http.server 3000
```

Then open http://localhost:3000 in your browser.

### Option 2: Node.js serve

```bash
# Install serve globally if needed
npm install -g serve

# Run on port 3000
npx serve -p 3000
```

Then open http://localhost:3000 in your browser.

### Option 3: Live Server (VS Code)

If using VS Code, install the "Live Server" extension and right-click on `index.html` → "Open with Live Server" (configure to use port 3000 in settings).

## Features

- Display current count from API
- Increment button (+)
- Decrement button (-)
- Loading state during API calls
- Error handling for failed requests

## API Endpoints

The frontend connects to the following backend endpoints:

- `GET http://localhost:3001/api/counter` - Get current count
- `POST http://localhost:3001/api/counter/increment` - Increment count
- `POST http://localhost:3001/api/counter/decrement` - Decrement count

## Tech Stack

- Vanilla HTML/JS (single file: index.html)
- CSS for styling
- Fetch API for backend communication
- No build tools required
