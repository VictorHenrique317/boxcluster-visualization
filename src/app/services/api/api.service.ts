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
    rss_evolution = await invoke("getFullRssEvolution").catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Could not load rss graph.");
      throw error;
    });

    console.log("Received rss_evolution:");
    console.log(rss_evolution);

    return rss_evolution;
  }

  public async truncateModel(new_size: number): Promise<any>{
    console.log("Truncating datapoints to only: " + new_size);
    let truncated_datapoints;
    await invoke("truncateModel", {newSize: new_size}).catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while truncating datapoints.");
      throw error;
    });

    truncated_datapoints = await this.getDataPoints();

    return truncated_datapoints;
  }

  public async getIntersectionDetails(identifier: number): Promise<IntersectionDetails>{
    let data: any;
    data = await invoke("getIntersectionDetails", {identifier: identifier}).catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while fetching intersection details.");
      throw error;
    });

    let intersections: Map<number, [number, Array<Array<string>>]> = new Map();
    for (let key in data.intersections) { 
      let value = data.intersections[key];
      let percentage = Math.round(value[0]*100000)/100000;
      let dims_intersections = value[1];
      intersections.set(Number(key), [percentage, dims_intersections]);
    }

    let intersection_details: IntersectionDetails = new IntersectionDetails(
      data.identifier,
      Math.round(data.total_untouched_percentage * 10000)/10000,
      Math.round(data.total_intersection_percentage * 10000)/10000,
      intersections
    );

    return intersection_details;
  }

  public async getPattern(identifier: number): Promise<Pattern> {
    let pattern;
    pattern = await invoke("getPattern", {identifier: identifier}).catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while fetching pattern.");
      throw error;
    });
    
    return Pattern.fromResponse(pattern);
  }

  public async getDataPoints(): Promise<Array<DataPoint>> {
    console.log("Invoking getDataPoints");
    let datapoints;
    datapoints = await invoke("getDataPoints").catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while fetching data points.");
      throw error;
    });

    console.log("Received datapoints:");
    console.log(datapoints);

    return datapoints;
  }

  public async getAllSubpatternsIdentifiers(): Promise<Array<number>> {
    let subpatterns_identifiers;
    subpatterns_identifiers = await invoke("getAllSubPatternsIdentifiers").catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while fetching subpatterns identifiers.");
      throw error;
    });

    return subpatterns_identifiers;
  }

  public async getDatapointsWithSubPatterns(): Promise<Array<DataPoint>> {
    let datapoints;
    datapoints = await invoke("getDatapointsWithSubPatterns").catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while fetching datapoints with subpatterns.");
      throw error;
    });

    return datapoints;
  }

  public async descendDag(identifier: number): Promise<Array<DataPoint>> {
    let datapoints;
    datapoints = await invoke("descendDag", {nextIdentifier: identifier}).catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while descending DAG.");
      throw error;
    });

    return datapoints;
  }

  public async ascendDag(): Promise<Array<DataPoint>> {
    let datapoints;
    datapoints = await invoke("ascendDag").catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while ascending DAG.");
      throw error;
    });

    return datapoints;
  }

  public async getCurrentLevelBackgroundDensity(): Promise<number> {
    let density;
    density = await invoke("getCurrentLevelBackgroundDensity").catch((error: any) => {
      // console.error(error);
      this.dialog_service.openErrorDialog("Error while fetching background density.");
      throw error;
    });

    return density;
  }

  public async getAllDimsValues(): Promise<string[][]> {
    let dims_values;
    dims_values = await invoke("getAllDimsValues").catch((error: any) => {
      this.dialog_service.openErrorDialog("Error while fetching dimensions values.");
      throw error;
    });

    return dims_values;
  }

  public async filterDatapoints(filters: string[][]): Promise<DataPoint[]> {
    let datapoints;
    datapoints = await invoke("filterDatapoints", {filters: filters}).catch((error: any) => {
      this.dialog_service.openErrorDialog("Error while filtering datapoints.");
      throw error;
    });

    return datapoints;
  }

  public async getNbOfSubpatterns(identifier: number){
    let nb_subpatterns;
    nb_subpatterns = await invoke("getNbOfSubpatterns", {identifier: identifier}).catch((error: any) => {
      this.dialog_service.openErrorDialog("Error while fetching number of subpatterns.");
      throw error;
    });

    return nb_subpatterns;
  }
}
