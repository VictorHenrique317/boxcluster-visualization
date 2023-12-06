use crate::{database::{pattern::Pattern, dag_node::DagNode, datapoint::DataPoint}, common::generic_error::GenericError};


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

    pub fn asPattern(&self) -> Result<&Pattern, GenericError> {
        return self.pattern_representation.as_ref()
            .ok_or(GenericError::new("Could not get pattern representation", file!(), &line!()));
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