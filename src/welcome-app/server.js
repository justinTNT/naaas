const express = require('express');
const os = require('os');

const app = express();
const port = process.env.PORT || 3000;
const hostname = os.hostname();
const startTime = Date.now();

// Middleware to log requests
app.use((req, res, next) => {
    console.log(`${new Date().toISOString()} - ${req.method} ${req.path} from ${req.ip}`);
    next();
});

// Main welcome page
app.get('/', (req, res) => {
    const uptime = Math.floor((Date.now() - startTime) / 1000);
    
    const html = `
    <!DOCTYPE html>
    <html>
    <head>
        <title>NAAAS Welcome App</title>
        <style>
            body { 
                font-family: Arial, sans-serif; 
                margin: 40px; 
                background: #f5f5f5;
                color: #333;
            }
            .container { 
                max-width: 800px; 
                margin: 0 auto; 
                background: white; 
                padding: 30px; 
                border-radius: 8px;
                box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            }
            h1 { 
                color: #2c5282; 
                border-bottom: 3px solid #4299e1;
                padding-bottom: 10px;
            }
            .status { 
                background: #48bb78; 
                color: white; 
                padding: 10px 20px; 
                border-radius: 5px;
                display: inline-block;
                margin: 10px 0;
            }
            .info-section {
                margin: 20px 0;
                padding: 15px;
                background: #f7fafc;
                border-left: 4px solid #4299e1;
            }
            .info-grid {
                display: grid;
                grid-template-columns: 150px 1fr;
                gap: 10px;
                margin: 10px 0;
            }
            .label { font-weight: bold; }
            pre { 
                background: #2d3748; 
                color: #e2e8f0; 
                padding: 15px; 
                border-radius: 5px;
                overflow-x: auto;
                font-size: 14px;
            }
        </style>
    </head>
    <body>
        <div class="container">
            <h1>🎉 Welcome to NAAAS!</h1>
            <div class="status">✅ Unikernel is running successfully</div>
            
            <div class="info-section">
                <h3>System Information</h3>
                <div class="info-grid">
                    <span class="label">Hostname:</span>
                    <span><strong>${hostname}</strong></span>
                    
                    <span class="label">Port:</span>
                    <span>${port}</span>
                    
                    <span class="label">Uptime:</span>
                    <span>${uptime} seconds</span>
                    
                    <span class="label">Node Version:</span>
                    <span>${process.version}</span>
                    
                    <span class="label">Platform:</span>
                    <span>${os.platform()}</span>
                    
                    <span class="label">Architecture:</span>
                    <span>${os.arch()}</span>
                </div>
            </div>
            
            <div class="info-section">
                <h3>Request Headers</h3>
                <pre>${JSON.stringify(req.headers, null, 2)}</pre>
            </div>
            
            <div class="info-section">
                <h3>Environment Variables</h3>
                <pre>${JSON.stringify({
                    NODE_ENV: process.env.NODE_ENV || 'not set',
                    PORT: process.env.PORT || 'not set',
                    NAAAS_TENANT: process.env.NAAAS_TENANT || 'not set',
                    NAAAS_REGION: process.env.NAAAS_REGION || 'not set'
                }, null, 2)}</pre>
            </div>
            
            <div class="info-section">
                <h3>Memory Usage</h3>
                <div class="info-grid">
                    <span class="label">RSS:</span>
                    <span>${Math.round(process.memoryUsage().rss / 1024 / 1024)} MB</span>
                    
                    <span class="label">Heap Used:</span>
                    <span>${Math.round(process.memoryUsage().heapUsed / 1024 / 1024)} MB</span>
                    
                    <span class="label">External:</span>
                    <span>${Math.round(process.memoryUsage().external / 1024 / 1024)} MB</span>
                </div>
            </div>
            
            <div class="info-section">
                <p><strong>This is a minimal Express.js application running as a unikernel for NAAAS development and testing.</strong></p>
                <p>If you can see this page, your NAAAS deployment is working correctly!</p>
            </div>
        </div>
    </body>
    </html>`;
    
    res.send(html);
});

// Health check endpoint
app.get('/health', (req, res) => {
    res.json({
        status: 'healthy',
        hostname: hostname,
        uptime: Math.floor((Date.now() - startTime) / 1000),
        timestamp: new Date().toISOString(),
        service: 'naaas-welcome-app'
    });
});

// API info endpoint
app.get('/api/info', (req, res) => {
    res.json({
        hostname: hostname,
        port: port,
        uptime: Math.floor((Date.now() - startTime) / 1000),
        memory: process.memoryUsage(),
        node_version: process.version,
        platform: os.platform(),
        architecture: os.arch(),
        environment: {
            NODE_ENV: process.env.NODE_ENV,
            NAAAS_TENANT: process.env.NAAAS_TENANT,
            NAAAS_REGION: process.env.NAAAS_REGION
        },
        headers: req.headers
    });
});

// Start the server
app.listen(port, () => {
    console.log(`🚀 NAAAS Welcome App started successfully!`);
    console.log(`📍 Hostname: ${hostname}`);
    console.log(`🌐 Port: ${port}`);
    console.log(`📊 Memory: ${Math.round(process.memoryUsage().rss / 1024 / 1024)} MB`);
    console.log(`⏰ Started at: ${new Date().toISOString()}`);
    console.log(`🔗 Local URL: http://localhost:${port}`);
    console.log('');
    console.log('Ready for NAAAS deployment testing! 🎉');
});

// Graceful shutdown
process.on('SIGTERM', () => {
    console.log('🛑 SIGTERM received, shutting down gracefully...');
    process.exit(0);
});

process.on('SIGINT', () => {
    console.log('🛑 SIGINT received, shutting down gracefully...');
    process.exit(0);
});