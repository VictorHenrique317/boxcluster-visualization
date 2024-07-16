import { Injectable } from '@angular/core';
import { fs, invoke } from '@tauri-apps/api';
import { Pattern } from 'src/app/models/pattern';
import { environment } from 'src/environments/environment';
import { DialogService } from '../dialog/dialog.service';
import { resolveResource } from '@tauri-apps/api/path';
import { DataPoint } from 'src/app/models/datapoint';
import { IntersectionDetails } from 'src/app/models/intersection_details';

@Injectable({
  providedIn: 'root'
})
export class ApiService {
  constructor(private dialog_service: DialogService) { }

  public async initApplication(tensor_path: string, patterns_path: string){
    await invoke("initApplication", {tensorPath: tensor_path, patternsPath: patterns_path}).catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("ERROR Could not read tensor or patterns.");
      throw error;
    });
  }

  public async getFullRssEvolution(): Promise<Array<number>> {
    console.log("Initializing rss view component");
    console.log("Invoking getFullRssEvolution");

    let rss_evolution;
    if(!environment.dev_mode){
      rss_evolution = await invoke("getFullRssEvolution").catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Could not load rss graph.");
        throw error;
      });

    } else if(environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/rss_evolution.json'));
      rss_evolution = JSON.parse(rawdata);
    }

    console.log("Received rss_evolution:");
    console.log(rss_evolution);

    return rss_evolution;
  }

  public async truncateModel(new_size: number): Promise<any>{
    console.log("Truncating datapoints to only: " + new_size);
    let truncated_datapoints;
    if(!environment.dev_mode){
      await invoke("truncateModel", {newSize: new_size}).catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Error while truncating datapoints.");
        throw error;
      });
  
      truncated_datapoints = await this.getDataPoints();
    }
    else if(environment.dev_mode) {
      let datapoints = await this.getDataPoints(); // Getting all original datapoints in dev mode
      truncated_datapoints = datapoints.slice(0, new_size);
    }

    return truncated_datapoints;
  }

  public async getIntersectionsPercentages(identifier: number): Promise<Map<number, number>> {
    let raw_data;
    if(!environment.dev_mode){
      raw_data = await invoke("getIntersectionsPercentages", {identifier: identifier})
        .catch((error: any) => {
          // console.error(error);
          this.dialog_service.openErrorDialog("Error while getting intersections.");
          throw error;
      });
      
    }else{
      raw_data = await fs.readTextFile(await resolveResource('resources/intersections2.json'));
      raw_data = JSON.parse(raw_data);
    }

    let intersections = new Map<number, number>();
    for (let key in raw_data) { intersections.set(Number(key), Number(raw_data[key])); }

    return intersections;
  }

  public async getIntersectionDetails(identifier: number): Promise<IntersectionDetails>{
    let data: any;
    if(!environment.dev_mode){
      data = await invoke("getIntersectionDetails", {identifier: identifier}).catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Error while fetching intersection details.");
        throw error;
      });

    }else if(environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/intersection_details.json'));
      data = JSON.parse(rawdata);
    }

    let intersections: Map<number, [number, Array<Array<string>>]> = new Map();
    for (let key in data.intersections) { 
      let value = data.intersections[key];
      let percentage = Math.round(value[0]*100)/100;
      let dims_intersections = value[1];
      intersections.set(Number(key), [percentage, dims_intersections]);
    }

    let intersection_details: IntersectionDetails = new IntersectionDetails(
      data.identifier,
      Math.round(data.total_untouched_percentage * 100)/100,
      Math.round(data.total_intersection_percentage * 100)/100,
      intersections
    );

    return intersection_details;
  }

  public async getPattern(identifier: number): Promise<Pattern> {
    let pattern;
    if(!environment.dev_mode){
      pattern = await invoke("getPattern", {identifier: identifier}).catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Error while fetching pattern.");
        throw error;
      });
    }else{
      let rawdata = await fs.readTextFile(await resolveResource('resources/pattern.json'));
      pattern = JSON.parse(rawdata);
      pattern.identifier = identifier;
    }
    
    return Pattern.fromResponse(pattern);
  }

  public async getDataPoints(): Promise<Array<DataPoint>> {
    console.log("Invoking getDataPoints");
    let datapoints;
    if(!environment.dev_mode){
      datapoints = await invoke("getDataPoints").catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Error while fetching data points.");
        throw error;
      });

    } else if (environment.dev_mode){
      let rawdata = await fs.readTextFile(await resolveResource('resources/datapoints2.json'));
      datapoints = JSON.parse(rawdata);
    }

    console.log("Received datapoints:");
    console.log(datapoints);

    return datapoints;
  }

  public async getAllSubpatternsIdentifiers(): Promise<Array<number>> {
    let subpatterns_identifiers;
    if(!environment.dev_mode){
      subpatterns_identifiers = await invoke("getAllSubPatternsIdentifiers").catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Error while fetching subpatterns identifiers.");
        throw error;
      });
    }else{
      let rawdata = await fs.readTextFile(await resolveResource('resources/subpatterns_identifiers.json'));
      subpatterns_identifiers = JSON.parse(rawdata);
    }

    return subpatterns_identifiers;
  }

  public async getDatapointsWithSubPatterns(): Promise<Array<DataPoint>> {
    let datapoints;
    if(!environment.dev_mode){
      datapoints = await invoke("getDatapointsWithSubPatterns").catch((error: any) => {
        // console.error(error);
        this.dialog_service.openErrorDialog("Error while fetching datapoints with subpatterns.");
        throw error;
      });
    }else{
      let rawdata = await fs.readTextFile(await resolveResource('resources/datapoints_with_subpatterns.json'));
      datapoints = JSON.parse(rawdata);
    }

    return datapoints;
  }
  
}
