import { ChangeDetectionStrategy, Component, inject, type OnInit, signal } from '@angular/core';
import initWasm from '../../assets/wasm_backend/wasm_backend.js';
import { KeyProviderConfigComponent } from '../key-provider-config/key-provider-config.js';
import { MatDialog } from '@angular/material/dialog';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatButtonModule } from '@angular/material/button';
import { MatDivider } from '@angular/material/divider';
import { MatListModule } from '@angular/material/list';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatExpansionModule } from '@angular/material/expansion';

@Component({
  selector: 'portal-demo',
  standalone: true,
  imports: [MatSidenavModule, MatButtonModule, MatDivider, MatListModule, MatToolbarModule, MatExpansionModule],
  templateUrl: './portal.component.html',
  styleUrl: './portal.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})

export class PortalComponent implements OnInit {
  ngOnInit() {
    if (typeof window !== 'undefined') {
      // ✅ only initialize wasm in browser, not during SSR
      initWasm();
    }
  }
  readonly dialog = inject(MatDialog);

  configResponse: any = { google: {}, aws: {}, azure: {} };
  openKPC(): void {
    this.dialog.open(KeyProviderConfigComponent, {
      panelClass: 'no-default-dialog',
      autoFocus: false,
    });
  }
}
