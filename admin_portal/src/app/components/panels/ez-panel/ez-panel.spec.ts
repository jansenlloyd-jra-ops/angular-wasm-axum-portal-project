import { ComponentFixture, TestBed } from '@angular/core/testing';

import { EzPanelComponent } from './ez-panel';

describe('EzPanelComponent', () => {
  let component: EzPanelComponent;
  let fixture: ComponentFixture<EzPanelComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [EzPanelComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(EzPanelComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
