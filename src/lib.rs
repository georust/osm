mod test;

use std::collections::HashMap;

pub struct ElementInfo {
    pub id: i64,
    pub user: String,
    pub uid: i32,
    pub timestamp: String,
    pub visible: bool,
    pub version: i32,
    pub changeset: i64
}

pub struct Node {
    pub element_info: ElementInfo,
    pub tags: HashMap<String, String>,
    pub lat: f64,
    pub lon: f64
}

pub struct Way {
    pub element_info: ElementInfo,
    pub tags: HashMap<String, String>,
    pub nodes: Vec<i64>
}

pub enum RelationMemberType {
    Node,
    Way,
    Relation
}

pub struct RelationMember {
    pub member_type: RelationMemberType,
    pub reference: i64,
    pub role: String
}

pub struct Relation {
    pub element_info: ElementInfo,
    pub tags: HashMap<String, String>,
}