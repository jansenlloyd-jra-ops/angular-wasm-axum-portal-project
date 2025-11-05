import { Component, signal, NgZone } from '@angular/core';
import { FormsModule } from '@angular/forms';
import initWASM, { is_authenticated, provision_config_request } from '../../../assets/wasm_backend/wasm_backend.js';
import { MatButtonModule } from '@angular/material/button';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { Router } from '@angular/router';

@Component({
  selector: 'app-sign-in',
  imports: [MatInputModule, MatFormFieldModule, MatButtonModule, FormsModule],
  templateUrl: './sigin-in.component.html',
  styleUrl: './sigin-in.component.scss'
})
export class SiginInComponent {
  ngOnInit() {
    if (typeof window !== 'undefined') {
      // ✅ only initialize wasm in browser, not during SSR
      initWASM();
    }
  }
  constructor(private router: Router, private zone: NgZone) { }
  requesting = signal<boolean>(false);
  signin = signal<boolean>(true);
  configResponse: any = { response: '' };
  provisionConfig = JSON.stringify(
    {
      provider: "google",
      scope: "openid email profile",
      type: "oidc",
      client_id: "181370640671-rb2l88739bspe0ifbnsq7inoniqu4mgu.apps.googleusercontent.com",
      client_secret: "",
      discovery_url: "https://accounts.google.com/.well-known/openid-configuration",
      redirect_uri: "http://localhost:8080/v1/portal/callback",
      issuer: "https://accounts.google.com"
    },
    null,
    2
  );

  async provisionRequest() {
    this.requesting.set(true);
    let res = await provision_config_request(this.provisionConfig);
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
    // console.log(signURL);
    window.open(signURL, "_blank");
    const interval = setInterval(async () => {
      const result = JSON.parse(await is_authenticated());
      if (result.authorized) {
        clearInterval(interval);
        this.zone.run(() => {
          window.close();
          this.router.navigate(['/dashboard']);
        });
      } else {
        this.signin.set(false);
      }
    }, 1000);


    // this.router.navigate(['/dashboard']);
    // setTimeout(() => {
    //   this.signin.set(false);
    // }, 20);
  }
}
