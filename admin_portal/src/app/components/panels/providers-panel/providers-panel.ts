import { Component, OnInit, AfterViewInit, ViewChild } from '@angular/core';
import { fetch_all_kpc } from '../../../../assets/wasm_backend/wasm_backend';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { MatPaginator } from '@angular/material/paginator';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';

export interface ProvidersTableStruct {
  name: string;
  provider: string;
  kpc_id: string;
  configuration: string;
}

@Component({
  selector: 'app-providers-panel',
  standalone: true,
  imports: [MatTableModule, MatPaginator, CommonModule, MatButtonModule],
  templateUrl: './providers-panel.html',
  styleUrl: './providers-panel.scss'
})
export class ProvidersPanelComponent implements OnInit, AfterViewInit {
  displayedColumns: string[] = ['kpc_id', 'name', 'provider', 'configuration'];
  dataSource_Providers = new MatTableDataSource<ProvidersTableStruct>([]);
  loading = false;

  @ViewChild(MatPaginator) paginator!: MatPaginator;

  async ngOnInit(): Promise<void> {
    await this.loadProviders();
  }

  ngAfterViewInit() {
    this.dataSource_Providers.paginator = this.paginator;
  }

  async loadProviders() {
    this.loading = true;
    try {
      const providers: ProvidersTableStruct[] = JSON.parse(await fetch_all_kpc());
      this.dataSource_Providers = new MatTableDataSource<ProvidersTableStruct>(providers);
      this.dataSource_Providers.paginator = this.paginator;
    } catch (error) {
      console.error('Failed to load provider data:', error);
    } finally {
      this.loading = false;
    }
  }

}
