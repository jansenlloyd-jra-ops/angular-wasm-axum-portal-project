import { ComponentFixture, TestBed } from '@angular/core/testing';

import { KeyProviderConfigComponent } from './key-provider-config';

describe('KeyProviderConfig', () => {
  let component: KeyProviderConfigComponent;
  let fixture: ComponentFixture<KeyProviderConfigComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [KeyProviderConfigComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(KeyProviderConfigComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
