use roxmltree::Node;

use super::node_parser::parse_node;
use super::types::Group;
use super::utils::get_documentation;
use super::xsd_elements::ElementType;
use super::{types::RsEntity, xsd_elements::XsdNode};

const AVAILABLE_CONTENT_TYPES: [ElementType; 3] =
    [ElementType::All, ElementType::Choice, ElementType::Sequence];

fn parse_global(group: &Node) -> RsEntity {
    let name = group.attr_name().unwrap();

    if let Some(content) = group
        .children()
        .filter(|child| child.is_element() && AVAILABLE_CONTENT_TYPES.contains(&child.xsd_type()))
        .last()
    {
        let entity = parse_node(&content, group);
        return RsEntity::Group(Group {
            comment: get_documentation(group),
            subtypes: vec![entity],
            name: name.to_string(),
            reference: None,
        });
    }

    RsEntity::Group(Group {
        comment: get_documentation(group),
        subtypes: vec![],
        name: name.to_string(),
        reference: None,
    })
}

fn parse_reference(group: &Node) -> RsEntity {
    let ref_name = group
        .attr_ref()
        .expect(&format!("Non-global group must be references."));
    RsEntity::Group(Group {
        comment: None,
        subtypes: vec![],
        name: ref_name.to_string(),
        reference: Some(ref_name.to_string()),
    })
}

pub fn parse_group(group: &Node, parent: &Node) -> RsEntity {
    match parent.xsd_type() {
        ElementType::Choice | ElementType::Sequence | ElementType::ComplexType => {
            parse_reference(group)
        }
        ElementType::Schema => parse_global(group),
        _ => panic!(""),
    }
}
