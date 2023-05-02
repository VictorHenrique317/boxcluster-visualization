use crate::{subtensor::pattern::Pattern, dag::dag_node::DagNode, metrics::datapoint::DataPoint};

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

    pub fn asPattern(&self) -> &Pattern {
        return self.pattern_representation.as_ref().unwrap();
    }

    pub fn asDagNode(&self) -> &DagNode {
        return self.dag_node_representation.as_ref().unwrap();
    }

    pub fn asDataPoint(&self) -> &DataPoint {
        return self.data_point_representation.as_ref().unwrap();
    }
}