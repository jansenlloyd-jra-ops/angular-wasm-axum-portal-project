import { ChangeDetectionStrategy, Component, inject, ViewChildren, QueryList, type OnInit, AfterViewInit } from '@angular/core';
import initWasm from '../../../assets/wasm_backend/wasm_backend.js';
import { KpcPanelComponent } from '../panels/kpc-panel/kpc-panel.js';
import { UserPanelComponent } from '../panels/user-panel/user-panel.js';
import { LogsPanelComponent } from '../panels/logs-panel/logs-panel.js';
import { Router } from "@angular/router";
import { MatDialog } from '@angular/material/dialog';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatButtonModule } from '@angular/material/button';
import { MatDivider } from '@angular/material/divider';
import { MatListModule } from '@angular/material/list';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatExpansionModule, MatExpansionPanel } from '@angular/material/expansion';
import { ProvidersPanelComponent } from '../panels/providers-panel/providers-panel.js';

@Component({
  selector: 'portal-demo',
  standalone: true,
  imports: [
    MatSidenavModule,
    MatButtonModule,
    MatDivider,
    MatListModule,
    MatToolbarModule,
    MatExpansionModule,
    UserPanelComponent,
    ProvidersPanelComponent
    // LogsPanelComponent
  ],
  templateUrl: './portal.component.html',
  styleUrl: './portal.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})

export class PortalComponent implements OnInit, AfterViewInit {
  [x: string]: any;
  ngOnInit() {
    if (typeof window !== 'undefined') {
      // only initialize wasm in browser, not during SSR
      initWasm();
    }
  }
  constructor(private router: Router) {}
  configResponse: any = { google: {}, aws: {}, azure: {} };

  readonly dialog = inject(MatDialog);
  openKPC(): void {
    this.dialog.open(KpcPanelComponent, {
      panelClass: 'no-default-dialog',
      autoFocus: false,
    });
  }

  @ViewChildren(MatExpansionPanel) panels!: QueryList<MatExpansionPanel>;
  allExpanded = true; // ✅ start with true since panels are expanded

  ngAfterViewInit() {
    this.panels.forEach(panel => {
      panel.opened.subscribe(() => this.updateExpandState());
      panel.closed.subscribe(() => this.updateExpandState());
    });
  }

  private updateExpandState() {
    this.allExpanded = this.panels.toArray().every(panel => panel.expanded);
  }

  toggleAllPanels() {
    const shouldExpand = !this.allExpanded;
    this.panels.forEach(panel => {
      shouldExpand ? panel.open() : panel.close();
    });
    this.allExpanded = shouldExpand;
  }

  logOut(){
    this.router.navigate(['/']);
  }
  // async fetchAllKPC() {
  //   let configs = await kpc_fetch_all();
  //   console.log(configs);
  // }
}
