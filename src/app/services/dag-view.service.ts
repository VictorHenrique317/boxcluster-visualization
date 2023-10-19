import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/tauri';
import { Subject } from 'rxjs';
import { DataPoint } from 'src/models/datapoint';

@Injectable({
  providedIn: 'root'
})
export class DagViewService {
  private datapoints = new Subject<Array<DataPoint>>();
  public datapoints$ = this.datapoints.asObservable();
  
  constructor() { }

  private changeDataPoints(new_datapoints: Array<DataPoint>){
    this.datapoints.next(new_datapoints);
  }

  public truncateDataPoints(new_size: number) {
    invoke("truncateModel", {newSize: new_size}).then((result: any) => {
      this.updateDataPoints();
    });
  }

  public updateDataPoints(){
    invoke("getDataPoints").then((result: Array<DataPoint>) =>{
      this.changeDataPoints(result);
    });
  }

  // public getDataPoints(): Array<DataPoint>{
  //   return this.datapoints;
  // }
}
