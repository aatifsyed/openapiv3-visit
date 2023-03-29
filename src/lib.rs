use openapiv3::{
    AdditionalProperties, AnySchema, ArrayType, Contact, CookieStyle, Discriminator, Encoding,
    Example, ExternalDocumentation, Header, HeaderStyle, Info, IntegerFormat, IntegerType, License,
    MediaType, NumberFormat, NumberType, ObjectType, OpenAPI, Operation, Parameter, ParameterData,
    ParameterSchemaOrContent, PathItem, PathStyle, Paths, QueryStyle, ReferenceOr,
    ReferenceOr::Item, RequestBody, Schema, SchemaData, SchemaKind, Server, ServerVariable,
    StringFormat, StringType, Type, VariantOrUnknownOrEmpty,
};

pub trait VisitMut<'openapi> {}

pub fn visit_openapi_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut OpenAPI)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let OpenAPI {
        openapi: _,
        info,
        servers,
        paths,
        components,
        security,
        tags,
        external_docs,
        extensions,
    } = node;
    visit_info_mut(visitor, info);
    for node in servers {
        visit_server_mut(visitor, node);
    }
}

pub fn visit_paths_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Paths)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let Paths {
        paths,
        extensions: _,
    } = node;
    for (_, node) in paths {
        visit_reference_or_path_item_mut(visitor, node);
    }
}

pub fn visit_reference_or_path_item_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<PathItem>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_path_item_mut(visitor, node);
    }
}

pub fn visit_path_item_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut PathItem)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let PathItem {
        summary: _,
        description: _,
        get,
        put,
        post,
        delete,
        options,
        head,
        patch,
        trace,
        servers,
        parameters,
        extensions: _,
    } = node;
}

pub fn visit_operation_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut Operation,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let Operation {
        tags: _,
        summary: _,
        description: _,
        external_docs,
        operation_id: _,
        parameters,
        request_body,
        responses,
        deprecated: _,
        security: _,
        servers,
        extensions: _,
    } = node;
    if let Some(node) = external_docs.as_mut() {
        visit_external_documentation_mut(visitor, node);
    }
    for node in parameters {
        visit_reference_or_parameter_mut(visitor, node);
    }
    if let Some(node) = request_body.as_mut() {
        visit_reference_or_request_body_mut(visitor, node);
    }
}

pub fn visit_reference_or_request_body_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<RequestBody>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_request_body_mut(visitor, node);
    }
}

pub fn visit_request_body_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut RequestBody,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let RequestBody {
        description: _,
        content,
        required: _,
        extensions: _,
    } = node;
    for (_, node) in content {
        visit_media_type_mut(visitor, node);
    }
}

pub fn visit_reference_or_parameter_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<Parameter>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_parameter_mut(visitor, node);
    }
}
pub fn visit_parameter_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut Parameter,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        Parameter::Query {
            parameter_data,
            allow_reserved: _,
            style,
            allow_empty_value: _,
        } => {
            visit_parameter_data_mut(visitor, parameter_data);
            visit_query_style_mut(visitor, style);
        }
        Parameter::Header {
            parameter_data,
            style,
        } => {
            visit_parameter_data_mut(visitor, parameter_data);
            visit_header_style_mut(visitor, style);
        }
        Parameter::Path {
            parameter_data,
            style,
        } => {
            visit_parameter_data_mut(visitor, parameter_data);
            visit_path_style_mut(visitor, style);
        }
        Parameter::Cookie {
            parameter_data,
            style,
        } => {
            visit_parameter_data_mut(visitor, parameter_data);
            visit_cookie_style_mut(visitor, style);
        }
    }
}

pub fn visit_cookie_style_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut CookieStyle,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_path_style_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut PathStyle,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_parameter_data_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ParameterData,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let ParameterData {
        name: _,
        description: _,
        required: _,
        deprecated: _,
        format,
        example: _,
        examples,
        explode: _,
        extensions: _,
    } = node;
    visit_parameter_schema_or_content_mut(visitor, format);
    for (_, node) in examples {
        visit_reference_or_example_mut(visitor, node);
    }
}

pub fn visit_parameter_schema_or_content_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ParameterSchemaOrContent,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        ParameterSchemaOrContent::Schema(node) => visit_reference_or_schema_mut(visitor, node),
        ParameterSchemaOrContent::Content(node) => {
            for (_, node) in node {
                visit_media_type_mut(visitor, node);
            }
        }
    }
}

