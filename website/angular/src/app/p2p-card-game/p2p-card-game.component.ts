import { Component, OnInit } from '@angular/core';


declare var init: any;

@Component({
  selector: 'app-p2p-card-game',
  imports: [],
  templateUrl: './p2p-card-game.component.html',
  styleUrl: './p2p-card-game.component.scss'
})
export class P2pCardGameComponent implements OnInit {


  async ngOnInit() {
    try {
      const wasmModule = await import('../../../public/wasm.js'); // Adjust path as needed
      (wasmModule as any).default(); // Call an exported function from the module
    } catch (error) {
      console.error('Error loading WASM module:', error);
    }
  }

}
