import { TestBed } from '@angular/core/testing';

import { DagViewService } from './dag-view.service';

describe('DagViewService', () => {
  let service: DagViewService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DagViewService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
