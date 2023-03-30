use openapiv3::{
    APIKeyLocation, AdditionalProperties, AnySchema, ArrayType, Callback, Components, Contact,
    CookieStyle, Discriminator, Encoding, Example, ExternalDocumentation, Header, HeaderStyle,
    Info, IntegerFormat, IntegerType, License, Link, LinkOperation, MediaType, NumberFormat,
    NumberType, OAuth2Flow, OAuth2Flows, ObjectType, OpenAPI, Operation, Parameter, ParameterData,
    ParameterSchemaOrContent, PathItem, PathStyle, Paths, QueryStyle, ReferenceOr,
    ReferenceOr::Item, RequestBody, Response, Responses, Schema, SchemaData, SchemaKind,
    SecurityScheme, Server, ServerVariable, StringFormat, StringType, Tag, Type,
    VariantOrUnknownOrEmpty,
};

pub trait VisitMut<'openapi> {
    fn visit_openapi_mut(&mut self, node: &'openapi mut OpenAPI) {
        visit_openapi_mut(self, node)
    }
    fn visit_tag_mut(&mut self, node: &'openapi mut Tag) {
        visit_tag_mut(self, node)
    }
    fn visit_components_mut(&mut self, node: &'openapi mut Components) {
        visit_components_mut(self, node)
    }
    fn visit_reference_or_callback_mut(&mut self, node: &'openapi mut ReferenceOr<Callback>) {
        visit_reference_or_callback_mut(self, node)
    }
    fn visit_callback_mut(&mut self, node: &'openapi mut Callback) {
        visit_callback_mut(self, node)
    }
    fn visit_reference_or_security_scheme_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<SecurityScheme>,
    ) {
        visit_reference_or_security_scheme_mut(self, node)
    }
    fn visit_security_scheme_mut(&mut self, node: &'openapi mut SecurityScheme) {
        visit_security_scheme_mut(self, node)
    }
    fn visit_oauth2_flows_mut(&mut self, node: &'openapi mut OAuth2Flows) {
        visit_oauth2_flows_mut(self, node)
    }
    fn visit_oauth2_flow_mut(&mut self, node: &'openapi mut OAuth2Flow) {
        visit_oauth2_flow_mut(self, node)
    }
    fn visit_api_key_location_mut(&mut self, node: &'openapi mut APIKeyLocation) {
        visit_api_key_location_mut(self, node)
    }
    fn visit_paths_mut(&mut self, node: &'openapi mut Paths) {
        visit_paths_mut(self, node)
    }
    fn visit_reference_or_path_item_mut(&mut self, node: &'openapi mut ReferenceOr<PathItem>) {
        visit_reference_or_path_item_mut(self, node)
    }
    fn visit_path_item_mut(&mut self, node: &'openapi mut PathItem) {
        visit_path_item_mut(self, node)
    }
    fn visit_operation_mut(&mut self, node: &'openapi mut Operation) {
        visit_operation_mut(self, node)
    }
    fn visit_responses_mut(&mut self, node: &'openapi mut Responses) {
        visit_responses_mut(self, node)
    }
    fn visit_reference_or_response_mut(&mut self, node: &'openapi mut ReferenceOr<Response>) {
        visit_reference_or_response_mut(self, node)
    }
    fn visit_response_mut(&mut self, node: &'openapi mut Response) {
        visit_response_mut(self, node)
    }
    fn visit_reference_or_link_mut(&mut self, node: &'openapi mut ReferenceOr<Link>) {
        visit_reference_or_link_mut(self, node)
    }
    fn visit_link_mut(&mut self, node: &'openapi mut Link) {
        visit_link_mut(self, node)
    }
    fn visit_link_operation_mut(&mut self, node: &'openapi mut LinkOperation) {
        visit_link_operation_mut(self, node)
    }
    fn visit_reference_or_request_body_mut(
        &mut self,
        node: &'openapi mut ReferenceOr<RequestBody>,
    ) {
        visit_reference_or_request_body_mut(self, node)
    }
    fn visit_request_body_mut(&mut self, node: &'openapi mut RequestBody) {
        visit_request_body_mut(self, node)
    }
    fn visit_reference_or_parameter_mut(&mut self, node: &'openapi mut ReferenceOr<Parameter>) {
        visit_reference_or_parameter_mut(self, node)
    }
    fn visit_parameter_mut(&mut self, node: &'openapi mut Parameter) {
        visit_parameter_mut(self, node)
    }
    fn visit_cookie_style_mut(&mut self, node: &'openapi mut CookieStyle) {
        visit_cookie_style_mut(self, node)
    }
    fn visit_path_style_mut(&mut self, node: &'openapi mut PathStyle) {
        visit_path_style_mut(self, node)
    }
    fn visit_parameter_data_mut(&mut self, node: &'openapi mut ParameterData) {
        visit_parameter_data_mut(self, node)
    }
    fn visit_parameter_schema_or_content_mut(
        &mut self,
        node: &'openapi mut ParameterSchemaOrContent,
    ) {
        visit_parameter_schema_or_content_mut(self, node)
    }
    fn visit_media_type_mut(&mut self, node: &'openapi mut MediaType) {
        visit_media_type_mut(self, node)
    }
    fn visit_encoding_mut(&mut self, node: &'openapi mut Encoding) {
        visit_encoding_mut(self, node)
    }
    fn visit_query_style_mut(&mut self, node: &'openapi mut QueryStyle) {
        visit_query_style_mut(self, node)
    }
    fn visit_reference_or_header_mut(&mut self, node: &'openapi mut ReferenceOr<Header>) {
        visit_reference_or_header_mut(self, node)
    }
    fn visit_header_mut(&mut self, node: &'openapi mut Header) {
        visit_header_mut(self, node)
    }
    fn visit_header_style_mut(&mut self, node: &'openapi mut HeaderStyle) {
        visit_header_style_mut(self, node)
    }
    fn visit_reference_or_example_mut(&mut self, node: &'openapi mut ReferenceOr<Example>) {
        visit_reference_or_example_mut(self, node)
    }
    fn visit_example_mut(&mut self, node: &'openapi mut Example) {
        visit_example_mut(self, node)
    }
    fn visit_reference_or_schema_mut(&mut self, node: &'openapi mut ReferenceOr<Schema>) {
        visit_reference_or_schema_mut(self, node)
    }
    fn visit_schema_mut(&mut self, node: &'openapi mut Schema) {
        visit_schema_mut(self, node)
    }
    fn visit_schema_data_mut(&mut self, node: &'openapi mut SchemaData) {
        visit_schema_data_mut(self, node)
    }
    fn visit_discriminator_mut(&mut self, node: &'openapi mut Discriminator) {
        visit_discriminator_mut(self, node)
    }
    fn visit_schema_kind_mut(&mut self, node: &'openapi mut SchemaKind) {
        visit_schema_kind_mut(self, node)
    }
    fn visit_any_schema_mut(&mut self, node: &'openapi mut AnySchema) {
        visit_any_schema_mut(self, node)
    }
    fn visit_type_mut(&mut self, node: &'openapi mut Type) {
        visit_type_mut(self, node)
    }
    fn visit_array_type_mut(&mut self, node: &'openapi mut ArrayType) {
        visit_array_type_mut(self, node)
    }
    fn visit_object_type_mut(&mut self, node: &'openapi mut ObjectType) {
        visit_object_type_mut(self, node)
    }
    fn visit_reference_or_box_schema_mut(&mut self, node: &'openapi mut ReferenceOr<Box<Schema>>) {
        visit_reference_or_box_schema_mut(self, node)
    }
    fn visit_additional_properties_mut(&mut self, node: &'openapi mut AdditionalProperties) {
        visit_additional_properties_mut(self, node)
    }
    fn visit_integer_type_mut(&mut self, node: &'openapi mut IntegerType) {
        visit_integer_type_mut(self, node)
    }
    fn visit_variant_or_unknown_or_empty_integer_format_mut(
        &mut self,
        node: &'openapi mut VariantOrUnknownOrEmpty<IntegerFormat>,
    ) {
        visit_variant_or_unknown_or_empty_integer_format_mut(self, node)
    }
    fn visit_integer_format_mut(&mut self, node: &'openapi mut IntegerFormat) {
        visit_integer_format_mut(self, node)
    }
    fn visit_number_type_mut(&mut self, node: &'openapi mut NumberType) {
        visit_number_type_mut(self, node)
    }
    fn visit_variant_or_unknown_or_empty_number_format_mut(
        &mut self,
        node: &'openapi mut VariantOrUnknownOrEmpty<NumberFormat>,
    ) {
        visit_variant_or_unknown_or_empty_number_format_mut(self, node)
    }
    fn visit_number_format_mut(&mut self, node: &'openapi mut NumberFormat) {
        visit_number_format_mut(self, node)
    }
    fn visit_string_type_mut(&mut self, node: &'openapi mut StringType) {
        visit_string_type_mut(self, node)
    }
    fn visit_variant_or_unknown_or_empty_string_format_mut(
        &mut self,
        node: &'openapi mut VariantOrUnknownOrEmpty<StringFormat>,
    ) {
        visit_variant_or_unknown_or_empty_string_format_mut(self, node)
    }
    fn visit_string_format_mut(&mut self, node: &'openapi mut StringFormat) {
        visit_string_format_mut(self, node)
    }
    fn visit_external_documentation_mut(&mut self, node: &'openapi mut ExternalDocumentation) {
        visit_external_documentation_mut(self, node)
    }
    fn visit_server_mut(&mut self, node: &'openapi mut Server) {
        visit_server_mut(self, node)
    }
    fn visit_server_variable_mut(&mut self, node: &'openapi mut ServerVariable) {
        visit_server_variable_mut(self, node)
    }
    fn visit_info_mut(&mut self, node: &'openapi mut Info) {
        visit_info_mut(self, node)
    }
    fn visit_contact_mut(&mut self, node: &'openapi mut Contact) {
        visit_contact_mut(self, node)
    }
    fn visit_license_mut(&mut self, node: &'openapi mut License) {
        visit_license_mut(self, node)
    }
}

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
        security: _,
        tags,
        external_docs,
        extensions: _,
    } = node;
    visitor.visit_info_mut(info);
    for node in servers {
        visitor.visit_server_mut(node);
    }
    visitor.visit_paths_mut(paths);
    if let Some(node) = components.as_mut() {
        visitor.visit_components_mut(node);
    }
    for node in tags {
        visitor.visit_tag_mut(node);
    }
    if let Some(node) = external_docs.as_mut() {
        visitor.visit_external_documentation_mut(node);
    }
}

