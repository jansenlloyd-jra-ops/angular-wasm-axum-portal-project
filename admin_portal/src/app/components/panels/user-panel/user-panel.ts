import { Component, ViewChild } from '@angular/core';
import { MatTableModule, MatTableDataSource } from '@angular/material/table';
import { MatPaginator } from '@angular/material/paginator';

export interface UserTableStruct {
  name: string,
  email: string,
  kpc_id: string,
}

export const USER_TABLE_DATA: UserTableStruct[] = [
  { name: 'Alice Johnson', email: 'alice.johnson@example.com', kpc_id: 'c3a59b16-1f6e-4c12-9f0d-5b74b1f146ae' },
  { name: 'Brian Kim', email: 'brian.kim@example.com', kpc_id: '7d0a9323-92ff-4de8-9a67-38e9c6e10b7e' },
  { name: 'Carla Mendoza', email: 'carla.mendoza@example.com', kpc_id: '2fdd89ac-3a47-4b5f-8a11-9cfb2d94a052' },
  { name: 'David Smith', email: 'david.smith@example.com', kpc_id: 'b4c76267-ef1b-4e63-bb03-1af5a6c4a1d4' },
  { name: 'Alice Johnson', email: 'alice@example.com', kpc_id: 'b4c76267-ef1b-4e63-bb03-1af5a6c4a1d4' },
  { name: 'Bob Smith', email: 'bob@example.com', kpc_id: 'b4c76267-ef1b-4e63-bb03-1af5a6c4a1d4' },
  { name: 'Carol White', email: 'carol@example.com', kpc_id: '2fdd89ac-3a47-4b5f-8a11-9cfb2d94a052' },
  { name: 'David Brown', email: 'david@example.com', kpc_id: '2fdd89ac-3a47-4b5f-8a11-9cfb2d94a052' },
  { name: 'Eve Green', email: 'eve@example.com', kpc_id: '7d0a9323-92ff-4de8-9a67-38e9c6e10b7e' },
  { name: 'Frank Blue', email: 'frank@example.com', kpc_id: '7d0a9323-92ff-4de8-9a67-38e9c6e10b7e' },
];

@Component({
  selector: 'app-user-panel',
  imports: [MatTableModule, MatPaginator],
  templateUrl: './user-panel.html',
  styleUrl: './user-panel.scss'
})
export class UserPanelComponent {
  displayedColumns: string[] = ['name', 'email', 'kpc_id'];
  dataSource_User = new MatTableDataSource<UserTableStruct>(USER_TABLE_DATA);

  @ViewChild(MatPaginator) paginator!: MatPaginator;

  ngAfterViewInit() {
    this.dataSource_User.paginator = this.paginator;
  }

}
