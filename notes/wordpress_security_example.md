# WordPress Security: Perfect Example of Proxy Layer Intelligence

## The Problem: WordPress Under Attack

WordPress powers 40% of the web and is a constant target for automated attacks. Traditional security approaches either:
1. **Application-level plugins**: Add overhead and can be bypassed
2. **WAF solutions**: Expensive and often over-engineered
3. **Server hardening**: Reactive rather than proactive

**The proxy layer solution**: Detect and block attacks before they ever reach WordPress.

---

## Attack Patterns Visible at Proxy Layer

### 1. **Brute Force Attacks**

**Login Brute Force (`/wp-login.php`):**
```
POST /wp-login.php - Failed (user: admin, pass: password)
POST /wp-login.php - Failed (user: admin, pass: 123456)
POST /wp-login.php - Failed (user: admin, pass: admin)
POST /wp-login.php - Failed (user: admin, pass: wordpress)
```

**XML-RPC Brute Force (`/xmlrpc.php`):**
```
POST /xmlrpc.php - Multiple authentication attempts in single request
POST /xmlrpc.php - Dictionary attack via system.multicall
```

**User Enumeration:**
```
GET /?author=1 - Returns username in redirect
GET /?author=2 - Mapping all user accounts
GET /wp-json/wp/v2/users - REST API user discovery
```

### 2. **Vulnerability Scanning**

**Configuration File Probes:**
```
GET /wp-config.php - Direct config access attempt
GET /wp-config.php.backup - Backup file probe
GET /wp-config.txt - Alternative config names
GET /.wp-config.php.swp - Editor backup files
```

**Plugin Vulnerability Scans:**
```
GET /wp-content/plugins/vulnerable-plugin/readme.txt - Version detection
GET /wp-content/plugins/revslider/temp/update_extract/ - Known exploit paths
GET /wp-content/plugins/*/admin.php - Admin file discovery
```

**Theme Exploitation:**
```
GET /wp-content/themes/twentytwenty/404.php - Direct file access
GET /wp-content/themes/*/functions.php - Theme function exposure
```

### 3. **File Inclusion & Path Traversal**

**Local File Inclusion (LFI):**
```
GET /wp-content/themes/theme/index.php?file=../../../etc/passwd
GET /wp-admin/admin-ajax.php?action=../../../wp-config.php
```

**Directory Traversal:**
```
GET /wp-content/uploads/../../../wp-config.php
GET /wp-includes/../wp-config.php
```

### 4. **Automated Bot Behavior**

**Predictable Scanning Patterns:**
```
GET /wp-admin/ (no session, no referrer)
GET /wp-login.php (immediate, no prior page visits)
GET /xmlrpc.php (directly, no site interaction)
GET /readme.html (version fingerprinting)
```

