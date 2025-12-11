const express = require('express');
const os = require('os');

const app = express();
const port = process.env.PORT || 3000;
const hostname = os.hostname();
const startTime = Date.now();

// Tenant-specific configuration for integration testing
const tenantName = process.env.NAAAS_TENANT || 'default';
const backgroundColor = process.env.NAAAS_BACKGROUND_COLOR || '#f5f5f5';
const primaryColor = process.env.NAAAS_PRIMARY_COLOR || '#2c5282';

// Middleware to log requests
app.use((req, res, next) => {
    console.log(`${new Date().toISOString()} - ${req.method} ${req.path} from ${req.ip}`);
    next();
});

// Middleware to parse JSON bodies
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

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
                background: ${backgroundColor};
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
                color: ${primaryColor}; 
                border-bottom: 3px solid ${primaryColor};
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
            <h1>🎉 Welcome to ${tenantName}!</h1>
            <div class="status">✅ NAAAS tenant instance running successfully</div>
            
            <div class="info-section">
                <h3>System Information</h3>
                <div class="info-grid">
                    <span class="label">Tenant:</span>
                    <span><strong>${tenantName}</strong></span>
                    
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
                    NAAAS_TENANT: tenantName,
                    NAAAS_BACKGROUND_COLOR: backgroundColor,
                    NAAAS_PRIMARY_COLOR: primaryColor,
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
                <h3>Integration Testing</h3>
                <p><strong>This is a tenant-specific Express.js application running behind the NAAAS shim proxy.</strong></p>
                <p>Each tenant gets their own dedicated instance with custom configuration.</p>
                <p>If you can see this page with the correct tenant name and colors, your NAAAS integration is working correctly!</p>
                
                <h4>Test Endpoints (for shim integration testing):</h4>
                <ul>
                    <li><a href="/health" target="_blank">/health</a> - Health check with tenant info</li>
                    <li><a href="/api/info" target="_blank">/api/info</a> - Detailed system information</li>
                    <li><a href="/styles.css" target="_blank">/styles.css</a> - Tenant-specific CSS</li>
                    <li><a href="/search?q=test&limit=10" target="_blank">/search?q=test&limit=10</a> - Query parameter test</li>
                    <li><a href="/api/data" target="_blank">/api/data</a> - JSON API response</li>
                    <li><a href="/api/status" target="_blank">/api/status</a> - Plain text response</li>
                    <li><a href="/error" target="_blank">/error</a> - Simulated server error</li>
                    <li><strong>POST /api/test</strong> - Test request body forwarding</li>
                    <li><strong>PUT /api/test</strong> - Test PUT method forwarding</li>
                    <li><strong>POST /api/upload</strong> - File upload simulation</li>
                </ul>
                
                <p><strong>Shim Config Test:</strong> Visit <code>/config</code> to see NAAAS shim configuration</p>
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
        tenant: tenantName,
        hostname: hostname,
        uptime: Math.floor((Date.now() - startTime) / 1000),
        timestamp: new Date().toISOString(),
        service: 'naaas-welcome-app',
        theme: {
            background_color: backgroundColor,
            primary_color: primaryColor
        }
    });
});

// API info endpoint
app.get('/api/info', (req, res) => {
    res.json({
        tenant: tenantName,
        hostname: hostname,
        port: port,
        uptime: Math.floor((Date.now() - startTime) / 1000),
        memory: process.memoryUsage(),
        node_version: process.version,
        platform: os.platform(),
        architecture: os.arch(),
        theme: {
            background_color: backgroundColor,
            primary_color: primaryColor
        },
        environment: {
            NODE_ENV: process.env.NODE_ENV,
            NAAAS_TENANT: tenantName,
            NAAAS_BACKGROUND_COLOR: backgroundColor,
            NAAAS_PRIMARY_COLOR: primaryColor,
            NAAAS_REGION: process.env.NAAAS_REGION
        },
        headers: req.headers
    });
});

// POST/PUT endpoint to test request body forwarding through shim
app.post('/api/test', (req, res) => {
    res.json({
        message: 'POST request received successfully',
        tenant: tenantName,
        received_body: req.body,
        content_type: req.headers['content-type'],
        method: req.method,
        headers: req.headers,
        timestamp: new Date().toISOString()
    });
});

