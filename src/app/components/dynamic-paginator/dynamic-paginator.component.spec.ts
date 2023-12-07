import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DynamicPaginatorComponent } from './dynamic-paginator.component';

describe('DynamicPaginatorComponent', () => {
  let component: DynamicPaginatorComponent;
  let fixture: ComponentFixture<DynamicPaginatorComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ DynamicPaginatorComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DynamicPaginatorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