**Missing Browser Signatures:**
- No Accept-Language headers
- Missing Accept-Encoding
- Suspicious User-Agent strings
- No JavaScript/CSS requests (bots don't render)

---

## Proxy Layer Security Rules

### Immediate Block Rules (Zero Tolerance)

```yaml
block_rules:
  - path: "/wp-config.php*"
    action: permanent_ban
    reason: "Configuration file access attempt"
  
  - path: "*/wp-config.php*"
    action: permanent_ban
    reason: "Backup configuration probe"
  
  - pattern: "\\.\\./"
    action: immediate_block
    reason: "Directory traversal attempt"
  
  - user_agent: "*bot*" 
    exclude: ["googlebot", "bingbot"]
    action: challenge
    reason: "Unauthorized bot activity"
```

### Rate Limiting Rules (Graduated Response)

```yaml
rate_limits:
  wp_login_failures:
    path: "/wp-login.php"
    condition: "status >= 400"
    limit: 5
    window: 60s
    action: block_1hour
  
  xmlrpc_requests:
    path: "/xmlrpc.php"
    limit: 2
    window: 60s
    action: block_10min
  
  admin_access:
    path: "/wp-admin/*"
    condition: "no_valid_session"
    limit: 10
    window: 300s
    action: challenge
```

### Behavioral Analysis Rules

```yaml
behavior_rules:
  rapid_scanning:
    description: "Multiple admin paths without CSS/JS requests"
    conditions:
      - admin_requests > 5
      - css_requests == 0
      - js_requests == 0
      - timeframe < 30s
    action: tarpit
  
  user_enumeration:
    description: "Sequential author ID probing"
    conditions:
      - path_pattern: "/?author=*"
      - sequential_ids: true
      - requests > 3
    action: block_24hours
  
  plugin_scanning:
    description: "Systematic plugin directory probing"
    conditions:
      - path_pattern: "/wp-content/plugins/*"
      - unique_plugins > 10
      - timeframe < 60s
    action: permanent_ban
```

---

## Per-Tenant Security Profiles

### Enterprise Tenant (High Security)
```yaml
tenant_security:
  profile: enterprise
  rules:
    - block_all_xmlrpc: true
    - admin_ip_whitelist: ["192.168.1.0/24"]
    - require_2fa_headers: true
    - max_login_attempts: 3
    - geographic_restrictions: ["US", "CA"]
```

### Standard Tenant (Balanced Security)
```yaml
tenant_security:
  profile: standard
  rules:
    - xmlrpc_rate_limit: 2_per_minute
    - login_attempt_limit: 5
    - challenge_suspicious_behavior: true
    - block_known_bad_ips: true
```

### Developer Tenant (Relaxed for Testing)
```yaml
tenant_security:
  profile: development
  rules:
    - allow_config_access: from_whitelist_ips
    - disable_bot_blocking: true
    - extended_rate_limits: true
    - verbose_security_headers: true
```

---

## Response Strategies

### 1. **Immediate Blocking**
- Return 403 Forbidden for obvious attacks
- Log attack details for analysis
- Add IP to permanent ban list

### 2. **Tarpitting**
- Slow down responses to waste attacker time
- Return convincing fake responses
- Make attacks uneconomical

### 3. **Challenge Responses**
- JavaScript challenges for suspicious requests
- CAPTCHA for repeated violations
- Temporary blocks with appeal process

### 4. **Honeypots**
- Fake wp-config.php with tracking
- Fake admin pages that identify attackers
- Tempting but non-existent plugin directories

---

## Integration with WordPress Provider

A WordPress hosting provider could extend the base NAAAS unikernel:

### Extended Threat Intelligence
```rust
// Custom WordPress security rules
impl WordPressSecurityExtension {
    fn check_plugin_vulnerability(&self, path: &str) -> SecurityAction {
        if self.vulnerability_db.contains_exploit(path) {
            SecurityAction::Block("Known vulnerable plugin")
        } else {
            SecurityAction::Allow
        }
    }
    
    fn detect_theme_exploitation(&self, request: &Request) -> SecurityAction {
        if request.path.contains("/wp-content/themes/") 
           && request.path.contains(".php") 
           && !self.allowed_theme_files.contains(&request.path) {
            SecurityAction::Block("Theme file direct access")
        } else {
            SecurityAction::Allow
        }
    }
}
```

### Custom Response Pages
- Fake WordPress error pages that look legitimate
- Honeypot login forms that capture credentials
- Realistic 404 pages for blocked requests

### Advanced Analytics
- Attack pattern reporting for hosting provider
- Tenant security health scores
- Automated security recommendations

---

## Why This Works at the Proxy Layer

### **Visibility**: Sees all requests, can detect patterns
### **Speed**: Blocks attacks before they reach WordPress
### **Isolation**: Each tenant gets independent security state
### **Efficiency**: No WordPress plugin overhead
### **Flexibility**: Rules can be updated without touching WordPress

This is exactly the kind of "obvious responsibility" that naturally belongs at the NAAAS layer - protecting applications from threats they can't see coming.

---

## Implementation Priority

### Phase 1: Basic Protection
- Block config file access
- Simple rate limiting on login endpoints
- Basic bot detection

### Phase 2: Pattern Recognition  
- Brute force detection across multiple endpoints
- User enumeration protection
- Plugin scanning detection

### Phase 3: Advanced Intelligence
- Behavioral analysis
- Machine learning threat detection
- Coordinated attack response

This demonstrates how the unikernel layer can provide immediate, high-value security that would be extremely difficult to implement consistently across individual WordPress installations.