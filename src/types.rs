extern crate chrono;
extern crate geo_types;

use self::chrono::{DateTime, Utc};
use self::geo_types::Point;
use std::convert::From;

/// A structure containing all the common attributes
/// of the basic OSM data types
///
/// See: [OSM wiki - Common attributes](https://wiki.openstreetmap.org/wiki/Elements#Common_attributes)
#[derive(Debug)]
pub struct ElementInfo {
    pub id: i64,
    pub user: Option<String>,
    pub uid: Option<i32>,
    pub timestamp: Option<DateTime<Utc>>,
    pub visible: Option<bool>,
    pub version: Option<i32>,
    pub changeset: Option<i64>,
    pub tags: Vec<String>,
}

impl ElementInfo {
    fn create(id: i64) -> ElementInfo {
        ElementInfo {
            id: id,
            user: Option::None,
            uid: Option::None,
            timestamp: Option::None,
            visible: Option::None,
            version: Option::None,
            changeset: Option::None,
            tags: vec![],
        }
    }
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
    pub lat: i32,
    pub lon: i32,
}

impl Node {
    /// Convert latitude or longitude from natural f64 to i32, the
    /// datatype to store the value more efficiently
    pub fn coord_ftoi(coord: f64) -> i32 {
        (coord * 1e7) as i32
    }

    /// Convert latitude or longitude from i32, the datatype to store
    /// the value more efficiently, to f64
    pub fn coord_itof(coord: i32) -> f64 {
        (coord as f64) / 1e7
    }
}

impl From<Node> for Point<f64> {
    fn from(other: Node) -> Point<f64> {
        Point::new(Node::coord_itof(other.lon), Node::coord_itof(other.lat))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        (self.element_info.id == other.element_info.id)
            && (self.lat == other.lat)
            && (self.lon == self.lon)
    }
}

/// A way is a Polyline that is defined by a ordered list of Nodes. If a Way share the same
/// starting and end point it is considered as a closed Way. Most closed ways are considered
/// to be areas even without an area=yes tag. Only if the way have the tags highway=* or
/// barrier=* they are not considered as an area.
///
/// See: [OSM wiki - Way](https://wiki.openstreetmap.org/wiki/Way)
pub struct Way {
    pub element_info: ElementInfo,
    pub nodes: Vec<i64>,
}

pub enum RelationMemberType {
    Node,
    Way,
    Relation,
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
    pub members: Vec<RelationMember>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_node_coordinate_conversion() {
        assert_eq!(Node::coord_itof(515098650), 51.509865);
        assert_eq!(Node::coord_itof(-1180920), -0.118092);
        assert_eq!(Node::coord_ftoi(40.730610), 407306100);
        assert_eq!(Node::coord_ftoi(-73.935242), -739352420);
    }

    #[test]
    fn test_node_equality() {
        let mut node_a = Node {
            element_info: ElementInfo::create(1),
            lat: Node::coord_ftoi(51.509865),
            lon: Node::coord_ftoi(-0.118092),
        };

        let mut node_b = Node {
            element_info: ElementInfo::create(1),
            lat: 515098650,
            lon: -1180920,
        };

        let mut node_c = Node {
            element_info: ElementInfo::create(2),
            lat: 515098650,
            lon: -1180920,
        };

        assert_eq!(node_a, node_b);
        assert_eq!(node_b, node_a);
        assert_ne!(node_a, node_c);
        assert_ne!(node_b, node_c);
    }

    #[test]
    fn test_node_to_point() {
        let mut node = Node {
            element_info: ElementInfo::create(1),
            lat: Node::coord_ftoi(51.509865),
            lon: Node::coord_ftoi(-0.118092),
        };

        let point = Point::from(node);

        assert_eq!(point.x(), -0.118092f64);
        assert_eq!(point.y(), 51.509865);
    }
}