pub fn visit_tag_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Tag)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let Tag {
        name: _,
        description: _,
        external_docs,
        extensions: _,
    } = node;
    if let Some(node) = external_docs.as_mut() {
        visitor.visit_external_documentation_mut(node);
    }
}

pub fn visit_components_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut Components,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let Components {
        security_schemes,
        responses,
        parameters,
        examples,
        request_bodies,
        headers,
        schemas,
        links,
        callbacks,
        extensions: _,
    } = node;
    for (_, node) in security_schemes {
        visitor.visit_reference_or_security_scheme_mut(node);
    }
    for (_, node) in responses {
        visitor.visit_reference_or_response_mut(node);
    }
    for (_, node) in parameters {
        visitor.visit_reference_or_parameter_mut(node);
    }
    for (_, node) in examples {
        visitor.visit_reference_or_example_mut(node);
    }
    for (_, node) in request_bodies {
        visitor.visit_reference_or_request_body_mut(node);
    }
    for (_, node) in headers {
        visitor.visit_reference_or_header_mut(node);
    }
    for (_, node) in schemas {
        visitor.visit_reference_or_schema_mut(node);
    }
    for (_, node) in links {
        visitor.visit_reference_or_link_mut(node);
    }
    for (_, node) in callbacks {
        visitor.visit_reference_or_callback_mut(node);
    }
}

