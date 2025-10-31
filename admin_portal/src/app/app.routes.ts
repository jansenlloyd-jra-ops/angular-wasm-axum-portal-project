import { Routes } from '@angular/router';
import { SiginInComponent } from './components/sigin-in/sigin-in.component';
import { PortalComponent } from './components/portal/portal.component';

export const routes: Routes = [
  { path: '', component: SiginInComponent },
  { path: 'dashboard', component: PortalComponent },
  { path: '**', redirectTo: ''}
];
