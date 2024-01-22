import { Component, ElementRef, Input, Renderer2 } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DataPoint } from 'src/app/models/datapoint';

@Component({
  selector: 'app-datapoint-tooltip',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './datapoint-tooltip.component.html',
  styleUrls: ['./datapoint-tooltip.component.scss']
})
export class DataPointTooltipComponent {
  protected visible: boolean = false;
  private datapoint: DataPoint;

  constructor(private elementRef: ElementRef, private renderer: Renderer2) {}

  ngOnInit(): void {
    console.log(this.datapoint)
  }

  public toggleVisibility(){
    this.visible = !this.visible;
  }

  public setDatapoint(datapoint: DataPoint){
    this.datapoint = datapoint;
  }

  public setPosition(top: number, left: number) {
    let top_str = top + 'px';
    let left_str = left + 'px';
    this.renderer.setStyle(this.elementRef.nativeElement, 'top', top_str);
    this.renderer.setStyle(this.elementRef.nativeElement, 'left', left_str);
  }
}
