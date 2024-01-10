import { AfterViewInit, Component, ComponentRef, ElementRef, EventEmitter, Input, Output, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatListModule } from '@angular/material/list';
import { MatRippleModule } from '@angular/material/core';
import { Pattern } from 'src/app/models/pattern';

@Component({
  selector: 'app-pattern-summary',
  standalone: true,
  templateUrl: './pattern-summary.component.html',
  styleUrls: ['./pattern-summary.component.scss'],
  imports: [
    CommonModule,
    MatListModule,
    MatRippleModule],
})
export class PatternSummaryComponent implements AfterViewInit{
  @Input() id: number;
  @Input() pattern_id: number;
  @Input() pattern: Pattern;
  @Input() available_height: number; // 620
  
  @ViewChild("matListItemContainer") matListItemContainer: ElementRef<HTMLElement>;

  @Output() page_size = new EventEmitter<number>();

  private height: number;
  public hidden = false;
  
  private padding_top = 8;
  private padding_bottom = 16;
  private mat_list_padding = 0; // 8
  private paginator_options_padding = 16;
  
  ngAfterViewInit(){
    this.height = this.matListItemContainer.nativeElement.clientHeight;
    this.height += this.padding_top;
    this.height += this.padding_bottom;

    this.available_height -= this.mat_list_padding * 2 + this.paginator_options_padding;
    let page_size = Math.floor(this.available_height / this.height);

    this.page_size.emit(page_size);
  }

}
