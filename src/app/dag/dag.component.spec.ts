import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DagComponent } from './dag.component';

describe('DagComponent', () => {
  let component: DagComponent;
  let fixture: ComponentFixture<DagComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ DagComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DagComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
