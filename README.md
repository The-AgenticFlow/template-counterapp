# Counter App

A simple counter application with a modern web UI and REST API backend.

## Overview

This project consists of two parts:

1. **Frontend** - Static HTML/CSS/JS application deployed to GitHub Pages
2. **Backend** - Node.js/Express REST API (separate deployment)

## Features

- 🎨 Modern, responsive UI design
- ➕ Increment and ➖ Decrement counter operations
- 🔌 Real-time connection status indicator
- 📱 Mobile-friendly responsive layout
- ⚡ Graceful error handling when backend unavailable
- 🌐 CORS-enabled API

## Quick Start

### Prerequisites

- Node.js 18+ (for backend)
- Modern web browser

### Running Locally

1. **Start the Backend API**
   ```bash
   # Clone the repository
   git clone https://github.com/The-AgenticFlow/template-counterapp.git
   cd template-counterapp
   
   # Install dependencies
   npm install
   
   # Start the server
   npm start
   ```
   The API will run on http://localhost:3001

2. **Open the Frontend**
   - Open `index.html` in your browser
   - Or serve via a local server:
     ```bash
     npx serve .
     ```

### Live Demo

The frontend is deployed at: **https://the-agenticflow.github.io/template-counterapp/**

> Note: The backend API needs to be deployed separately for the live demo to work.

## API Endpoints

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| GET | `/api/counter` | Get current counter value | `{"count": N}` |
| POST | `/api/counter/increment` | Increment counter by 1 | `{"count": N+1}` |
| POST | `/api/counter/decrement` | Decrement counter (min 0) | `{"count": max(0, N-1)}` |

## Project Structure

```
├── index.html      # Main HTML page
├── style.css       # Stylesheet with responsive design
├── app.js          # JavaScript application logic
├── .nojekyll       # GitHub Pages configuration
├── server.js       # Express.js backend server
├── package.json    # Node.js dependencies
└── TESTING.md      # API testing documentation
```

## Configuration

### Frontend API URL

The frontend auto-detects the environment:

- **Local development**: Uses `http://localhost:3001`
- **Production**: Configure in `app.js`:
  ```javascript
  // Update this URL when backend is deployed
  return 'https://your-backend-url.herokuapp.com';
  ```

### Environment Variables (Backend)

| Variable | Default | Description |
|----------|---------|-------------|
| PORT | 3001 | Server port |

## Deployment

### GitHub Pages (Frontend)

1. Push changes to the `main` branch
2. Go to Repository Settings → Pages
3. Select Source: `main` branch, `/ (root)` folder
4. Site will be live at `https://[username].github.io/[repo-name]/`

### Backend Deployment

The Express.js backend can be deployed to:
- Heroku
- Railway
- Render
- Fly.io
- Any Node.js hosting provider

Make sure to update the frontend's API URL configuration.

## Testing

### Manual Testing with curl

See [TESTING.md](TESTING.md) for detailed API testing instructions.

### Frontend Testing

1. Start the backend server
2. Open `index.html` in browser
3. Test increment and decrement buttons
4. Verify connection status indicator
5. Test error handling (stop backend, verify graceful degradation)

## Browser Support

- Chrome 80+
- Firefox 75+
- Safari 13+
- Edge 80+

## Technologies

- **Frontend**: HTML5, CSS3, Vanilla JavaScript (ES6+)
- **Backend**: Node.js, Express.js
- **API**: REST with JSON responses

## License

MIT

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

---

Built with ❤️ by The AgenticFlow
