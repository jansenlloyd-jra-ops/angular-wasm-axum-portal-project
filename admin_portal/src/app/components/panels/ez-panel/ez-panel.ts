import { CommonModule } from '@angular/common';
import { Component, inject, ViewChild } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatDialog } from '@angular/material/dialog';
import { MatPaginator } from '@angular/material/paginator';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { fetch_all_ez } from '../../../../assets/wasm_backend/wasm_backend';
import { CreateEzDialog } from '../../dialog/create-ez-dialog/create-ez-dialog';

export interface EncryptionZoneTableStruct {
  zone_name: string,
  ez_id: string,
  kpc_id: string,
}

@Component({
  selector: 'app-ez-panel',
  imports: [MatTableModule, MatPaginator, CommonModule, MatButtonModule],
  templateUrl: './ez-panel.html',
  styleUrl: './ez-panel.scss'
})
export class EzPanelComponent {
  displayedColumns: string[] = ['zone_name', 'ez_id', 'kpc_id'];
  dataSource_EZ = new MatTableDataSource<EncryptionZoneTableStruct>([{zone_name:"test", ez_id: "test", kpc_id:"test"}]);
  loading = false;

  @ViewChild(MatPaginator) paginator!: MatPaginator;

  readonly dialog = inject(MatDialog);
  openAddUserDialog(): void {
    this.dialog.open(CreateEzDialog, {
      panelClass: 'no-default-dialog',
      autoFocus: false,
    });
  }

  async ngOnInit(): Promise<void> {
    await this.loadEZ();
  }

  ngAfterViewInit() {
    this.dataSource_EZ.paginator = this.paginator;
  }

  async loadEZ() {
    this.loading = true;
    try {
      const providers: EncryptionZoneTableStruct[] = JSON.parse(await fetch_all_ez());
      this.dataSource_EZ = new MatTableDataSource<EncryptionZoneTableStruct>(providers);
      this.dataSource_EZ.paginator = this.paginator;
    } catch (error) {
      console.error('Failed to load provider data:', error);
    } finally {
      this.loading = false;
    }
  }
}
