use std::collections::HashMap;

/// A structure containing all the common attributes
/// of the basic OSM data types
/// 
/// See: [OSM wiki - Common attributes](https://wiki.openstreetmap.org/wiki/Elements#Common_attributes)
#[derive(Debug)]
pub struct ElementInfo {
    pub id: i64,
    pub user: Option<String>,
    pub uid: Option<i32>,
    pub timestamp: String,
    pub visible: bool,
    pub version: i32,
    pub changeset: Option<i64>
}

/// A node is a single Point holding `lat` and `lon` coordinates (WGS84 reference). 
/// They are used to define standalone point features. They are also used for modelling 
/// a Way or they can be a member of a relation.
/// 
/// Latitude and longitude are represented as 32-bit integers. Each coordinate is 
/// multiplied by 1E7 and rounded.
/// 
/// See: [OSM wiki - Node](https://wiki.openstreetmap.org/wiki/Node)
#[derive(Debug)]
pub struct Node {
    pub element_info: ElementInfo,
    pub tags: HashMap<String, String>,
    pub lat: i32,
    pub lon: i32
}

/// A way is a Polyline that is defined by a ordered list of Nodes. If a Way share the same
/// starting and end point it is considered as a closed Way. Most closed ways are considered 
/// to be areas even without an area=yes tag. Only if the way have the tags highway=* or 
/// barrier=* they are not considered as an area.
/// 
/// See: [OSM wiki - Way](https://wiki.openstreetmap.org/wiki/Way)
pub struct Way {
    pub element_info: ElementInfo,
    pub tags: HashMap<String, String>,
    pub nodes: Vec<i64>,
}

pub enum RelationMemberType {
    Node,
    Way,
    Relation
}

/// Each Relation consists of multiple members. Each member holds which `member_type` (Node, 
/// Way, Relation) it is, the `reference` (id) of it and what `role` it has inside the 
/// relation.
pub struct RelationMember {
    pub member_type: RelationMemberType,
    pub reference: i64,
    pub role: String,
}

/// A Relation is a multi-purpose data structure that is used to define relationship between
/// multiple other Elements, such as nodes, ways or even other relations. 
/// 
/// See: [OSM wiki - Relation](https://wiki.openstreetmap.org/wiki/Relation)
pub struct Relation {
    pub element_info: ElementInfo,
    pub tags: HashMap<String, String>,
    pub members: Vec<RelationMember>,
}