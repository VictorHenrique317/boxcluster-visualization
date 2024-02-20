#![allow(non_snake_case)]

use crate::{database::pattern::Pattern, common::generic_error::GenericError};
use super::{translator::Translator, reader::Reader};

pub struct PatternReader<'a> {
    pub file_path: String,
    pub translator: &'a Translator,
    file_has_densities: bool,
}

impl PatternReader<'_>{
    pub fn new<'a>(file_path: &String, translator: &'a Translator) -> Result<PatternReader<'a>, GenericError> {
        let file_has_densities = Reader::fileHasDensity(&file_path)?;

        let instance = PatternReader {
            file_path: file_path.clone(),
            translator: translator,
            file_has_densities: file_has_densities,
        };

        return Ok(instance);
    }

    pub fn read<'a>(self) -> Result<Vec<Pattern>, GenericError>{
        let mut patterns: Vec<Pattern> = Vec::new();        
        let lines: Vec<String> = Reader::readRawLines(&self.file_path)?;
        
        for (i, line) in lines.iter().enumerate() {
            let mut density: f64 = 1.0;
            let mut line_dims: Vec<String> = Reader::preProcessLine(&line);

            if self.file_has_densities{
                density = line_dims.pop()
                    .ok_or(GenericError::new("Could not get density", file!(), &line!()))?
                    .parse::<f64>()
                    .map_err(|_| GenericError::new("Could not parse density to f64", file!(), &line!()))?
            }

            patterns.push(Pattern::new(
                i as u32 + 1, 
                self.translator.translateLineDims(&line_dims)?,
                density
            ));
        }

        return Ok(patterns);
    }
}

