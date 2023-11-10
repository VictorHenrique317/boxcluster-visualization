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

  public updateDataPoints(){
    // invoke("getDataPoints").then((result: Array<DataPoint>) =>{
    //   this.changeDataPoints(result);
    // });

    let datapoints: DataPoint[] = [
      // new DataPoint(1, 5.000000476837158, 2, -0.09219828993082047, -0.13575132191181183, 185, 70, 0),
      // new DataPoint(2, 5.000000476837158, 2, -0.1434944123029709, -0.025769922882318497, 172, 83, 0),
      // new DataPoint(3, 5.000000476837158, 2, 0.13556334376335144, 0.04386178404092789, 172, 83, 0),
      // new DataPoint(4, 5.000000476837158, 2, -0.07058414816856384, 0.12081196159124374, 172, 83, 0),
      // new DataPoint(5, 5.000000476837158, 2, -0.10426267236471176, 0.050395093858242035, 170, 85, 0),
      // new DataPoint(6, 5.000000476837158, 2, 0.019231455400586128, 0.12873762845993042, 167, 88, 0),
      // new DataPoint(7, 5.313292980194092, 2, -0.012485436163842678, -0.1166980192065239, 159, 96, 0),
      // new DataPoint(8, 5.000000476837158, 2, 0.0902477353811264, -0.09746352583169937, 166, 89, 0),
      // new DataPoint(9, 5.000000476837158, 2, 0.11125318706035614, -0.029057452455163002, 164, 91, 0),
      // new DataPoint(10, 5.000000476837158, 2, 0.07510804384946823, 0.07280625402927399, 164, 91, 0),
      // new DataPoint(11,10.591045379638672 ,2 ,0.017545919865369797 ,-0.0036888536997139454 ,108 ,147 ,0 ),
      // new DataPoint(12 ,9.654894828796387 ,2 ,-0.0016832565888762474 ,-0.0028650283347815275 ,103 ,152 ,0 ),
      // new DataPoint(13 ,8.077374458312988 ,2 ,-0.022442487999796867 ,0.0031338557600975037 ,119 ,136 ,0 ),
      // new DataPoint(14 ,13.373766899108887 ,2 ,0.0032756526488810778 ,0.0005078032845631242 ,101 ,154 ,0 ),
      // new DataPoint(15 ,12.493330955505371 ,2 ,-0.01358407735824585 ,-0.009488344192504883 ,103 ,152 ,0 ),
      // new DataPoint(16 ,9.654894828796387 ,2 ,-0.020130377262830734 ,-0.032513659447431564 ,112 ,143 ,0 ),
      // new DataPoint(17 ,8.406118392944336 ,2 ,-0.012485436163842678 ,-0.03319296985864639 ,117 ,138 ,0 ),
      // new DataPoint(18 ,8.88748836517334 ,2 ,-0.010542476549744606 ,-0.021444378420710564 ,114 ,141 ,0 ),
      // new DataPoint(19 ,8.737260818481445 ,2 ,-0.010542476549744606 ,-0.021444378420710564 ,116 ,139 ,0 ),
      // new DataPoint(20 ,8.88748836517334  ,2 ,-0.020931201055645943 ,-0.015955956652760506, 114, 141, 0)

      new DataPoint(1, 5.000000476837158, 2, -1, 1, 185, 70, 0),
      new DataPoint(2, 5.000000476837158, 2, 1, 1, 172, 83, 0),
      new DataPoint(3, 5.000000476837158, 2, -1, -1, 172, 83, 0),
      new DataPoint(4, 5.000000476837158, 2, 1, -1, 172, 83, 0),
      new DataPoint(4, 5.000000476837158, 2, 0, 0, 172, 83, 0),

    ];
    this.changeDataPoints(datapoints);
  }

  public truncateDataPoints(new_size: number) {
    invoke("truncateModel", {newSize: new_size}).then((result: any) => {
      this.updateDataPoints();
    });
  }
}
