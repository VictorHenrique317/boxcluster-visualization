import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PatternDimDialogComponent } from './pattern-dim-dialog.component';

describe('PatternDimDialogComponent', () => {
  let component: PatternDimDialogComponent;
  let fixture: ComponentFixture<PatternDimDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ PatternDimDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PatternDimDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
