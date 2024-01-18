import { Route } from '@angular/router';
import { VisualizationComponent } from './app/components/visualization/visualization.component';
import { RssViewComponent } from './app/components/main_options/rss-view/rss-view.component';

export const routes: Route[] = [
  // {
  //   path: '',
  //   component: DagComponent,
  // },
  {
    path: 'visualizationView',
    component: VisualizationComponent,
  },
  {
    path: 'rssView',
    component: RssViewComponent,
  },
  {
    path: '**',
    redirectTo: '',
  },
];