pub fn visit_media_type_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut MediaType,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let MediaType {
        schema,
        example: _,
        examples,
        encoding,
        extensions: _,
    } = node;
    if let Some(node) = schema.as_mut() {
        visit_reference_or_schema_mut(visitor, node);
    }
    for (_, node) in examples {
        visit_reference_or_example_mut(visitor, node);
    }
    for (_, node) in encoding {
        visit_encoding_mut(visitor, node);
    }
}

pub fn visit_encoding_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Encoding)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let Encoding {
        content_type: _,
        headers,
        style,
        explode: _,
        allow_reserved: _,
        extensions: _,
    } = node;
    for (_, node) in headers {
        visit_reference_or_header_mut(visitor, node);
    }
    if let Some(node) = style.as_mut() {
        visit_query_style_mut(visitor, node);
    }
}

pub fn visit_query_style_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut QueryStyle,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_reference_or_header_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<Header>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_header_mut(visitor, node);
    }
}

pub fn visit_header_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Header)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let Header {
        description: _,
        style,
        required: _,
        deprecated: _,
        format,
        example: _,
        examples,
        extensions: _,
    } = node;
    visit_header_style_mut(visitor, style);
    visit_parameter_schema_or_content_mut(visitor, format);
    for (_, node) in examples {
        visit_reference_or_example_mut(visitor, node);
    }
}

pub fn visit_header_style_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut HeaderStyle,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_reference_or_example_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<Example>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_example_mut(visitor, node);
    }
}

pub fn visit_example_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Example)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_reference_or_schema_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<Schema>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_schema_mut(visitor, node);
    }
}

pub fn visit_schema_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Schema)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let Schema {
        schema_data,
        schema_kind,
    } = node;
    visit_schema_data_mut(visitor, schema_data);
    visit_schema_kind_mut(visitor, schema_kind);
}

pub fn visit_schema_data_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut SchemaData,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let SchemaData {
        nullable: _,
        read_only: _,
        write_only: _,
        deprecated: _,
        external_docs,
        example: _,
        title: _,
        description: _,
        discriminator,
        default: _,
        extensions: _,
    } = node;
    if let Some(node) = external_docs.as_mut() {
        visit_external_documentation_mut(visitor, node);
    }
    if let Some(node) = discriminator.as_mut() {
        visit_discriminator_mut(visitor, node);
    }
}

pub fn visit_discriminator_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut Discriminator,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_schema_kind_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut SchemaKind,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        SchemaKind::Type(node) => visit_type_mut(visitor, node),
        SchemaKind::OneOf { one_of } => {
            for node in one_of {
                visit_reference_or_schema_mut(visitor, node);
            }
        }
        SchemaKind::AllOf { all_of } => {
            for node in all_of {
                visit_reference_or_schema_mut(visitor, node);
            }
        }
        SchemaKind::AnyOf { any_of } => {
            for node in any_of {
                visit_reference_or_schema_mut(visitor, node);
            }
        }
        SchemaKind::Not { not } => visit_reference_or_schema_mut(visitor, not),
        SchemaKind::Any(node) => visit_any_schema_mut(visitor, node),
    }
}

pub fn visit_any_schema_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut AnySchema,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let AnySchema {
        typ: _,
        pattern: _,
        multiple_of: _,
        exclusive_minimum: _,
        exclusive_maximum: _,
        minimum: _,
        maximum: _,
        properties,
        required: _,
        additional_properties,
        min_properties: _,
        max_properties: _,
        items,
        min_items: _,
        max_items: _,
        unique_items: _,
        enumeration: _,
        format: _,
        min_length: _,
        max_length: _,
        one_of,
        all_of,
        any_of,
        not,
    } = node;
    for (_, node) in properties {
        visit_reference_or_box_schema_mut(visitor, node);
    }
    if let Some(node) = additional_properties.as_mut() {
        visit_additional_properties_mut(visitor, node);
    }
    if let Some(node) = items.as_mut() {
        visit_reference_or_box_schema_mut(visitor, node);
    }
    for node in one_of {
        visit_reference_or_schema_mut(visitor, node);
    }
    for node in all_of {
        visit_reference_or_schema_mut(visitor, node);
    }
    for node in any_of {
        visit_reference_or_schema_mut(visitor, node);
    }
    if let Some(node) = not.as_mut() {
        visit_reference_or_schema_mut(visitor, node);
    }
}

