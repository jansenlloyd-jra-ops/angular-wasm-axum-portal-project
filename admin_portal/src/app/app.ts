import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { PortalComponent } from './components/portal.component';
@Component({
  selector: 'app-root',
  imports: [RouterOutlet, PortalComponent],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App {
  protected readonly title = signal('admin_portal');
}
