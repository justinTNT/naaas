#!/bin/bash

# Sprint 2 Test Script: Transparent Proxy

echo "🧪 NAAAS Sprint 2 Testing"
echo "========================="
echo

# Start Ghost CMS in Docker
echo "1. Starting Ghost CMS on localhost:2368..."
docker run -d --name ghost-test -p 2368:2368 \
  -e NODE_ENV=development \
  ghost:5-alpine || echo "Ghost container already running"

# Wait for Ghost to start
echo "   Waiting for Ghost to be ready..."
sleep 10

# Test Ghost is accessible
echo "2. Testing Ghost accessibility..."
curl -s -o /dev/null -w "Ghost HTTP Status: %{http_code}\n" http://localhost:2368

echo

# Instructions for manual testing
echo "🎯 Sprint 2 Goals:"
echo "=================+"
echo "1. Deploy a naaas-shim that transparently proxies to Ghost"
echo "2. Verify the proxy returns Ghost's homepage"
echo "3. Verify the /config endpoint works"
echo

echo "📋 Manual Test Steps:"
echo "===================="
echo "1. Build the components:"
echo "   cd src/naaas-shim && cargo build --release"
echo "   cd ../naaas-server && cargo build --release" 
echo "   cd ../naaas-ctl && cargo build --release"
echo
echo "2. Start naaas-server:"
echo "   ./src/naaas-server/target/release/naaas-server"
echo
echo "3. In another terminal, deploy the shim:"
echo "   ./src/naaas-ctl/target/release/naaas-ctl deploy \\"
echo "     --name ghost-proxy \\"
echo "     --unikernel ./src/naaas-shim/target/release/naaas-shim \\"
echo "     --port 3001 \\"
echo "     --upstream http://localhost:2368 \\"
echo "     --config '{\"name\":\"Ghost CMS\",\"primary_color\":\"#15171a\"}'"
echo
echo "4. Test the proxy:"
echo "   curl http://localhost:3001        # Should return Ghost homepage"
echo "   curl http://localhost:3001/config # Should return app config"
echo
echo "5. Cleanup:"
echo "   docker stop ghost-test && docker rm ghost-test"
echo

echo "✅ Ghost CMS is ready at http://localhost:2368"
echo "   You can now proceed with the manual tests above."