pub fn visit_type_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Type)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        Type::String(node) => visit_string_type_mut(visitor, node),
        Type::Number(node) => visit_number_type_mut(visitor, node),
        Type::Integer(node) => visit_integer_type_mut(visitor, node),
        Type::Object(node) => visit_object_type_mut(visitor, node),
        Type::Array(node) => visit_array_type_mut(visitor, node),
        Type::Boolean {} => (),
    }
}

pub fn visit_array_type_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ArrayType,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let ArrayType {
        items,
        min_items: _,
        max_items: _,
        unique_items: _,
    } = node;
    if let Some(node) = items.as_mut() {
        visit_reference_or_box_schema_mut(visitor, node);
    }
}

pub fn visit_object_type_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ObjectType,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let ObjectType {
        properties,
        required: _,
        additional_properties,
        min_properties: _,
        max_properties: _,
    } = node;
    for (_, node) in properties {
        visit_reference_or_box_schema_mut(visitor, node);
    }
    if let Some(node) = additional_properties.as_mut() {
        visit_additional_properties_mut(visitor, node);
    }
}

pub fn visit_reference_or_box_schema_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<Box<Schema>>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_schema_mut(visitor, node);
    }
}

pub fn visit_additional_properties_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut AdditionalProperties,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        AdditionalProperties::Any(_) => (),
        AdditionalProperties::Schema(node) => visit_reference_or_schema_mut(visitor, node),
    }
}

pub fn visit_integer_type_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut IntegerType,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let IntegerType {
        format,
        multiple_of: _,
        exclusive_minimum: _,
        exclusive_maximum: _,
        minimum: _,
        maximum: _,
        enumeration: _,
    } = node;
    visit_variant_or_unknown_or_empty_integer_format_mut(visitor, format);
}

pub fn visit_variant_or_unknown_or_empty_integer_format_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut VariantOrUnknownOrEmpty<IntegerFormat>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visit_integer_format_mut(visitor, node),
        VariantOrUnknownOrEmpty::Unknown(_) => (),
        VariantOrUnknownOrEmpty::Empty => (),
    }
}

pub fn visit_integer_format_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut IntegerFormat,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_number_type_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut NumberType,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let NumberType {
        format,
        multiple_of: _,
        exclusive_minimum: _,
        exclusive_maximum: _,
        minimum: _,
        maximum: _,
        enumeration: _,
    } = node;
    visit_variant_or_unknown_or_empty_number_format_mut(visitor, format);
}

pub fn visit_variant_or_unknown_or_empty_number_format_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut VariantOrUnknownOrEmpty<NumberFormat>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visit_number_format_mut(visitor, node),
        VariantOrUnknownOrEmpty::Unknown(_) => (),
        VariantOrUnknownOrEmpty::Empty => (),
    }
}

pub fn visit_number_format_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut NumberFormat,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_string_type_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut StringType,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let StringType {
        format,
        pattern: _,
        enumeration: _,
        min_length: _,
        max_length: _,
    } = node;
    visit_variant_or_unknown_or_empty_string_format_mut(visitor, format);
}

pub fn visit_variant_or_unknown_or_empty_string_format_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut VariantOrUnknownOrEmpty<StringFormat>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visit_string_format_mut(visitor, node),
        VariantOrUnknownOrEmpty::Unknown(_) => (),
        VariantOrUnknownOrEmpty::Empty => (),
    }
}

pub fn visit_string_format_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut StringFormat,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_external_documentation_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ExternalDocumentation,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_server_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Server)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let Server {
        url: _,
        description: _,
        variables,
        extensions: _,
    } = node;
    if let Some(node) = variables.as_mut() {
        for (_, node) in node {
            visit_server_variable_mut(visitor, node);
        }
    }
}

pub fn visit_server_variable_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ServerVariable,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_info_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Info)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let Info {
        title: _,
        description: _,
        terms_of_service: _,
        contact,
        license,
        version: _,
        extensions: _,
    } = node;
    if let Some(node) = contact.as_mut() {
        visit_contact_mut(visitor, node)
    }
    if let Some(node) = license.as_mut() {
        visit_license_mut(visitor, node);
    }
}

pub fn visit_contact_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Contact)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_license_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut License)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}
