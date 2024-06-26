use crate::{database::{dag_node::DagNode, datapoint::DataPoint, pattern::Pattern, raw_pattern::RawPattern}, common::generic_error::GenericError};

use super::io::translator::Translator;


pub struct IdentifierRepresentation {
    pattern_representation: Option<Pattern>,
    dag_node_representation: Option<DagNode>,
    data_point_representation: Option<DataPoint>,
}

impl IdentifierRepresentation {
    pub fn new(pattern_representation: Pattern) -> IdentifierRepresentation {
        return IdentifierRepresentation { 
            pattern_representation: Some(pattern_representation), 
            dag_node_representation: None, 
            data_point_representation: None 
        };
    }

    pub fn insertDagNodeRepresentation(&mut self, dag_node_representation: DagNode){
        self.dag_node_representation = Some(dag_node_representation);
    }

    pub fn insertDataPointRepresentation(&mut self, data_point_representation: DataPoint){
        self.data_point_representation = Some(data_point_representation);
    }

    pub fn removeDagNodeRepresentation(&mut self){
        self.dag_node_representation = None;
    }

    pub fn removeDatapointRepresentation(&mut self){
        self.data_point_representation = None;
    }

    pub fn asPattern(&self) -> Result<&Pattern, GenericError> {
        return self.pattern_representation.as_ref()
            .ok_or(GenericError::new("Could not get pattern representation", file!(), &line!()));
    }

    pub fn asRawPattern(&self, translator: &Translator) -> Result<RawPattern, GenericError> {
        let pattern = self.pattern_representation.as_ref()
            .ok_or(GenericError::new("Could not get pattern representation", file!(), &line!()))?;

        let raw_dims_values = translator.untranslateLineDims(&pattern.unordered_dims_values)?;
        let raw_dims_values: Vec<Vec<String>> = raw_dims_values.iter()
            .map(|raw_dim_values| raw_dim_values.split(",").map(|s| s.to_string()).collect())
            .collect();

        let raw_pattern = RawPattern::new(&pattern.identifier, &raw_dims_values, &pattern.density, &pattern.size);

        return Ok(raw_pattern);
    }

    pub fn asDagNode(&self) -> Result<&DagNode, GenericError> {
        return self.dag_node_representation.as_ref()
            .ok_or(GenericError::new("Could not get dag node representation", file!(), &line!()));
    }

    pub fn asDataPoint(&self) -> Result<&DataPoint, GenericError> {
        return self.data_point_representation.as_ref()
            .ok_or(GenericError::new("Could not get data point representation", file!(), &line!()));
    }
}