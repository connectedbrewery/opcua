use std::result::Result;

use opcua_types::{
    *,
    status_code::StatusCode,
    service_types::*,
    node_ids::ObjectId,
};

use crate::{
    address_space::{
        AddressSpace,
        relative_path,
        types::*,
    },
    session::Session,
    services::Service,
    state::ServerState,
};

pub(crate) struct NodeManagementService;

impl Service for NodeManagementService {}

impl NodeManagementService {
    pub fn new() -> NodeManagementService {
        NodeManagementService {}
    }

    /// Implements the AddNodes service
    pub fn add_nodes(&self, server_state: &ServerState, session: &Session, address_space: &mut AddressSpace, request: &AddNodesRequest) -> Result<SupportedMessage, StatusCode> {
        if let Some(ref nodes_to_add) = request.nodes_to_add {
            if !nodes_to_add.is_empty() {
                if nodes_to_add.len() <= server_state.max_nodes_per_node_management() {
                    let results = nodes_to_add.iter().map(|node_to_add| {
                        let (status_code, added_node_id) = Self::add_node(session, address_space, node_to_add);
                        AddNodesResult {
                            status_code,
                            added_node_id,
                        }
                    }).collect();
                    let response = AddNodesResponse {
                        response_header: ResponseHeader::new_good(&request.request_header),
                        results: Some(results),
                        diagnostic_infos: None,
                    };
                    Ok(response.into())
                } else {
                    Ok(self.service_fault(&request.request_header, StatusCode::BadTooManyOperations))
                }
            } else {
                Ok(self.service_fault(&request.request_header, StatusCode::BadNothingToDo))
            }
        } else {
            Ok(self.service_fault(&request.request_header, StatusCode::BadNothingToDo))
        }
    }

    /// Implements the AddReferences service
    pub fn add_references(&self, server_state: &ServerState, session: &Session, address_space: &mut AddressSpace, request: &AddReferencesRequest) -> Result<SupportedMessage, StatusCode> {
        if let Some(ref references_to_add) = request.references_to_add {
            if !references_to_add.is_empty() {
                if references_to_add.len() <= server_state.max_nodes_per_node_management() {
                    let results = references_to_add.iter().map(|r| {
                        Self::add_reference(session, address_space, r)
                    }).collect();
                    Ok(AddReferencesResponse {
                        response_header: ResponseHeader::new_good(&request.request_header),
                        results: Some(results),
                        diagnostic_infos: None,
                    }.into())
                } else {
                    Ok(self.service_fault(&request.request_header, StatusCode::BadTooManyOperations))
                }
            } else {
                Ok(self.service_fault(&request.request_header, StatusCode::BadNothingToDo))
            }
        } else {
            Ok(self.service_fault(&request.request_header, StatusCode::BadNothingToDo))
        }
    }

    /// Implements the DeleteNodes service
    pub fn delete_nodes(&self, server_state: &ServerState, session: &Session, address_space: &mut AddressSpace, request: &DeleteNodesRequest) -> Result<SupportedMessage, StatusCode> {
        if let Some(ref nodes_to_delete) = request.nodes_to_delete {
            if !nodes_to_delete.is_empty() {
                if nodes_to_delete.len() <= server_state.max_nodes_per_node_management() {
                    let results = nodes_to_delete.iter().map(|node_to_delete| {
                        Self::delete_node(session, address_space, node_to_delete)
                    }).collect();
                    let response = DeleteNodesResponse {
                        response_header: ResponseHeader::new_good(&request.request_header),
                        results: Some(results),
                        diagnostic_infos: None,
                    };
                    Ok(response.into())
                } else {
                    Ok(self.service_fault(&request.request_header, StatusCode::BadTooManyOperations))
                }
            } else {
                Ok(self.service_fault(&request.request_header, StatusCode::BadNothingToDo))
            }
        } else {
            Ok(self.service_fault(&request.request_header, StatusCode::BadNothingToDo))
        }
    }

