use tauri::State;
use crate::{common::generic_error::GenericError, controller::states::states::{ApplicationServiceState, PaginatorServiceState}, database::raw_pattern::RawPattern};

#[tauri::command]
pub fn getSoundingPattern(paginator_service: State<PaginatorServiceState>, application_service: State<ApplicationServiceState>) 
        -> Result<RawPattern, GenericError> {

    println!("Calling getSoundingPattern...");

    let paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.getSoundingPattern(identifier_mapper, application_service.getTranslator());
}

#[tauri::command]
pub fn refreshPageSize(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>, page_size: u32) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {
    
    println!("Calling refreshPageSize...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.refreshPageSize(identifier_mapper, application_service.getTranslator(), page_size);
}

#[tauri::command]
pub fn goToPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>, page_index: u32) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling goToPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.goToPage(identifier_mapper, application_service.getTranslator(), &page_index);
}

#[tauri::command]
pub fn goToFirstPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling goToFirstPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    let first_page = paginator_service.first_page.clone();
    return paginator_service.goToPage(identifier_mapper, application_service.getTranslator(), &first_page);
}

#[tauri::command]
pub fn goToLastPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling goToLastPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let last_page = paginator_service.last_page.clone();
    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.goToPage(identifier_mapper, application_service.getTranslator(), &last_page);
}

#[tauri::command]
pub fn nextPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling nextPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.nextPage(identifier_mapper, application_service.getTranslator());
}

#[tauri::command]
pub fn previousPage(paginator_service: State<PaginatorServiceState>, 
        application_service: State<ApplicationServiceState>) -> Result<(Vec<RawPattern>, u32, u32), GenericError> {

    println!("Calling previousPage...");

    let mut paginator_service = GenericError::from(paginator_service.0.lock(), "Could not lock paginator service", file!(), &line!())?;
    let application_service = GenericError::from(application_service.0.lock(), "Could not lock application service", file!(), &line!())?;

    let identifier_mapper = application_service.getIdentifierMapper()?;

    return paginator_service.previousPage(identifier_mapper, application_service.getTranslator());
}