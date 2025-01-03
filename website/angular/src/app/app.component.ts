import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import { P2pCardGameComponent } from './p2p-card-game/p2p-card-game.component';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, P2pCardGameComponent],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent {
  title = 'angular';
}
