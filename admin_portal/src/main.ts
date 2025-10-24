import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/app.config';
import { App } from './app/app';
import init from './assets/wasm_backend/wasm_backend.js';

async function main() {
  await init();
  await bootstrapApplication(App, appConfig);

}

main().catch((err) => console.error(err));