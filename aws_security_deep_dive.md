# Doubling Down on Security: A Holistic AWS Architecture for NAAAS

This document outlines a comprehensive, multi-layered security strategy for the NAAAS project, leveraging the full capabilities of the AWS ecosystem. The goal is to build a defense-in-depth architecture that provides enterprise-grade security for all tenants.

## 1. The Identity & Access Layer

This layer focuses on securing access to both the application (for end-users) and the infrastructure (for administrators and services).

*   **Offloaded User Authentication (Amazon Cognito):** As discussed, this is a primary feature. By delegating user authentication for tenant applications (like Ghost comments) to Cognito, NAAAS protects the upstream application from all credential-based attacks (e.g., password spraying, credential stuffing). It also enables modern security features like MFA, social sign-on, and federated identity.

*   **Secrets Management (AWS Secrets Manager):** Unikernel credentials (e.g., API keys, database passwords for upstream apps) are never stored in configuration files. They are stored and encrypted in AWS Secrets Manager. Each unikernel is granted a tightly-scoped IAM role that allows it to fetch *only its own secret* at runtime. This practice dramatically reduces the risk of credential leakage.

*   **Principle of Least Privilege (IAM Roles):** Every component of the system operates under a specific IAM Role with the absolute minimum required permissions.
    *   **Unikernel Role:** Can only write to its own CloudWatch Log stream and fetch its specific secret from Secrets Manager. It has no access to other tenants' resources.
    *   **Control Plane Role (`naaas-server`):** Has permissions to manage EC2/Firecracker instances and their associated IAM roles, but has no access to the secrets themselves.

## 2. The Edge & Network Layer

This layer focuses on stopping threats before they reach the application.

*   **DDoS Protection (AWS Shield):** AWS Shield Standard is enabled by default on the Application Load Balancer (ALB), providing automatic protection against common, network-level DDoS attacks.

*   **Web Application Firewall (AWS WAF):** The ALB is integrated with AWS WAF. This allows for:
    *   **Managed Rulesets:** Instant protection against the OWASP Top 10, SQL injection, XSS, and other common exploits by enabling AWS-managed rules.
    *   **Custom Rules:** The ability to define custom rules, such as blocking traffic from specific geographic locations or IP address ranges.

*   **Comprehensive Encryption:** All data is encrypted in transit through every step of its journey.
    1.  **Client to ALB:** Standard TLS encryption, managed by AWS Certificate Manager (ACM).
    2.  **ALB to EC2 Host:** The ALB is configured to re-encrypt traffic before sending it to the backend, ensuring it's not in the clear within the VPC.
    3.  **Shim to Upstream Application:** The NAAAS shim should be configured to connect to the monolith via HTTPS, ensuring end-to-end encryption.

## 3. The Application & Runtime Layer

This layer focuses on security intelligence built into the NAAAS shim itself.

*   **Intelligent Rate Limiting:** The shim enforces sophisticated rate limiting beyond simple request counts. It can analyze patterns to distinguish between normal user behavior and malicious scanning or brute-force attempts.
*   **Application-Aware Filtering:** The shim can act as a highly specialized WAF for the application it's wrapping. For a WordPress shim, it could instantly block all requests to `/xmlrpc.php` or known-vulnerable plugin paths, offloading that security logic from the application itself.
*   **Immutable Audit Trails:** The structured JSON logs sent to CloudWatch Logs serve as a detailed, per-tenant audit trail of all activity. These logs can be configured for immutability to ensure a reliable forensic record.

## 4. The Proactive & Automated Defense Layer

This is the most advanced layer, creating a self-healing system that actively responds to threats.

*   **Threat Detection (Amazon GuardDuty):** GuardDuty is enabled across the AWS account to continuously monitor for anomalous behavior. It can detect if an instance is being probed for open ports, communicating with known malicious IPs, or exhibiting other signs of compromise.

*   **Automated Response (AWS Lambda & EventBridge):**
    1.   GuardDuty detects a threat (e.g., a malicious IP scanning Tenant C) and publishes a finding to Amazon EventBridge.
    2.   An EventBridge rule, filtered for specific finding types, triggers an AWS Lambda function.
    3.   The Lambda function executes a pre-programmed response:
        *   **Block:** It can instantly add the malicious IP to a blocklist in AWS WAF, protecting all tenants from that attacker.
        *   **Isolate:** It can call the `naaas-server`'s API to immediately "quarantine" or shut down the affected tenant's unikernel to prevent lateral movement or further damage.
        *   **Alert:** It can send a high-priority notification to a security operations channel.

This automated feedback loop transforms the security posture from reactive to proactive, neutralizing threats in near real-time without human intervention.
