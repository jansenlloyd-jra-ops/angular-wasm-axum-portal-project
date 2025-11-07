import { CommonModule } from '@angular/common';
import { Component, OnInit, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule, MatLabel } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { fetch_all_ez, request_add_user } from '../../../../assets/wasm_backend/wasm_backend';
import { EncryptionZoneStruct } from '../add-user-dialog/add-user-dialog';

@Component({
  selector: 'app-edit-user-dialog',
  imports: [MatFormFieldModule, MatInputModule, MatSelectModule, MatLabel, MatButtonModule, CommonModule, FormsModule],
  templateUrl: './edit-user-dialog.html',
  styleUrl: './edit-user-dialog.scss'
})
export class EditUserDialog implements OnInit {
  name = '';
  email = '';
  encryption_zone: EncryptionZoneStruct[] = [];
  selectedEz: EncryptionZoneStruct | null = null;
  requesting = signal<boolean>(false);
  addUserResponse: any = {};
  async ngOnInit(): Promise<void> {
    try {
      this.encryption_zone = JSON.parse(await fetch_all_ez());
    } catch (e) {
      console.error('Failed to fetch encryption zones', e);
    }
  };

  async submitEditUser() {
    this.requesting.set(true);
    if (this.name != '' && this.email != '' && this.selectedEz != null) {
      let res = await request_add_user(this.name, this.email, this.selectedEz.ez_id);
      this.addUserResponse = JSON.parse(res);
    }
    setTimeout(() => {
      this.requesting.set(false);
      this.clear()
    }, 500);
  };

  async clear() {
    this.name = '';
    this.email = '';
    this.selectedEz = null;
  }
}
