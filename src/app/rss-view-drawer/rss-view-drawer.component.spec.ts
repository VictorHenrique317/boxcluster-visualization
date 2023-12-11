import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RssViewDrawerComponent } from './rss-view-drawer.component';

describe('RssViewDrawerComponent', () => {
  let component: RssViewDrawerComponent;
  let fixture: ComponentFixture<RssViewDrawerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ RssViewDrawerComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(RssViewDrawerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