pub fn visit_reference_or_callback_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<Callback>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visitor.visit_callback_mut(node);
    }
}

pub fn visit_callback_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Callback)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    for (_, node) in node {
        visitor.visit_path_item_mut(node);
    }
}

pub fn visit_reference_or_security_scheme_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<SecurityScheme>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visitor.visit_security_scheme_mut(node);
    }
}

pub fn visit_security_scheme_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut SecurityScheme,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        SecurityScheme::APIKey {
            location,
            name: _,
            description: _,
        } => visitor.visit_api_key_location_mut(location),
        SecurityScheme::HTTP {
            scheme: _,
            bearer_format: _,
            description: _,
        } => (),
        SecurityScheme::OAuth2 {
            flows,
            description: _,
        } => visitor.visit_oauth2_flows_mut(flows),
        SecurityScheme::OpenIDConnect {
            open_id_connect_url: _,
            description: _,
        } => (),
    }
}

pub fn visit_oauth2_flows_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut OAuth2Flows,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let OAuth2Flows {
        implicit,
        password,
        client_credentials,
        authorization_code,
    } = node;
    if let Some(node) = implicit.as_mut() {
        visitor.visit_oauth2_flow_mut(node);
    }
    if let Some(node) = password.as_mut() {
        visitor.visit_oauth2_flow_mut(node);
    }
    if let Some(node) = client_credentials.as_mut() {
        visitor.visit_oauth2_flow_mut(node);
    }
    if let Some(node) = authorization_code.as_mut() {
        visitor.visit_oauth2_flow_mut(node);
    }
}

