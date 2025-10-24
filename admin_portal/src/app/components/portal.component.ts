import { ChangeDetectionStrategy, Component, type OnInit, signal } from '@angular/core';
import { send_config } from '../../assets/wasm_backend/wasm_backend.js';
import { FormsModule } from '@angular/forms';
import init from '../../assets/wasm_backend/wasm_backend.js';


@Component({
  selector: 'portal-demo',
  standalone: true,
  imports: [FormsModule],
  templateUrl: './portal.component.html',
  styleUrl: './portal.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})

export class PortalComponent implements OnInit {
  ngOnInit() {
    if (typeof window !== 'undefined') {
      // ✅ only initialize wasm in browser, not during SSR
      init();
    }
  }

  // jsResult = signal<string>('');
  // rsResult = signal<string>('');
  // jsTime = signal<string>('');
  // rsTime = signal<string>('');
  // calculating = signal<boolean>(false);

  // calculate(inp: number | string) {
  //   this.calculating.set(true);

  //   setTimeout(async () => {
  //     const n = typeof inp === 'number' ? inp : parseInt(inp, 10);
  //     const jsTimeStart = performance.now();
  //     let f = 0;
  //     for (let i = 0; i < 10000000; i++) {
  //       f = factorial(n);
  //     }
  //     this.jsResult.set(f.toString());
  //     this.jsTime.set(((performance.now() - jsTimeStart) / 1000).toFixed(4) + 's');

  //     const rsTimeStart = performance.now();
  //     let fa = get_factorial(n);
  //     this.rsResult.set(fa);
  //     this.rsTime.set(((performance.now() - rsTimeStart) / 1000).toFixed(4) + 's');


  //   }, 50);
  // }

  // async request_wrap_key(inp: string) {
  //   this.requesting.set(true);

  //   const serveTimeStart = performance.now()
  //   let res = await wrapped_key_request(inp);
  //   // console.log("response: " + res);

  //   this.serverResponse = JSON.parse(res);
  //   this.serveTimeResp.set(((performance.now() - serveTimeStart) / 1000).toFixed(4) + 's');
  //   this.requesting.set(false);
  // }


  // @ViewChild('fileInput') fileInput!: ElementRef;
  // file: File | null = null;
  // fileContent: string | null = null;

  // private setFile(file: File): void {
  //   this.file = file;
  //   this.FileAsString(file);
  // }

  // private FileAsString(file: File): void {
  //   const reader = new FileReader();
  //   reader.onload = () => {
  //     this.fileContent = reader.result as string;
  //   };
  //   reader.readAsText(file);
  // }

  // onDragOver(event: DragEvent) {
  //   event.preventDefault();
  //   event.stopPropagation();
  // }

  // onDragLeave(event: DragEvent) {
  //   event.preventDefault();
  //   event.stopPropagation();
  // }

  // onDrop(event: DragEvent) {
  //   event.preventDefault();
  //   event.stopPropagation();

  //   const droppedFile = event.dataTransfer?.files?.[0];
  //   if (droppedFile) {
  //     this.setFile(droppedFile);
  //   }
  // }

  // onFileSelected(event: Event): void {
  //   const input = event.target as HTMLInputElement;
  //   const selectedFile = input.files?.[0];
  //   if (selectedFile) {
  //     this.setFile(selectedFile);
  //   }
  // }

  // async uploadFile() {
  //   if (this.file) {
  //     this.requesting.set(true);
  //     console.log('Uploading:', this.file.name);
  //     console.log('Contents:', this.fileContent);
  //     let res = await send_config(this.fileContent != null ? this.fileContent : "{\"error\":\"missing content\"}");
  //     this.configResponse = JSON.parse(res);
  //     this.requesting.set(false);
  //     // send this.fileContent to your Rust WASM endpoint here
  //   }

  // }

  // clearFile() {
  //   this.file = null;
  //   this.fileContent = null;
  //   this.fileInput.nativeElement.value = '';
  // }
  serveTime = signal<string>('');
  serverResponse: any = {};
  configResponse: any = {};
  serveTimeResp = signal<string>('');
  requesting = signal<boolean>(false);
  signin = signal<boolean>(true);

  jsonInput = JSON.stringify(
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
    2 // <- pretty print with 2-space indentation
  );

  async provisionRequest() {
    this.requesting.set(true);
    let res = await send_config(this.jsonInput);
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
    let jsonRequest = JSON.parse(this.jsonInput);
    let signURL = `https://accounts.google.com/o/oauth2/v2/auth/oauthchooseaccount?client_id=${jsonRequest.client_id}&redirect_uri=${jsonRequest.redirect_uri}&response_type=code&scope=${jsonRequest.scope}`
    console.log(signURL);
    window.open(signURL, "_blank");
    setTimeout(() => {
      this.signin.set(false);
    }, 20);
  }

}
