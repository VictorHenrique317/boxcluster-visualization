import { Directive, ViewContainerRef } from '@angular/core';

@Directive({
  selector: '[appPortal]',
})
export class PortalDirective {
  private viewContainerRef: ViewContainerRef;

  constructor(viewContainerRef: ViewContainerRef) {
    this.viewContainerRef = viewContainerRef;
  }

  attachComponent(component: any) {
    this.viewContainerRef.clear();
    this.viewContainerRef.createComponent(component);
  }
}
