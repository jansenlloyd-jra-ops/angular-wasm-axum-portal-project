import { ChangeDetectionStrategy, Component, type OnInit, signal } from '@angular/core';
import { send_config, kms_config_request } from '../../assets/wasm_backend/wasm_backend.js';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';


@Component({
  selector: 'kpc',
  standalone: true,
  imports: [FormsModule, MatButtonModule],
  templateUrl: './key-provider-config.html',
  styleUrl: './key-provider-config.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})

export class KeyProviderConfigComponent {
  serveTime = signal<string>('');
  serverResponse: any = {};
  serveTimeResp = signal<string>('');
  signin = signal<boolean>(true);

  // ====================================================================================
  configResponse: any = { google: {}, aws: {}, azure: {} };
  requesting = signal<boolean>(false);
  copiedPrompt = signal<string>('');
  defaultAWS_kpc = JSON.stringify({
    provider_name: "provider_name",
    region: "region",
    access_key_id: "access_key_id_default",
    secret_access_key: "secret_access_key_default"
  },
    null,
    2
  );

  defaultAzure_kpc = JSON.stringify({
    vault_url: "vault_default_url",
    tenant_id: "tenant1_id",
    client_id: "client_id_default",
    client_secret: "client_secret_default"
  },
    null,
    2
  );

  defaultGoogle_kpc = JSON.stringify({
    type: "service_account",
    project_id: "project_id",
    private_key_id: "private_key_id",
    private_key: "private_key",
    client_email: "client@yourdomain.iam.gserviceaccount.com",
    client_id: "client_id",
    auth_uri: "https://accounts.google.com/o/oauth2/auth",
    token_uri: "https://oauth2.googleapis.com/token",
    auth_provider_x509_cert_url: "https://www.googleapis.com/oauth2/v1/certs",
    client_x509_cert_url: "https://www.googleapis.com/robot/v1/metadata/x509/client%40yourdomain.iam.gserviceaccount.com",
    universe_domain: "googleapis.com"
  },
    null,
    2
  );

  selectedProvider: any = null;
  selectedProviderConfig = JSON.stringify({ select_provider: "key_provider..." }, null, 2);

  toggleProvider(provider: string) {
    switch (provider) {
      case 'Google':
        this.selectedProviderConfig = this.defaultGoogle_kpc;
        this.selectedProvider = provider;
        break;
      case 'AWS':
        this.selectedProviderConfig = this.defaultAWS_kpc;
        this.selectedProvider = provider;
        break;
      case 'Azure':
        this.selectedProviderConfig = this.defaultAzure_kpc;
        this.selectedProvider = provider;
        break;
      case null:
        this.selectedProviderConfig = JSON.stringify({ select_provider: "key_provider..." }, null, 2);
        this.selectedProvider = "";
        break;
    }
  }
  async kpc_request() {
    this.requesting.set(true);
    let res = await kms_config_request(String(this.selectedProvider).toLowerCase(), this.selectedProviderConfig);
    this.configResponse[this.selectedProvider] = JSON.parse(res);
    setTimeout(() => {
      this.requesting.set(false);
    }, 300);
  };

  copyToClipboard(text: string) {
    navigator.clipboard.writeText(text).then(() => {
      console.log('Copied to clipboard!');
      this.copiedPrompt.set("Key Provider ID Copied!");
      setTimeout(() => {
        this.copiedPrompt.set('');
      }, 5000);
    }).catch(err => {
      console.error('Failed to copy: ', err);
    });
  }
  // ====================================================================================

  provisionConfig = JSON.stringify(
    {
      provider: "google",
      scope: "openid email profile",
      type: "oidc",
      client_id: "181370640671-rb2l88739bspe0ifbnsq7inoniqu4mgu.apps.googleusercontent.com",
      client_secret: "",
      discovery_url: "https://accounts.google.com/.well-known/openid-configuration",
      redirect_uri: "http://localhost:8080/portal/callback",
      issuer: "https://accounts.google.com"
    },
    null,
    2
  );


  async provisionRequest() {
    this.requesting.set(true);
    let res = await send_config(this.provisionConfig);
    this.configResponse = JSON.parse(res);
    setTimeout(() => {
      this.requesting.set(false);
    }, 300);
    if (this.configResponse) {
      this.signin.set(false);
    }
  }

  signinRequest() {
    this.signin.set(true);
    let jsonRequest = JSON.parse(this.provisionConfig);
    let signURL = `https://accounts.google.com/o/oauth2/v2/auth/oauthchooseaccount?client_id=${jsonRequest.client_id}&redirect_uri=${jsonRequest.redirect_uri}&response_type=code&scope=${jsonRequest.scope}`
    console.log(signURL);
    window.open(signURL, "_blank");
    setTimeout(() => {
      this.signin.set(false);
    }, 20);
  }

}
