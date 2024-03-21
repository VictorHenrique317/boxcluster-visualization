import { ComponentFixture, TestBed } from '@angular/core/testing';

import { IntersectionDetailsDialogComponent } from './intersection-details-dialog.component';

describe('IntersectionDetailsDialogComponent', () => {
  let component: IntersectionDetailsDialogComponent;
  let fixture: ComponentFixture<IntersectionDetailsDialogComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ IntersectionDetailsDialogComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(IntersectionDetailsDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
