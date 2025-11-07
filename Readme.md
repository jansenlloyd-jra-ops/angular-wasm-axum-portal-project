### Build WASM file to admin portal
1. cd wasm_backend
2. wasm-pack build --target web --out-dir ../admin_portal/src/assets/wasm_backend
3. cd ..
4. cd admin_portal
5. ng serve
### Test Locally
*Run with admin portal*
1. cd wasm_server
2. cargo run

