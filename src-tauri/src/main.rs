#![allow(non_snake_case)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use boxcluster_visualization::{self, controller::states::states::*, database::{pattern::Pattern, datapoint::DataPoint}};
use tauri::State;
use boxcluster_visualization::common::generic_error::GenericError;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

//////////////////////////// Paginator ////////////////////////////

#[tauri::command]
fn getSoundingPattern(paginator_service: State<PaginatorServiceState>, application_service: State<ApplicationServiceState>) 
        -> Result<Pattern, GenericError> {

    println!("Calling getSoundingPattern...");

    let paginator_service = match paginator_service.0.lock() {
        Ok(paginator_service) => paginator_service,
        Err(_) => {
            let error = GenericError::new("Could not lock paginator service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let identifier_mapper = match application_service.getIdentifierMapper() {
        Ok(identifier_mapper) => identifier_mapper,
        Err(error) => {
            error.print();
            return Err(error);
        }
    };

    match paginator_service.getSoundingPattern(identifier_mapper) {
        Ok(pattern) => return Ok(pattern),
        Err(error) => return Err(error),
    }
}

#[tauri::command]
fn refreshPageSize(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>, page_size: u32) -> Result<(Vec<Pattern>, u32, u32), GenericError> {
    
    println!("Calling refreshPageSize...");

    let mut paginator_service = match paginator_service.0.lock() {
        Ok(paginator_service) => paginator_service,
        Err(_) => {
            let error = GenericError::new("Could not lock paginator service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let identifier_mapper = match application_service.getIdentifierMapper() {
        Ok(identifier_mapper) => identifier_mapper,
        Err(error) => {
            error.print();
            return Err(error);
        }
    };

    match paginator_service.refreshPageSize(identifier_mapper, page_size) {
        Ok(patterns) => return Ok(patterns),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

#[tauri::command]
fn goToPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>, page_index: u32) -> Result<(Vec<Pattern>, u32, u32), GenericError> {

    println!("Calling goToPage...");

    let mut paginator_service = match paginator_service.0.lock() {
        Ok(paginator_service) => paginator_service,
        Err(_) => {
            let error = GenericError::new("Could not lock paginator service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let identifier_mapper = match application_service.getIdentifierMapper() {
        Ok(identifier_mapper) => identifier_mapper,
        Err(error) => {
            error.print();
            return Err(error);
        }
    };

    match paginator_service.goToPage(identifier_mapper, &page_index) {
        Ok(patterns) => return Ok(patterns),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

#[tauri::command]
fn goToFirstPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<Pattern>, u32, u32), GenericError> {

    println!("Calling goToFirstPage...");

    let mut paginator_service = match paginator_service.0.lock() {
        Ok(paginator_service) => paginator_service,
        Err(_) => {
            let error = GenericError::new("Could not lock paginator service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let identifier_mapper = match application_service.getIdentifierMapper() {
        Ok(identifier_mapper) => identifier_mapper,
        Err(error) => {
            error.print();
            return Err(error);
        }
    };

    let first_page = paginator_service.first_page.clone();
    match paginator_service.goToPage(identifier_mapper, &first_page) {
        Ok(patterns) => return Ok(patterns),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

#[tauri::command]
fn goToLastPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<Pattern>, u32, u32), GenericError> {

    println!("Calling goToLastPage...");

    let mut paginator_service = match paginator_service.0.lock() {
        Ok(paginator_service) => paginator_service,
        Err(_) => {
            let error = GenericError::new("Could not lock paginator service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let last_page = paginator_service.last_page.clone();
    let identifier_mapper = match application_service.getIdentifierMapper() {
        Ok(identifier_mapper) => identifier_mapper,
        Err(error) => {
            error.print();
            return Err(error);
        }
    };

    match paginator_service.goToPage(identifier_mapper, &last_page) {
        Ok(patterns) => return Ok(patterns),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

#[tauri::command]
fn nextPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<Pattern>, u32, u32), GenericError> {

    println!("Calling nextPage...");

    let mut paginator_service = match paginator_service.0.lock() {
        Ok(paginator_service) => paginator_service,
        Err(_) => {
            let error = GenericError::new("Could not lock paginator service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let identifier_mapper = match application_service.getIdentifierMapper() {
        Ok(identifier_mapper) => identifier_mapper,
        Err(error) => {
            error.print();
            return Err(error);
        }
    };

    match paginator_service.nextPage(identifier_mapper) {
        Ok(patterns) => return Ok(patterns),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

#[tauri::command]
fn previousPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<Pattern>, u32, u32), GenericError> {

    println!("Calling previousPage...");

    let mut paginator_service = match paginator_service.0.lock() {
        Ok(paginator_service) => paginator_service,
        Err(_) => {
            let error = GenericError::new("Could not lock paginator service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    let identifier_mapper = match application_service.getIdentifierMapper() {
        Ok(identifier_mapper) => identifier_mapper,
        Err(error) => {
            error.print();
            return Err(error);
        }
    };

    match paginator_service.previousPage(identifier_mapper) {
        Ok(patterns) => return Ok(patterns),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

//////////////////////////// Application ////////////////////////////
#[tauri::command]
fn initApplication(application_service: State<ApplicationServiceState>, tensor_path: String, patterns_path: String) 
        -> Result<(), GenericError> {
    println!("Calling changeTensor...");

    let mut application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    match application_service.init(&tensor_path, &patterns_path) {
        Ok(_) => return Ok(()),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

#[tauri::command]
fn changePatterns(application_service: State<ApplicationServiceState>, patterns_path: String) 
        -> Result<(), GenericError> {
    println!("Calling changePatterns...");

    let mut application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    match application_service.changePatterns(&patterns_path) {
        Ok(_) => return Ok(()),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

#[tauri::command]
fn truncateModel(application_service: State<ApplicationServiceState>, new_size: u32) 
        -> Result<(), GenericError>{
    println!("Calling truncateModel...");

    let mut application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    match application_service.truncateModel(&new_size) {
        Ok(_) => return Ok(()),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

#[tauri::command]
fn getFullRssEvolution(application_service: State<ApplicationServiceState>) -> Result<Vec<f64>, GenericError> {
    println!("Calling getFullRssEvolution...");

    let application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    match application_service.getFullRssEvolution() {
        Ok(rss_evolution) => return Ok(rss_evolution),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

#[tauri::command]
fn getTruncatedRssEvolution(application_service: State<ApplicationServiceState>) -> Result<Vec<f64>, GenericError> {
    println!("Calling getTruncatedRssEvolution...");

    let application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    match application_service.getTruncatedRssEvolution() {
        Ok(rss_evolution) => return Ok(rss_evolution),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

#[tauri::command]
fn descendDag(application_service: State<ApplicationServiceState>, next_identifier: u32) -> Result<(), GenericError>{
    println!("Calling descendDag...");

    let mut application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    match application_service.descendDag(&next_identifier) {
        Ok(_) => return Ok(()),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

#[tauri::command]
fn ascendDag(application_service: State<ApplicationServiceState>) -> Result<(), GenericError>{
    println!("Calling ascendDag...");

    let mut application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };
    
    match application_service.ascendDag() {
        Ok(_) => return Ok(()),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

#[tauri::command]
fn getDataPoints(application_service: State<ApplicationServiceState>) -> Result<Vec<DataPoint>, GenericError> {
    println!("Calling getDataPoints...");

    let application_service = match application_service.0.lock() {
        Ok(application_service) => application_service,
        Err(_) => {
            let error = GenericError::new("Could not lock application service", file!(), &line!());
            error.print();
            return Err(error);
        }
    };

    match application_service.getDataPoints() {
        Ok(data_points) => return Ok(data_points),
        Err(error) => {
            error.print();
            return Err(error)
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(ApplicationServiceState(Default::default()))
        .manage(PaginatorServiceState(Default::default()))
        .invoke_handler(tauri::generate_handler![ 
            getSoundingPattern,
            refreshPageSize,
            goToPage,
            goToFirstPage,
            goToLastPage,
            nextPage,
            previousPage,

            initApplication,
            changePatterns,
            ascendDag,
            descendDag,
            truncateModel,
            getFullRssEvolution,
            getTruncatedRssEvolution,
            getDataPoints
            ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");

    // boxcluster_visualization::main()
}
