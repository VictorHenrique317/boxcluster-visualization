use tauri::State;
use crate::{common::generic_error::GenericError, controller::states::states::{ApplicationServiceState, PaginatorServiceState}, database::raw_pattern::RawPattern};

#[tauri::command]
pub fn getSoundingPattern(paginator_service: State<PaginatorServiceState>, application_service: State<ApplicationServiceState>) 
        -> Result<RawPattern, GenericError> {

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

    match paginator_service.getSoundingPattern(identifier_mapper, application_service.getTranslator()) {
        Ok(pattern) => return Ok(pattern),
        Err(error) => return Err(error),
    }
}