app.put('/api/test', (req, res) => {
    res.json({
        message: 'PUT request received successfully', 
        tenant: tenantName,
        received_body: req.body,
        content_type: req.headers['content-type'],
        method: req.method,
        headers: req.headers,
        timestamp: new Date().toISOString()
    });
});

// CSS endpoint to test static resource forwarding through shim
app.get('/styles.css', (req, res) => {
    res.setHeader('content-type', 'text/css');
    res.send(`
/* Tenant-specific CSS for ${tenantName} */
body { 
    background: ${backgroundColor}; 
    font-family: Arial, sans-serif;
}

h1 { 
    color: ${primaryColor}; 
    border-bottom: 2px solid ${primaryColor};
}

.tenant-badge {
    background: ${primaryColor};
    color: white;
    padding: 5px 10px;
    border-radius: 3px;
    font-size: 12px;
}

.container {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    background: rgba(255,255,255,0.9);
    border-radius: 8px;
}
    `);
});

// Search endpoint to test query parameter forwarding
app.get('/search', (req, res) => {
    res.json({
        message: 'Search endpoint working',
        tenant: tenantName,
        query_params: req.query,
        url: req.url,
        path: req.path,
        query_string: req.url.split('?')[1] || null,
        headers: req.headers,
        search_results: Object.keys(req.query).length > 0 ? 
            `Found ${Object.keys(req.query).length} search parameters for tenant ${tenantName}` :
            'No search parameters provided'
    });
});

// Error endpoint to test shim error handling
app.get('/error', (req, res) => {
    console.log(`Simulating server error for tenant ${tenantName}`);
    res.status(500).json({
        error: "Simulated server error",
        tenant: tenantName,
        message: "This error is intentional for testing shim error handling",
        timestamp: new Date().toISOString()
    });
});

// JSON API endpoint to test different content types
app.get('/api/data', (req, res) => {
    res.json({
        tenant: tenantName,
        data: {
            users: [
                {id: 1, name: 'Alice', tenant: tenantName},
                {id: 2, name: 'Bob', tenant: tenantName}
            ],
            config: {
                theme: {
                    background: backgroundColor,
                    primary: primaryColor
                },
                features: ['proxy_test', 'tenant_isolation', 'json_api']
            }
        },
        meta: {
            timestamp: new Date().toISOString(),
            hostname: os.hostname(),
            version: '1.0.0'
        }
    });
});

// Text endpoint to test plain text responses
app.get('/api/status', (req, res) => {
    res.setHeader('content-type', 'text/plain');
    res.send(`NAAAS Tenant Status
Tenant: ${tenantName}
Hostname: ${os.hostname()}
Uptime: ${Math.floor((Date.now() - startTime) / 1000)} seconds
Status: Active
Theme: ${backgroundColor} / ${primaryColor}
Time: ${new Date().toISOString()}
`);
});

// File upload simulation endpoint
app.post('/api/upload', (req, res) => {
    res.json({
        message: 'File upload endpoint (simulation)',
        tenant: tenantName,
        received_content_type: req.headers['content-type'],
        content_length: req.headers['content-length'] || '0',
        body_size: JSON.stringify(req.body).length,
        note: 'This would handle file uploads in a real application'
    });
});

// WebSocket simulation endpoint (HTTP upgrade test)
app.get('/api/websocket', (req, res) => {
    res.json({
        message: 'WebSocket simulation endpoint',
        tenant: tenantName,
        upgrade_header: req.headers['upgrade'] || null,
        connection_header: req.headers['connection'] || null,
        note: 'This would handle WebSocket upgrades in a real application'
    });
});

// Start the server
app.listen(port, () => {
    console.log(`🚀 NAAAS Welcome App started successfully!`);
    console.log(`👤 Tenant: ${tenantName}`);
    console.log(`📍 Hostname: ${hostname}`);
    console.log(`🌐 Port: ${port}`);
    console.log(`🎨 Theme: ${backgroundColor} / ${primaryColor}`);
    console.log(`📊 Memory: ${Math.round(process.memoryUsage().rss / 1024 / 1024)} MB`);
    console.log(`⏰ Started at: ${new Date().toISOString()}`);
    console.log(`🔗 Local URL: http://localhost:${port}`);
    console.log('');
    console.log('Ready for NAAAS shim integration testing! 🎉');
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