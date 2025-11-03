import { Component, OnInit, AfterViewInit, ViewChild } from '@angular/core';
import { MatTableModule, MatTableDataSource } from '@angular/material/table';
import { MatPaginator } from '@angular/material/paginator';
import { CommonModule } from '@angular/common';
import { users_fetch_all } from '../../../../assets/wasm_backend/wasm_backend';
import { MatButton, MatButtonModule } from '@angular/material/button';

export interface UserTableStruct {
  name: string,
  email: string,
  ez_id: string,
}

@Component({
  selector: 'app-user-panel',
  imports: [MatTableModule, MatPaginator, CommonModule, MatButtonModule],
  templateUrl: './user-panel.html',
  styleUrl: './user-panel.scss'
})
export class UserPanelComponent {
  displayedColumns: string[] = ['name', 'email', 'ez_id'];
  dataSource_User = new MatTableDataSource<UserTableStruct>([]);
  loading = false;

  @ViewChild(MatPaginator) paginator!: MatPaginator;

  async ngOnInit(): Promise<void> {
    await this.loadUsers();
  }

  ngAfterViewInit() {
    this.dataSource_User.paginator = this.paginator;
  }

  async loadUsers() {
    this.loading = true;
    try {
      const providers: UserTableStruct[] = JSON.parse(await users_fetch_all());
      this.dataSource_User = new MatTableDataSource<UserTableStruct>(providers);
      this.dataSource_User.paginator = this.paginator;
    } catch (error) {
      console.error('Failed to load provider data:', error);
    } finally {
      this.loading = false;
    }
  }

}