pub fn visit_oauth2_flow_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut OAuth2Flow,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_api_key_location_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut APIKeyLocation,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
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
        visitor.visit_reference_or_path_item_mut(node);
    }
}

pub fn visit_reference_or_path_item_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<PathItem>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visitor.visit_path_item_mut(node);
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
    if let Some(node) = get.as_mut() {
        visitor.visit_operation_mut(node);
    }
    if let Some(node) = put.as_mut() {
        visitor.visit_operation_mut(node);
    }
    if let Some(node) = post.as_mut() {
        visitor.visit_operation_mut(node);
    }
    if let Some(node) = delete.as_mut() {
        visitor.visit_operation_mut(node);
    }
    if let Some(node) = options.as_mut() {
        visitor.visit_operation_mut(node);
    }
    if let Some(node) = head.as_mut() {
        visitor.visit_operation_mut(node);
    }
    if let Some(node) = patch.as_mut() {
        visitor.visit_operation_mut(node);
    }
    if let Some(node) = trace.as_mut() {
        visitor.visit_operation_mut(node);
    }
    for node in servers {
        visitor.visit_server_mut(node);
    }
    for node in parameters {
        visitor.visit_reference_or_parameter_mut(node);
    }
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
        visitor.visit_external_documentation_mut(node);
    }
    for node in parameters {
        visitor.visit_reference_or_parameter_mut(node);
    }
    if let Some(node) = request_body.as_mut() {
        visitor.visit_reference_or_request_body_mut(node);
    }
    visitor.visit_responses_mut(responses);
    for node in servers {
        visitor.visit_server_mut(node);
    }
}

pub fn visit_responses_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut Responses,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let Responses {
        default,
        responses,
        extensions: _,
    } = node;
    if let Some(node) = default.as_mut() {
        visitor.visit_reference_or_response_mut(node);
    }
    for (_, node) in responses {
        visitor.visit_reference_or_response_mut(node)
    }
}

pub fn visit_reference_or_response_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<Response>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visitor.visit_response_mut(node);
    }
}
pub fn visit_response_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Response)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let Response {
        description: _,
        headers,
        content,
        links,
        extensions: _,
    } = node;
    for (_, node) in headers {
        visitor.visit_reference_or_header_mut(node);
    }
    for (_, node) in content {
        visitor.visit_media_type_mut(node);
    }
    for (_, node) in links {
        visitor.visit_reference_or_link_mut(node);
    }
}

pub fn visit_reference_or_link_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<Link>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visitor.visit_link_mut(node);
    }
}

pub fn visit_link_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Link)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let Link {
        description: _,
        operation,
        request_body: _,
        parameters: _,
        server,
        extensions: _,
    } = node;
    visitor.visit_link_operation_mut(operation);
    if let Some(node) = server.as_mut() {
        visitor.visit_server_mut(node);
    }
}

pub fn visit_link_operation_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut LinkOperation,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_reference_or_request_body_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<RequestBody>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visitor.visit_request_body_mut(node);
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
        visitor.visit_media_type_mut(node);
    }
}

pub fn visit_reference_or_parameter_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<Parameter>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visitor.visit_parameter_mut(node);
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
            visitor.visit_parameter_data_mut(parameter_data);
            visitor.visit_query_style_mut(style);
        }
        Parameter::Header {
            parameter_data,
            style,
        } => {
            visitor.visit_parameter_data_mut(parameter_data);
            visitor.visit_header_style_mut(style);
        }
        Parameter::Path {
            parameter_data,
            style,
        } => {
            visitor.visit_parameter_data_mut(parameter_data);
            visitor.visit_path_style_mut(style);
        }
        Parameter::Cookie {
            parameter_data,
            style,
        } => {
            visitor.visit_parameter_data_mut(parameter_data);
            visitor.visit_cookie_style_mut(style);
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
    visitor.visit_parameter_schema_or_content_mut(format);
    for (_, node) in examples {
        visitor.visit_reference_or_example_mut(node);
    }
}

