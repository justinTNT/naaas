# Cognito Authentication Workflow for Ghost Visitor Comments

This document describes the step-by-step workflow for using Amazon Cognito, orchestrated by the NAAAS shim, to provide a secure and user-friendly authentication experience for visitors wishing to comment on a Ghost blog.

This flow replaces the default Ghost authentication mechanism with a modern SSO experience, without modifying the Ghost application itself.

### System Components

*   **User's Browser:** The end-user's web browser.
*   **NAAAS Shim:** The "intelligent wrapper" unikernel proxying the Ghost blog.
*   **Amazon Cognito:** The managed AWS identity service.
*   **Ghost Application:** The upstream, unmodified Ghost blog instance.

### The "Happy Path" Workflow

1.  **User Initiates Login:**
    *   A visitor is reading a post on the Ghost blog and wants to leave a comment.
    *   Instead of a standard login form, they click a custom link styled as "Login to Comment" or "Login with Google".
    *   This link points to a special, non-proxied endpoint on the NAAAS shim, for example: `https://your-tenant-blog.com/__/auth/login`.

2.  **Redirect to Cognito:**
    *   The NAAAS shim intercepts the request to `/__/auth/login`.
    *   It does not forward this to Ghost. Instead, it generates a state token to prevent CSRF attacks and redirects the user's browser to the Amazon Cognito hosted UI.
    *   The redirect URL includes parameters like the `client_id` for this application and the `redirect_uri` which points back to the shim.

3.  **User Authenticates with Cognito:**
    *   The user is now on a secure login page hosted by AWS. This page can be configured to offer various identity providers (e.g., Google, Facebook, SAML, or a simple email/password managed by Cognito).
    *   The user authenticates using their chosen method (e.g., by logging into their Google account).
    *   At no point are the user's actual credentials (like their Google password) ever exposed to the NAAAS shim or the Ghost application.

4.  **Callback to NAAAS Shim:**
    *   Upon successful authentication, Cognito redirects the user's browser back to the predefined callback URL on the shim, e.g., `https://your-tenant-blog.com/__/auth/callback`.
    *   This callback includes a temporary, single-use `authorization_code`.

5.  **The "Auth Adaptation" Logic (Token Exchange):**
    *   The NAAAS shim's backend code receives the request with the `authorization_code`.
    *   The shim makes a secure, direct, back-channel API call to Cognito, exchanging the `authorization_code` for a set of JWTs (JSON Web Tokens): an `id_token`, `access_token`, and `refresh_token`.
    *   The `id_token` contains the user's verified identity information, such as their email address and name. The shim cryptographically verifies the signature of this token to ensure it's authentic.

6.  **Upstream Application Login:**
    *   The shim now has definitive proof of the user's identity. Its next job is to log this user into the upstream Ghost application.
    *   It uses its "inside knowledge" to make an internal API call to the Ghost admin API.
    *   **Logic:**
        *   Does a user with the email `user@example.com` already exist in Ghost?
        *   If **YES**, proceed to the next step.
        *   If **NO**, create a new Ghost user with that email address. (The password can be set to a long, random string that is immediately discarded, as it will never be used for password-based login).
    *   The shim then authenticates to Ghost as this user (e.g., via an API call) to obtain a valid Ghost session cookie.

7.  **Session Establishment:**
    *   The NAAAS shim receives the session cookie from Ghost.
    *   It then issues a redirect response to the user's browser, sending them back to the original blog post they were on.
    *   Crucially, this response includes the `Set-Cookie` header with the Ghost session cookie.

### Final Result

The user's browser is redirected back to the blog post, now with a valid session cookie for the Ghost application. The user appears as fully logged in to Ghost and can now post comments.

**Key Benefits Achieved:**
*   **Enhanced Security:** The underlying application's authentication system is no longer the primary security boundary.
*   **Improved User Experience:** Users can log in with accounts they already have, removing the friction of creating a new password.
*   **Zero Application Modification:** This entire, modern authentication flow was added without changing a single line of the Ghost application's code.
