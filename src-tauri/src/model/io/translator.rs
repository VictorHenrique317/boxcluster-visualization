
#![allow(non_snake_case)]
use std::{collections::HashMap};
use super::reader::Reader;

#[derive(Default)]
pub struct Translator{
    translator: Vec<HashMap<String, u32>>,
    reversed_translator: Vec<HashMap<u32, String>>,
}

// Translation source path HAS to be a tensor

impl Translator {    

    pub fn new(translation_source_path: &String) -> Translator{
        println!("Creating translator...");
        let translator = Translator::createTranslator(&translation_source_path);
        let reversed_translator = Translator::reverseTranslator(&translator);

        return Translator { 
            translator: translator,
            reversed_translator: reversed_translator,
        }
    }
    
    fn createEmptyTranslator(sample_line:&String) -> Vec<HashMap<String, u32>>{
        let mut empty_translator: Vec<HashMap<String, u32>> = Vec::new();
    
        let sample_line: Vec<String> = sample_line.split(" ").map(|i| i.to_owned()).collect();

        let dimensions_nb = sample_line.len() - 1;
        
        for _ in 0..dimensions_nb{
            empty_translator.push(HashMap::new());
        }
    
        return empty_translator;
    }

    fn createTranslator(translation_source_path: &String) -> Vec<HashMap<String, u32>> {
        let lines = Reader::readRawLines(&translation_source_path);
        let mut translator: Vec<HashMap<String, u32>> = Translator::createEmptyTranslator(lines.get(0).unwrap());
        // let file_has_density: bool = AmbientReader::fileHasDensity(&lines);

        for line in lines{
            let dimensions: Vec<String> = line.split(" ")
                .map(|i| i.to_owned())
                .collect();
            
            for (i, dimension) in dimensions.iter().enumerate(){
                if i == dimensions.len() - 1 { // On the last 'dimension' of this line
                    break;
                }
                
                let dim_translator: &mut HashMap<String, u32> = translator.get_mut(i).unwrap();
    
                let values: Vec<String> = dimension.split(",")
                    .map(|i| i.to_owned())
                    .collect();
    
                // dbg!(&values);
                
                for value in values{
                    let translated_value = dim_translator.get(&value);
    
                    if translated_value.is_none(){ // Key does not exist
                        dim_translator.insert(value, dim_translator.len() as u32); // Starts from 0
                    }
                }
            }
        }
    
        return translator;
    }

    fn reverseTranslator(translator: &Vec<HashMap<String, u32>>) -> Vec<HashMap<u32, String>> {
        let mut reversed_translator: Vec<HashMap<u32, String>> = Vec::new();
    
        for dim_translator in translator{
            let mut reversed_dim_translator: HashMap<u32, String> = HashMap::new();
    
            for (key, value) in dim_translator{
                reversed_dim_translator.insert(*value, key.to_owned());
            }
            reversed_translator.push(reversed_dim_translator);
        }
    
        return reversed_translator;
    }

    pub fn translateLineDims(&self, line_dims: &Vec<String>) -> Vec<Vec<usize>>{
        let mut translated_line: Vec<Vec<usize>> = Vec::new();
        // dbg!(&line_dims);
    
        for (i, dim) in line_dims.iter().enumerate(){
            // dbg!(&i);
            
            // dbg!(self.translator.len());
            
            let dim_translator = self.translator.get(i).unwrap();
            let values: Vec<String> = dim.split(",").map(|i| i.to_owned()).collect();
            let mut translated_dim: Vec<usize> = Vec::new();
    
            for value in values{
                let translated_value = dim_translator.get(&value).unwrap(); 
                let translated_value = usize::try_from(*translated_value).unwrap();
                translated_dim.push(translated_value);
            }
            
            translated_line.push(translated_dim);
        }
    
        return translated_line;
    }
    
    pub fn untranslateLineDims(&self, dims_values: &Vec<Vec<usize>>) -> Vec<String>{
        let mut original_dims: Vec<String> = Vec::new();
        for (i, dim) in dims_values.iter().enumerate(){
            let mut original_dim: Vec<String> = Vec::new();
    
            for value in dim{
                let value = *value as u32;
                let original_value =  self.reversed_translator.get(i)
                    .unwrap()
                    .get(&value)
                    .unwrap()
                    .to_owned();
    
                original_dim.push(original_value);
            }
    
            original_dims.push(original_dim.join(","));
        }

        return original_dims;
    }

    pub fn getSize(&self) -> Vec<usize>{
        let mut translator_size: Vec<usize> = Vec::new();

        for dim_translator in self.translator.iter(){
            translator_size.push(dim_translator.len());
        }

        return translator_size;
    }
}