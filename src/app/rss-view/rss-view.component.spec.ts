import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RssViewComponent } from './rss-view.component';

describe('RssViewComponent', () => {
  let component: RssViewComponent;
  let fixture: ComponentFixture<RssViewComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ RssViewComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(RssViewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