    /// Implements the DeleteReferences service
    pub fn delete_references(&self, server_state: &ServerState, session: &Session, address_space: &mut AddressSpace, request: &DeleteReferencesRequest) -> Result<SupportedMessage, StatusCode> {
        if let Some(ref references_to_delete) = request.references_to_delete {
            if !references_to_delete.is_empty() {
                if references_to_delete.len() <= server_state.max_nodes_per_node_management() {
                    let results = references_to_delete.iter().map(|r| {
                        Self::delete_reference(session, address_space, r)
                    }).collect();
                    Ok(DeleteReferencesResponse {
                        response_header: ResponseHeader::new_good(&request.request_header),
                        results: Some(results),
                        diagnostic_infos: None,
                    }.into())
                } else {
                    Ok(self.service_fault(&request.request_header, StatusCode::BadTooManyOperations))
                }
            } else {
                Ok(self.service_fault(&request.request_header, StatusCode::BadNothingToDo))
            }
        } else {
            Ok(self.service_fault(&request.request_header, StatusCode::BadNothingToDo))
        }
    }

    fn create_node(node_id: &NodeId, node_class: NodeClass, browse_name: QualifiedName, node_attributes: &ExtensionObject) -> Result<NodeType, StatusCode> {
        let object_id = node_attributes.node_id.as_object_id().map_err(|_| StatusCode::BadNodeAttributesInvalid)?;
        // Note we are expecting the node_class and the object id for the attributes to be for the same
        // thing. If they are different, it is an error.

        let decoding_limits = DecodingLimits::default();
        match object_id {
            ObjectId::ObjectAttributes_Encoding_DefaultBinary => {
                if node_class == NodeClass::Object {
                    let attributes = node_attributes.decode_inner::<ObjectAttributes>(&decoding_limits)?;
                    Object::from_attributes(node_id, browse_name, attributes).map(|n| n.into())
                } else {
                    error!("node class and object node attributes are not compatible");
                    Err(())
                }
            }
            ObjectId::VariableAttributes_Encoding_DefaultBinary => {
                if node_class == NodeClass::Variable {
                    let attributes = node_attributes.decode_inner::<VariableAttributes>(&decoding_limits)?;
                    Variable::from_attributes(node_id, browse_name, attributes).map(|n| n.into())
                } else {
                    error!("node class and variable node attributes are not compatible");
                    Err(())
                }
            }
            ObjectId::MethodAttributes_Encoding_DefaultBinary => {
                if node_class == NodeClass::Method {
                    let attributes = node_attributes.decode_inner::<MethodAttributes>(&decoding_limits)?;
                    Method::from_attributes(node_id, browse_name, attributes).map(|n| n.into())
                } else {
                    error!("node class and method node attributes are not compatible");
                    Err(())
                }
            }
            ObjectId::ObjectTypeAttributes_Encoding_DefaultBinary => {
                if node_class == NodeClass::ObjectType {
                    let attributes = node_attributes.decode_inner::<ObjectTypeAttributes>(&decoding_limits)?;
                    ObjectType::from_attributes(node_id, browse_name, attributes).map(|n| n.into())
                } else {
                    error!("node class and object type node attributes are not compatible");
                    Err(())
                }
            }
            ObjectId::VariableTypeAttributes_Encoding_DefaultBinary => {
                if node_class == NodeClass::VariableType {
                    let attributes = node_attributes.decode_inner::<VariableTypeAttributes>(&decoding_limits)?;
                    VariableType::from_attributes(node_id, browse_name, attributes).map(|n| n.into())
                } else {
                    error!("node class and variable type node attributes are not compatible");
                    Err(())
                }
            }
            ObjectId::ReferenceTypeAttributes_Encoding_DefaultBinary => {
                if node_class == NodeClass::ReferenceType {
                    let attributes = node_attributes.decode_inner::<ReferenceTypeAttributes>(&decoding_limits)?;
                    ReferenceType::from_attributes(node_id, browse_name, attributes).map(|n| n.into())
                } else {
                    error!("node class and reference type node attributes are not compatible");
                    Err(())
                }
            }
            ObjectId::DataTypeAttributes_Encoding_DefaultBinary => {
                if node_class == NodeClass::DataType {
                    let attributes = node_attributes.decode_inner::<DataTypeAttributes>(&decoding_limits)?;
                    DataType::from_attributes(node_id, browse_name, attributes).map(|n| n.into())
                } else {
                    error!("node class and data type node attributes are not compatible");
                    Err(())
                }
            }
            ObjectId::ViewAttributes_Encoding_DefaultBinary => {
                if node_class == NodeClass::View {
                    let attributes = node_attributes.decode_inner::<ViewAttributes>(&decoding_limits)?;
                    View::from_attributes(node_id, browse_name, attributes).map(|n| n.into())
                } else {
                    error!("node class and view node attributes are not compatible");
                    Err(())
                }
            }
            _ => {
                error!("create_node was called with an object id which does not match a supported type");
                Err(())
            }
        }.map_err(|_| StatusCode::BadNodeAttributesInvalid)
    }

