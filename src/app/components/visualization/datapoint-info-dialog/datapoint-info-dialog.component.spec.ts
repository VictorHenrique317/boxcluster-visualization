import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DatapointInfoDialogComponent } from './datapoint-info-dialog.component';

describe('DatapointInfoDialogComponent', () => {
  let component: DatapointInfoDialogComponent;
  let fixture: ComponentFixture<DatapointInfoDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ DatapointInfoDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DatapointInfoDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
