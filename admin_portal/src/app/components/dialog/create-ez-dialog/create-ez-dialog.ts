import { Component, signal } from '@angular/core';
import { fetch_all_kpc, request_create_ez } from '../../../../assets/wasm_backend/wasm_backend';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule, MatLabel } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';

export interface KeyProvidersStruct {
  name: string;
  provider: string;
  kpc_id: string;
  configuration: string;
}


@Component({
  selector: 'app-create-ez-dialog',
  imports: [MatFormFieldModule, MatInputModule, MatSelectModule, MatLabel, MatButtonModule, CommonModule, FormsModule],
  templateUrl: './create-ez-dialog.html',
  styleUrl: './create-ez-dialog.scss'
})
export class CreateEzDialog {
  name = '';
  key_providers: KeyProvidersStruct[] = [];
  selectedKPC: KeyProvidersStruct | null = null;
  requesting = signal<boolean>(false);
  createEZResponse: any = {};
  async ngOnInit(): Promise<void> {
    try {
      this.key_providers = JSON.parse(await fetch_all_kpc());
    } catch (e) {
      console.error('Failed to fetch encryption zones', e);
    }
  };

  async submitCreateEZ() {
    this.requesting.set(true);
    if (this.name != ''  && this.selectedKPC != null) {
      let res = await request_create_ez(this.name, this.selectedKPC.kpc_id);
      this.createEZResponse = JSON.parse(res);
    }
    setTimeout(() => {
      this.requesting.set(false);
      this.clear()
    }, 500);
  };

  async clear() {
    this.name = '';
    this.selectedKPC = null;
  }
}