    fn add_node(session: &Session, address_space: &mut AddressSpace, item: &AddNodesItem) -> (StatusCode, NodeId) {
        if !session.can_modify_address_space() {
            // No permission to modify address space
            return (StatusCode::BadUserAccessDenied, NodeId::null());
        }

        let requested_new_node_id = &item.requested_new_node_id;
        if requested_new_node_id.server_index != 0 {
            // Server index is supposed to be 0
            error!("node cannot be created because server index is not 0");
            return (StatusCode::BadNodeIdRejected, NodeId::null());
        }

        if item.node_class == NodeClass::Unspecified {
            error!("node cannot be created because node class is unspecified");
            return (StatusCode::BadNodeClassInvalid, NodeId::null());
        }

        if !requested_new_node_id.is_null() {
            if address_space.node_exists(&requested_new_node_id.node_id) {
                // If a node id is supplied, it should not already exist
                error!("node cannot be created because node id already exists");
                return (StatusCode::BadNodeIdExists, NodeId::null());
            }
        }

        // Test for invalid browse name
        if item.browse_name.is_null() || item.browse_name.name.as_ref().is_empty() {
            error!("node cannot be created because the browse name is invalid");
            return (StatusCode::BadBrowseNameInvalid, NodeId::null());
        }

        // Test duplicate browse name to same parent
        let browse_name = if item.browse_name.namespace_index != 0 {
            format!("{}:{}", item.browse_name.namespace_index, item.browse_name.name.as_ref())
        } else {
            format!("/{}", item.browse_name.name.as_ref())
        };
        let relative_path = RelativePath::from_str(&browse_name, &RelativePathElement::default_node_resolver).unwrap();
        if let Ok(nodes) = relative_path::find_nodes_relative_path(address_space, &item.parent_node_id.node_id, &relative_path) {
            if !nodes.is_empty() {
                error!("node cannot be created because the browse name is a duplicate");
                return (StatusCode::BadBrowseNameDuplicated, NodeId::null());
            }
        }

        if let Ok(reference_type_id) = item.reference_type_id.as_reference_type_id() {
            // Node Id was either supplied or will be generated
            let new_node_id = if requested_new_node_id.is_null() {
                NodeId::next_numeric()
            } else {
                requested_new_node_id.node_id.clone()
            };

            // TODO test data model constraint
            // BadReferenceNotAllowed

            // Check the type definition is valid
            if !address_space.is_valid_type_definition(item.node_class, &item.type_definition.node_id) {
                // Type definition was either invalid or supplied when it should not have been supplied
                error!("node cannot be created because type definition is not valid");
                return (StatusCode::BadTypeDefinitionInvalid, NodeId::null());
            }

            // Check that the parent node exists
            if !item.parent_node_id.server_index == 0 || !address_space.node_exists(&item.parent_node_id.node_id) {
                error!("node cannot be created because parent node id is invalid or does not exist");
                return (StatusCode::BadParentNodeIdInvalid, NodeId::null());
            }

            // Create a node
            if let Ok(node) = Self::create_node(&new_node_id, item.node_class, item.browse_name.clone(), &item.node_attributes) {
                // Add the node to the address space
                address_space.insert(node, Some(&[
                    (&item.parent_node_id.node_id, reference_type_id, ReferenceDirection::Forward),
                ]));
                // Object / Variable types must add a reference to the type
                if item.node_class == NodeClass::Object || item.node_class == NodeClass::Variable {
                    address_space.set_node_type(&new_node_id, item.type_definition.node_id.clone());
                }
                (StatusCode::Good, new_node_id)
            } else {
                // Create node failed, so assume a problem with the node attributes
                error!("node cannot be created because attributes / not class are not valid");
                (StatusCode::BadNodeAttributesInvalid, NodeId::null())
            }
        } else {
            error!("node cannot be created because reference type is invalid");
            (StatusCode::BadReferenceTypeIdInvalid, NodeId::null())
        }
    }

