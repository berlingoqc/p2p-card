import { ComponentFixture, TestBed } from '@angular/core/testing';

import { P2pCardGameComponent } from './p2p-card-game.component';

describe('P2pCardGameComponent', () => {
  let component: P2pCardGameComponent;
  let fixture: ComponentFixture<P2pCardGameComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [P2pCardGameComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(P2pCardGameComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
