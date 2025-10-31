import { Component } from '@angular/core';

export interface ProvidersTableStruct {
  name: string,
  kpc_id: string,
  configuration: string,
}

@Component({
  selector: 'app-providers-panel',
  imports: [],
  templateUrl: './providers-panel.html',
  styleUrl: './providers-panel.scss'
})
export class ProvidersPanelComponent {

}
