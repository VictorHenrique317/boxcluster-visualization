#![allow(non_snake_case)]
use std::error::Error;

use serde::de;

use crate::database::pattern::Pattern;
use super::{translator::Translator, reader::Reader};

pub struct PatternReader<'a> {
    pub file_path: String,
    pub translator: &'a Translator,
    file_has_densities: bool,
}

impl PatternReader<'_>{
    pub fn new<'a>(file_path: &String, translator: &'a Translator) -> Result<PatternReader<'a>, Box<dyn Error>> {
        let density = Reader::fileHasDensity(&file_path)?;

        let instance = PatternReader {
            file_path: file_path.clone(),
            translator: translator,
            file_has_densities: density,
        };

        return Ok(instance);
    }

    pub fn read<'a>(self) -> Vec<Pattern>{
        let mut patterns: Vec<Pattern> = Vec::new();        
        let lines: Vec<String> = Reader::readRawLines(&self.file_path);
        
        for (i, line) in lines.iter().enumerate() {
            let mut density: f64 = 1.0;
            let mut line_dims: Vec<String> = line.split(" ")
                .map(|s| s.to_owned())
                .collect();

            if self.file_has_densities{
                density = line_dims.pop().unwrap().parse::<f64>().unwrap();
            }

            patterns.push(Pattern::new(
                i as u32 + 1, 
                self.translator.translateLineDims(&line_dims),
                density
            ));
        }

        return patterns;
    }
}

