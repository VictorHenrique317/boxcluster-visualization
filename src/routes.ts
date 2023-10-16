import { Route } from '@angular/router';
import { DagComponent } from './app/dag/dag.component';
import { RssViewComponent } from './app/rss-view/rss-view.component';

export const routes: Route[] = [
  // {
  //   path: '',
  //   component: DagComponent,
  // },
  {
    path: 'dagview',
    component: DagComponent,
  },
  {
    path: 'rssview',
    component: RssViewComponent,
  },
  {
    path: '**',
    redirectTo: '',
  },
];