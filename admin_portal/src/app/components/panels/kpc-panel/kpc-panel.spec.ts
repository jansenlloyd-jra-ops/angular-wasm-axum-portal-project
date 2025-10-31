import { ComponentFixture, TestBed } from '@angular/core/testing';

import { KpcPanelComponent } from './kpc-panel';

describe('KpcPanelComponent', () => {
  let component: KpcPanelComponent;
  let fixture: ComponentFixture<KpcPanelComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [KpcPanelComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(KpcPanelComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
