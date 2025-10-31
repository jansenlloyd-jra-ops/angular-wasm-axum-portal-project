import { Component, signal } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { PortalComponent } from './components/portal/portal.component';
import { SiginInComponent } from './components/sigin-in/sigin-in.component';
@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})
export class App {
  protected readonly title = signal('Admin Portal');
}
