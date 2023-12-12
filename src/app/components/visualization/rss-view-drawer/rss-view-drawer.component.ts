import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';

@Component({
  selector: 'app-rss-view-drawer',
  standalone: true,
  imports: [CommonModule, MatButtonModule],
  templateUrl: './rss-view-drawer.component.html',
  styleUrls: ['./rss-view-drawer.component.scss']
})
export class RssViewDrawerComponent {

}
