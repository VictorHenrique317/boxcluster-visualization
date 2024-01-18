import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DataPointTooltipComponent } from './datapoint-tooltip.component';

describe('DatapointTooltipComponent', () => {
  let component: DataPointTooltipComponent;
  let fixture: ComponentFixture<DataPointTooltipComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ DataPointTooltipComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DataPointTooltipComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
