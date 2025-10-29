import { ChangeDetectionStrategy, Component, inject, ViewChildren, QueryList, type OnInit, AfterViewInit } from '@angular/core';
import initWasm from '../../assets/wasm_backend/wasm_backend.js';
import { KeyProviderConfigComponent } from '../key-provider-config/key-provider-config.js';
import { UserTableComponent } from '../user-table/user-table.js';
import { LogsTableComponent } from '../logs-table/logs-table.js';
import { MatDialog } from '@angular/material/dialog';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatButtonModule } from '@angular/material/button';
import { MatDivider } from '@angular/material/divider';
import { MatListModule } from '@angular/material/list';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatExpansionModule, MatExpansionPanel } from '@angular/material/expansion';

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
    UserTableComponent,
    LogsTableComponent],
  templateUrl: './portal.component.html',
  styleUrl: './portal.component.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})

export class PortalComponent implements OnInit, AfterViewInit {
  ngOnInit() {
    if (typeof window !== 'undefined') {
      // only initialize wasm in browser, not during SSR
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
}
