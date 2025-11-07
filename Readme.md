### Build WASM file to admin portal
1. cd wasm_backend ; wasm-pack build --target web --out-dir ../admin_portal/src/assets/wasm_backend
2. cd ../admin_portal; npm install; ng serve
### Test Locally
*Run with admin portal*
1. cd ../wasm_server; cargo run

### Sample Config for testing
```
{
  "provider": "google",
  "scope": "openid email profile",
  "provision_type": "oidc",
  "client_id": "181370640671-rb2l88739bspe0ifbnsq7inoniqu4mgu.apps.googleusercontent.com",
  "client_secret": "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",
  "discovery_url": "https://accounts.google.com/.well-known/openid-configuration",
  "redirect_uri": "http://localhost:8080/v1/portal/callback",
  "issuer": "https://accounts.google.com"
}
```