pub fn visit_parameter_schema_or_content_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ParameterSchemaOrContent,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        ParameterSchemaOrContent::Schema(node) => visitor.visit_reference_or_schema_mut(node),
        ParameterSchemaOrContent::Content(node) => {
            for (_, node) in node {
                visitor.visit_media_type_mut(node);
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
        visitor.visit_reference_or_schema_mut(node);
    }
    for (_, node) in examples {
        visitor.visit_reference_or_example_mut(node);
    }
    for (_, node) in encoding {
        visitor.visit_encoding_mut(node);
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
        visitor.visit_reference_or_header_mut(node);
    }
    if let Some(node) = style.as_mut() {
        visitor.visit_query_style_mut(node);
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
        visitor.visit_header_mut(node);
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
    visitor.visit_header_style_mut(style);
    visitor.visit_parameter_schema_or_content_mut(format);
    for (_, node) in examples {
        visitor.visit_reference_or_example_mut(node);
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
        visitor.visit_example_mut(node);
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
        visitor.visit_schema_mut(node);
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
    visitor.visit_schema_data_mut(schema_data);
    visitor.visit_schema_kind_mut(schema_kind);
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
        visitor.visit_external_documentation_mut(node);
    }
    if let Some(node) = discriminator.as_mut() {
        visitor.visit_discriminator_mut(node);
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
        SchemaKind::Type(node) => visitor.visit_type_mut(node),
        SchemaKind::OneOf { one_of } => {
            for node in one_of {
                visitor.visit_reference_or_schema_mut(node);
            }
        }
        SchemaKind::AllOf { all_of } => {
            for node in all_of {
                visitor.visit_reference_or_schema_mut(node);
            }
        }
        SchemaKind::AnyOf { any_of } => {
            for node in any_of {
                visitor.visit_reference_or_schema_mut(node);
            }
        }
        SchemaKind::Not { not } => visitor.visit_reference_or_schema_mut(not),
        SchemaKind::Any(node) => visitor.visit_any_schema_mut(node),
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
        visitor.visit_reference_or_box_schema_mut(node);
    }
    if let Some(node) = additional_properties.as_mut() {
        visitor.visit_additional_properties_mut(node);
    }
    if let Some(node) = items.as_mut() {
        visitor.visit_reference_or_box_schema_mut(node);
    }
    for node in one_of {
        visitor.visit_reference_or_schema_mut(node);
    }
    for node in all_of {
        visitor.visit_reference_or_schema_mut(node);
    }
    for node in any_of {
        visitor.visit_reference_or_schema_mut(node);
    }
    if let Some(node) = not.as_mut() {
        visitor.visit_reference_or_schema_mut(node);
    }
}

pub fn visit_type_mut<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi mut Type)
where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        Type::String(node) => visitor.visit_string_type_mut(node),
        Type::Number(node) => visitor.visit_number_type_mut(node),
        Type::Integer(node) => visitor.visit_integer_type_mut(node),
        Type::Object(node) => visitor.visit_object_type_mut(node),
        Type::Array(node) => visitor.visit_array_type_mut(node),
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
        visitor.visit_reference_or_box_schema_mut(node);
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
        visitor.visit_reference_or_box_schema_mut(node);
    }
    if let Some(node) = additional_properties.as_mut() {
        visitor.visit_additional_properties_mut(node);
    }
}

pub fn visit_reference_or_box_schema_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut ReferenceOr<Box<Schema>>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visitor.visit_schema_mut(node);
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
        AdditionalProperties::Schema(node) => visitor.visit_reference_or_schema_mut(node),
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
    visitor.visit_variant_or_unknown_or_empty_integer_format_mut(format);
}

pub fn visit_variant_or_unknown_or_empty_integer_format_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut VariantOrUnknownOrEmpty<IntegerFormat>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_integer_format_mut(node),
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
    visitor.visit_variant_or_unknown_or_empty_number_format_mut(format);
}

pub fn visit_variant_or_unknown_or_empty_number_format_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut VariantOrUnknownOrEmpty<NumberFormat>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_number_format_mut(node),
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
    visitor.visit_variant_or_unknown_or_empty_string_format_mut(format);
}

pub fn visit_variant_or_unknown_or_empty_string_format_mut<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi mut VariantOrUnknownOrEmpty<StringFormat>,
) where
    VisitorT: VisitMut<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visitor.visit_string_format_mut(node),
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
            visitor.visit_server_variable_mut(node);
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
        visitor.visit_contact_mut(node)
    }
    if let Some(node) = license.as_mut() {
        visitor.visit_license_mut(node);
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