    fn add_reference(session: &Session, address_space: &mut AddressSpace, item: &AddReferencesItem) -> StatusCode {
        if !session.can_modify_address_space() {
            // No permission to modify address space
            StatusCode::BadUserAccessDenied
        } else if !item.target_server_uri.is_null() {
            StatusCode::BadServerUriInvalid
        } else if item.target_node_id.server_index != 0 {
            StatusCode::BadReferenceLocalOnly
        } else if !address_space.node_exists(&item.source_node_id) {
            StatusCode::BadSourceNodeIdInvalid
        } else if !address_space.node_exists(&item.target_node_id.node_id) {
            StatusCode::BadTargetNodeIdInvalid
        } else if item.target_node_class == NodeClass::Unspecified {
            StatusCode::BadNodeClassInvalid
        } else {
            if let Some(node_type) = address_space.find_node(&item.target_node_id.node_id) {
                // If the target node exists the class can be compared to the one supplied
                let valid_node_class = match item.target_node_class {
                    NodeClass::Object => {
                        if let NodeType::Object(_) = *node_type { true } else { false }
                    }
                    NodeClass::Variable => {
                        if let NodeType::Variable(_) = *node_type { true } else { false }
                    }
                    NodeClass::Method => {
                        if let NodeType::Method(_) = *node_type { true } else { false }
                    }
                    NodeClass::ObjectType => {
                        if let NodeType::ObjectType(_) = *node_type { true } else { false }
                    }
                    NodeClass::VariableType => {
                        if let NodeType::VariableType(_) = *node_type { true } else { false }
                    }
                    NodeClass::ReferenceType => {
                        if let NodeType::ReferenceType(_) = *node_type { true } else { false }
                    }
                    NodeClass::DataType => {
                        if let NodeType::DataType(_) = *node_type { true } else { false }
                    }
                    NodeClass::View => {
                        if let NodeType::View(_) = *node_type { true } else { false }
                    }
                    _ => false
                };
                if !valid_node_class {
                    return StatusCode::BadNodeClassInvalid;
                }
            }


            if let Ok(reference_type_id) = item.reference_type_id.as_reference_type_id() {
                if !address_space.has_reference(&item.source_node_id, &item.target_node_id.node_id, reference_type_id) {
                    // TODO test data model constraint
                    // BadReferenceNotAllowed
                    if item.is_forward {
                        address_space.insert_reference(&item.source_node_id, &item.target_node_id.node_id, reference_type_id);
                    } else {
                        address_space.insert_reference(&item.target_node_id.node_id, &item.source_node_id, reference_type_id);
                    }
                    StatusCode::Good
                } else {
                    error!("reference cannot be added because reference is a duplicate");
                    StatusCode::BadDuplicateReferenceNotAllowed
                }
            } else {
                error!("reference cannot be added because reference type id is invalid");
                StatusCode::BadReferenceTypeIdInvalid
            }
        }
    }

    fn delete_node(session: &Session, address_space: &mut AddressSpace, item: &DeleteNodesItem) -> StatusCode {
        if !session.can_modify_address_space() {
            // No permission to modify address space
            StatusCode::BadUserAccessDenied
        } else if address_space.delete_node(&item.node_id, item.delete_target_references) {
            StatusCode::Good
        } else {
            error!("node cannot be deleted");
            StatusCode::BadNodeIdUnknown
        }
    }

    fn delete_reference(session: &Session, address_space: &mut AddressSpace, item: &DeleteReferencesItem) -> StatusCode {
        let node_id = &item.source_node_id;
        let target_node_id = &item.target_node_id.node_id;

        if !session.can_modify_address_space() {
            // No permission to modify address space
            StatusCode::BadUserAccessDenied
        } else if item.target_node_id.server_index != 0 {
            error!("reference cannot be added because only local references are supported");
            StatusCode::BadReferenceLocalOnly
        } else if node_id.is_null() || !address_space.node_exists(&node_id) {
            error!("reference cannot be added because source node id is invalid");
            StatusCode::BadSourceNodeIdInvalid
        } else if target_node_id.is_null() || !address_space.node_exists(&target_node_id) {
            error!("reference cannot be added because target node id is invalid");
            StatusCode::BadTargetNodeIdInvalid
        } else {
            if let Ok(reference_type_id) = item.reference_type_id.as_reference_type_id() {
                if item.delete_bidirectional {
                    address_space.delete_reference(&node_id, &target_node_id, reference_type_id);
                    address_space.delete_reference(&target_node_id, &node_id, reference_type_id);
                } else if item.is_forward {
                    address_space.delete_reference(&node_id, &target_node_id, reference_type_id);
                } else {
                    address_space.delete_reference(&target_node_id, &node_id, reference_type_id);
                }
                StatusCode::Good
            } else {
                error!("reference cannot be added because reference type id is invalid");
                StatusCode::BadReferenceTypeIdInvalid
            }
        }
    }
}
