use crate::{model::io::{translator::Translator, tensor_reader::TensorReader, pattern_reader::PatternReader}, database::{tensor::Tensor, pattern::Pattern}, common::generic_error::GenericError};

pub struct IoService {
    tensor_path: String,
    patterns_path: String,
    translator: Translator,
}

impl Default for IoService{
    fn default() -> Self {
        return IoService{
            tensor_path: String::from(""),
            patterns_path: String::from(""),
            translator: Translator::default(),
        };
    }
}

impl IoService {
    pub fn new(tensor_path: &String, patterns_path: &String) -> Result<IoService, GenericError> {
        let translator = Translator::new(&tensor_path)?;

        return Ok(
            IoService {
                tensor_path: tensor_path.to_owned(),
                patterns_path: patterns_path.to_owned(),
                translator: translator,
            }
        );
    }

    pub fn setPatternsPath(&mut self, patterns_path: &String) {
        self.patterns_path = patterns_path.to_owned();
    }

    pub fn readTensor(&self) -> Result<Tensor, GenericError> {
        println!("Reading tensor ...");
        let tensor_reader = TensorReader::new(
            &self.tensor_path,
            &self.translator);
        return tensor_reader.read();
    }

    pub fn readPatterns(&self) -> Result<Vec<Pattern>, GenericError> {
        println!("Reading patterns ...");
        let pattern_reader = PatternReader::new(
                &self.patterns_path,
                &self.translator)?;

        return pattern_reader.read();
    }

    pub fn getTranslator(&self) -> &Translator {
        return &self.translator;
    }
}