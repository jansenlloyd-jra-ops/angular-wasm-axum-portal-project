import { Component, ViewChild } from '@angular/core';
import { MatTableModule, MatTableDataSource } from '@angular/material/table';
import { MatPaginator } from '@angular/material/paginator';


export interface LogsTableStruct {
  time: string,
  message: string,
}

export const LOGS_TABLE_DATA: LogsTableStruct[] = [
  { time: '2025-10-28T08:15:32Z', message: 'User admin logged in successfully.' },
  { time: '2025-10-28T08:17:04Z', message: 'Key Provider Configuration created for Google Cloud.' },
  { time: '2025-10-28T08:18:50Z', message: 'Tenant "AcmeCorp" updated encryption settings.' },
  { time: '2025-10-28T08:20:10Z', message: 'AWS provider configuration validation passed.' },
  { time: '2025-10-28T08:21:43Z', message: 'Azure API key revoked due to inactivity.' },
  { time: '2025-10-28T08:23:12Z', message: 'Client request received from 192.168.1.42.' },
  { time: '2025-10-28T08:24:39Z', message: 'Error: Invalid JSON input detected in configuration form.' },
  { time: '2025-10-28T08:26:18Z', message: 'System health check completed — all services operational.' },
  { time: '2025-10-28T08:28:04Z', message: 'User jdoe assigned as encryption admin.' },
  { time: '2025-10-28T08:30:00Z', message: 'Backup process completed successfully.' }
];

@Component({
  selector: 'logs-table',
  imports: [MatTableModule, MatPaginator],
  templateUrl: './logs-table.html',
  styleUrl: './logs-table.scss'
})
export class LogsTableComponent {
  displayedColumns: string[] = ['time', 'message'];
  dataSource_Logs = new MatTableDataSource<LogsTableStruct>(LOGS_TABLE_DATA);

  @ViewChild(MatPaginator) paginator!: MatPaginator;

  ngAfterViewInit() {
    this.dataSource_Logs.paginator = this.paginator;
  }
}
