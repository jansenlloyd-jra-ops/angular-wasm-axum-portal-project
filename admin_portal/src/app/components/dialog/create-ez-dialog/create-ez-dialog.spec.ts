import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CreateEzDialog } from './create-ez-dialog';

describe('CreateEzDialog', () => {
  let component: CreateEzDialog;
  let fixture: ComponentFixture<CreateEzDialog>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CreateEzDialog]
    })
    .compileComponents();

    fixture = TestBed.createComponent(CreateEzDialog);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
