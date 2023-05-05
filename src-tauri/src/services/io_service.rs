use crate::{model::io::{translator::Translator, tensor_reader::TensorReader, pattern_reader::PatternReader}, database::{tensor::Tensor, pattern::Pattern}};

pub struct IoService {
    tensor_path: String,
    patterns_path: String,
    translator: Translator,
}

impl IoService {
    pub fn new(tensor_path: &String, patterns_path: &String) -> IoService {
        let translator = Translator::new(&tensor_path);

        IoService {
            tensor_path: tensor_path.to_owned(),
            patterns_path: patterns_path.to_owned(),
            translator: translator,
        }
    }

    pub fn setPatternsPath(&mut self, patterns_path: &String) {
        self.patterns_path = patterns_path.to_owned();
    }

    pub fn readTensor(&self) -> Tensor {
        let tensor_reader = TensorReader::new(
            &self.tensor_path,
            &self.translator);
        return tensor_reader.read();
    }

    pub fn readPatterns(&self) -> Vec<Pattern> {
        let pattern_reader = PatternReader::new(
                &self.patterns_path,
                &self.translator);
        return pattern_reader.read();
    }
}