#![allow(non_snake_case)]

use crate::{subtensor::{pattern::Pattern}, common::identifier_mapper::IdentifierMapper};
pub struct DynamicPaginator<'a>{
    identifier_mapper: Option<&'a IdentifierMapper>,

    current_page: u32,
    page_size: u32,
  
    pub first_page: u32,
    pub last_page: u32,
}

impl<'a> Default for DynamicPaginator<'a>{
    fn default() -> Self { 
        return DynamicPaginator {
            identifier_mapper: None,
            current_page: 0,
            page_size: 1, 
            first_page: 0, 
            last_page: 0 };
    }
}

impl DynamicPaginator<'_>{
    pub fn getSoundingPattern(&self) -> Pattern{
        return self.identifier_mapper.unwrap().getRepresentation(&1).asPattern().clone(); // ID's start at 1
    }

    pub fn refreshPageSize(&mut self, page_size: u32) -> (Vec<Pattern>, u32, u32){
        self.page_size = page_size;
        self.refreshLastPage();

        let first_page = self.first_page.clone();
        return self.goToPage(&first_page);
    }

    fn refreshLastPage(&mut self){
        self.last_page = (self.identifier_mapper.unwrap().length() as f64 / self.page_size as f64).ceil() as u32 - 1;
    }

    pub fn goToPage(&mut self, page_index: &u32) -> (Vec<Pattern>, u32, u32){
        if *page_index > self.last_page {
            return self.goToPage(&self.last_page.clone());
        }

        if *page_index < self.first_page {
            return self.goToPage(&self.first_page.clone());
        }

        let mut page_patterns: Vec<Pattern> = Vec::new();
        self.current_page = *page_index;

        let first_index = self.current_page * self.page_size;
        let last_index = first_index + self.page_size - 1;
        let last_pattern_index = self.identifier_mapper.unwrap().length() - 1;

        for i in first_index..last_index + 1{
            if i > last_pattern_index {
                break;
            }
            page_patterns.push(self.identifier_mapper.unwrap().getRepresentation(&(i + 1)).asPattern().clone());
        }

        return (page_patterns, self.current_page.clone(), self.last_page.clone());
    }
    
    pub fn nextPage(&mut self) -> (Vec<Pattern>, u32, u32){
        return self.goToPage(&(self.current_page + 1));
    }

    pub fn previousPage(&mut self) -> (Vec<Pattern>, u32, u32){
        if self.current_page == self.first_page { // Prevents u32 overflow when trying to go to page -1
            return self.goToPage(&(self.first_page).clone());
        }
        return self.goToPage(&(self.current_page - 1).clone());
    }
}