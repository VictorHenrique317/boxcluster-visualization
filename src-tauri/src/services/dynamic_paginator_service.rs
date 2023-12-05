#![allow(non_snake_case)]

use crate::{model::identifier_mapper::IdentifierMapper, database::pattern::Pattern, common::generic_error::GenericError};

pub struct DynamicPaginatorService{
    current_page: u32,
    page_size: u32,
  
    pub first_page: u32,
    pub last_page: u32,
}

impl<'a> Default for DynamicPaginatorService{
    fn default() -> Self { 
        return DynamicPaginatorService {
            current_page: 0,
            page_size: 1, 
            first_page: 0, 
            last_page: 0 };
    }
}

impl DynamicPaginatorService{
    pub fn getSoundingPattern(&self, identifier_mapper: &IdentifierMapper) -> Result<Pattern, GenericError>{
        return Ok(identifier_mapper.getRepresentation(&1)?.asPattern()?.clone()); // ID's start at 1
    }

    pub fn refreshPageSize(&mut self, identifier_mapper: &IdentifierMapper, page_size: u32) -> Result<(Vec<Pattern>, u32, u32), GenericError>{
        self.page_size = page_size;
        self.refreshLastPage(identifier_mapper);

        let first_page = self.first_page.clone();
        return self.goToPage(identifier_mapper, &first_page);
    }

    fn refreshLastPage(&mut self, identifier_mapper: &IdentifierMapper){
        self.last_page = (identifier_mapper.length() as f64 / self.page_size as f64).ceil() as u32 - 1;
    }

    pub fn goToPage(&mut self, identifier_mapper: &IdentifierMapper, page_index: &u32) -> Result<(Vec<Pattern>, u32, u32), GenericError>{
        if *page_index > self.last_page {
            return self.goToPage(identifier_mapper, &self.last_page.clone());
        }

        if *page_index < self.first_page {
            return self.goToPage(identifier_mapper, &self.first_page.clone());
        }

        let mut page_patterns: Vec<Pattern> = Vec::new();
        self.current_page = *page_index;

        let first_index = self.current_page * self.page_size;
        let last_index = first_index + self.page_size - 1;
        let last_pattern_index = identifier_mapper.length() - 1;

        for i in first_index..last_index + 1{
            if i > last_pattern_index {
                break;
            }
            page_patterns.push(identifier_mapper.getRepresentation(&(i + 1))?.asPattern()?.clone());
        }

        return Ok((page_patterns, self.current_page.clone(), self.last_page.clone()));
    }
    
    pub fn nextPage(&mut self, identifier_mapper: &IdentifierMapper) -> Result<(Vec<Pattern>, u32, u32), GenericError>{
        return self.goToPage(identifier_mapper, &(self.current_page + 1));
    }

    pub fn previousPage(&mut self, identifier_mapper: &IdentifierMapper) -> Result<(Vec<Pattern>, u32, u32), GenericError>{
        if self.current_page == self.first_page { // Prevents u32 overflow when trying to go to page -1
            return self.goToPage(identifier_mapper, &(self.first_page).clone());
        }
        return self.goToPage(identifier_mapper, &(self.current_page - 1).clone());
    }
}