import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PatternSummaryComponent } from './pattern-summary.component';

describe('PatternSummaryComponent', () => {
  let component: PatternSummaryComponent;
  let fixture: ComponentFixture<PatternSummaryComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ PatternSummaryComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PatternSummaryComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
