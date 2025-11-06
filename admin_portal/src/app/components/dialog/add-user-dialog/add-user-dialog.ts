import { CommonModule } from '@angular/common';
import { Component, OnInit, signal } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule, MatLabel } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { FormsModule } from "@angular/forms";
import { ez_fetch_all, add_user_request } from '../../../../assets/wasm_backend/wasm_backend';

export interface EncryptionZoneStruct {
  zone_name: string,
  ez_id: string,
  kpc_id: string,
}

@Component({
  selector: 'app-add-user-dialog',
  imports: [MatFormFieldModule, MatInputModule, MatSelectModule, MatLabel, MatButtonModule, CommonModule, FormsModule],
  templateUrl: './add-user-dialog.html',
  styleUrl: './add-user-dialog.scss'
})
export class AddUserDialog implements OnInit {
  name = '';
  email = '';
  encryption_zone: EncryptionZoneStruct[] = [];
  selectedEz: EncryptionZoneStruct | null = null;
  requesting = signal<boolean>(false);
  addUserResponse: any = {};
  async ngOnInit(): Promise<void> {
    try {
      this.encryption_zone = JSON.parse(await ez_fetch_all());
    } catch (e) {
      console.error('Failed to fetch encryption zones', e);
    }
  };
  
  async submitAddUser() {
    this.requesting.set(true);
    if (this.name != '' && this.email != '' && this.selectedEz != null) {
      let res = await add_user_request(this.name, this.email, this.selectedEz.ez_id);
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
