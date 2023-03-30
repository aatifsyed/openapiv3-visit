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

pub trait Visit<'openapi> {
    fn visit_openapi(&mut self, node: &'openapi OpenAPI) {
        visit_openapi(self, node)
    }
    fn visit_tag(&mut self, node: &'openapi Tag) {
        visit_tag(self, node)
    }
    fn visit_components(&mut self, node: &'openapi Components) {
        visit_components(self, node)
    }
    fn visit_reference_or_callback(&mut self, node: &'openapi ReferenceOr<Callback>) {
        visit_reference_or_callback(self, node)
    }
    fn visit_callback(&mut self, node: &'openapi Callback) {
        visit_callback(self, node)
    }
    fn visit_reference_or_security_scheme(&mut self, node: &'openapi ReferenceOr<SecurityScheme>) {
        visit_reference_or_security_scheme(self, node)
    }
    fn visit_security_scheme(&mut self, node: &'openapi SecurityScheme) {
        visit_security_scheme(self, node)
    }
    fn visit_oauth2_flows(&mut self, node: &'openapi OAuth2Flows) {
        visit_oauth2_flows(self, node)
    }
    fn visit_oauth2_flow(&mut self, node: &'openapi OAuth2Flow) {
        visit_oauth2_flow(self, node)
    }
    fn visit_api_key_location(&mut self, node: &'openapi APIKeyLocation) {
        visit_api_key_location(self, node)
    }
    fn visit_paths(&mut self, node: &'openapi Paths) {
        visit_paths(self, node)
    }
    fn visit_reference_or_path_item(&mut self, node: &'openapi ReferenceOr<PathItem>) {
        visit_reference_or_path_item(self, node)
    }
    fn visit_path_item(&mut self, node: &'openapi PathItem) {
        visit_path_item(self, node)
    }
    fn visit_operation(&mut self, node: &'openapi Operation) {
        visit_operation(self, node)
    }
    fn visit_responses(&mut self, node: &'openapi Responses) {
        visit_responses(self, node)
    }
    fn visit_reference_or_response(&mut self, node: &'openapi ReferenceOr<Response>) {
        visit_reference_or_response(self, node)
    }
    fn visit_response(&mut self, node: &'openapi Response) {
        visit_response(self, node)
    }
    fn visit_reference_or_link(&mut self, node: &'openapi ReferenceOr<Link>) {
        visit_reference_or_link(self, node)
    }
    fn visit_link(&mut self, node: &'openapi Link) {
        visit_link(self, node)
    }
    fn visit_link_operation(&mut self, node: &'openapi LinkOperation) {
        visit_link_operation(self, node)
    }
    fn visit_reference_or_request_body(&mut self, node: &'openapi ReferenceOr<RequestBody>) {
        visit_reference_or_request_body(self, node)
    }
    fn visit_request_body(&mut self, node: &'openapi RequestBody) {
        visit_request_body(self, node)
    }
    fn visit_reference_or_parameter(&mut self, node: &'openapi ReferenceOr<Parameter>) {
        visit_reference_or_parameter(self, node)
    }
    fn visit_parameter(&mut self, node: &'openapi Parameter) {
        visit_parameter(self, node)
    }
    fn visit_cookie_style(&mut self, node: &'openapi CookieStyle) {
        visit_cookie_style(self, node)
    }
    fn visit_path_style(&mut self, node: &'openapi PathStyle) {
        visit_path_style(self, node)
    }
    fn visit_parameter_data(&mut self, node: &'openapi ParameterData) {
        visit_parameter_data(self, node)
    }
    fn visit_parameter_schema_or_content(&mut self, node: &'openapi ParameterSchemaOrContent) {
        visit_parameter_schema_or_content(self, node)
    }
    fn visit_media_type(&mut self, node: &'openapi MediaType) {
        visit_media_type(self, node)
    }
    fn visit_encoding(&mut self, node: &'openapi Encoding) {
        visit_encoding(self, node)
    }
    fn visit_query_style(&mut self, node: &'openapi QueryStyle) {
        visit_query_style(self, node)
    }
    fn visit_reference_or_header(&mut self, node: &'openapi ReferenceOr<Header>) {
        visit_reference_or_header(self, node)
    }
    fn visit_header(&mut self, node: &'openapi Header) {
        visit_header(self, node)
    }
    fn visit_header_style(&mut self, node: &'openapi HeaderStyle) {
        visit_header_style(self, node)
    }
    fn visit_reference_or_example(&mut self, node: &'openapi ReferenceOr<Example>) {
        visit_reference_or_example(self, node)
    }
    fn visit_example(&mut self, node: &'openapi Example) {
        visit_example(self, node)
    }
    fn visit_reference_or_schema(&mut self, node: &'openapi ReferenceOr<Schema>) {
        visit_reference_or_schema(self, node)
    }
    fn visit_schema(&mut self, node: &'openapi Schema) {
        visit_schema(self, node)
    }
    fn visit_schema_data(&mut self, node: &'openapi SchemaData) {
        visit_schema_data(self, node)
    }
    fn visit_discriminator(&mut self, node: &'openapi Discriminator) {
        visit_discriminator(self, node)
    }
    fn visit_schema_kind(&mut self, node: &'openapi SchemaKind) {
        visit_schema_kind(self, node)
    }
    fn visit_any_schema(&mut self, node: &'openapi AnySchema) {
        visit_any_schema(self, node)
    }
    fn visit_type(&mut self, node: &'openapi Type) {
        visit_type(self, node)
    }
    fn visit_array_type(&mut self, node: &'openapi ArrayType) {
        visit_array_type(self, node)
    }
    fn visit_object_type(&mut self, node: &'openapi ObjectType) {
        visit_object_type(self, node)
    }
    fn visit_reference_or_box_schema(&mut self, node: &'openapi ReferenceOr<Box<Schema>>) {
        visit_reference_or_box_schema(self, node)
    }
    fn visit_additional_properties(&mut self, node: &'openapi AdditionalProperties) {
        visit_additional_properties(self, node)
    }
    fn visit_integer_type(&mut self, node: &'openapi IntegerType) {
        visit_integer_type(self, node)
    }
    fn visit_variant_or_unknown_or_empty_integer_format(
        &mut self,
        node: &'openapi VariantOrUnknownOrEmpty<IntegerFormat>,
    ) {
        visit_variant_or_unknown_or_empty_integer_format(self, node)
    }
    fn visit_integer_format(&mut self, node: &'openapi IntegerFormat) {
        visit_integer_format(self, node)
    }
    fn visit_number_type(&mut self, node: &'openapi NumberType) {
        visit_number_type(self, node)
    }
    fn visit_variant_or_unknown_or_empty_number_format(
        &mut self,
        node: &'openapi VariantOrUnknownOrEmpty<NumberFormat>,
    ) {
        visit_variant_or_unknown_or_empty_number_format(self, node)
    }
    fn visit_number_format(&mut self, node: &'openapi NumberFormat) {
        visit_number_format(self, node)
    }
    fn visit_string_type(&mut self, node: &'openapi StringType) {
        visit_string_type(self, node)
    }
    fn visit_variant_or_unknown_or_empty_string_format(
        &mut self,
        node: &'openapi VariantOrUnknownOrEmpty<StringFormat>,
    ) {
        visit_variant_or_unknown_or_empty_string_format(self, node)
    }
    fn visit_string_format(&mut self, node: &'openapi StringFormat) {
        visit_string_format(self, node)
    }
    fn visit_external_documentation(&mut self, node: &'openapi ExternalDocumentation) {
        visit_external_documentation(self, node)
    }
    fn visit_server(&mut self, node: &'openapi Server) {
        visit_server(self, node)
    }
    fn visit_server_variable(&mut self, node: &'openapi ServerVariable) {
        visit_server_variable(self, node)
    }
    fn visit_info(&mut self, node: &'openapi Info) {
        visit_info(self, node)
    }
    fn visit_contact(&mut self, node: &'openapi Contact) {
        visit_contact(self, node)
    }
    fn visit_license(&mut self, node: &'openapi License) {
        visit_license(self, node)
    }
}

pub fn visit_openapi<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi OpenAPI)
where
    VisitorT: Visit<'openapi> + ?Sized,
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
    visit_info(visitor, info);
    for node in servers {
        visit_server(visitor, node);
    }
    visit_paths(visitor, paths);
    if let Some(node) = components.as_ref() {
        visit_components(visitor, node);
    }
    for node in tags {
        visit_tag(visitor, node);
    }
    if let Some(node) = external_docs.as_ref() {
        visit_external_documentation(visitor, node);
    }
}

pub fn visit_tag<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Tag)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let Tag {
        name: _,
        description: _,
        external_docs,
        extensions: _,
    } = node;
    if let Some(node) = external_docs.as_ref() {
        visit_external_documentation(visitor, node);
    }
}

pub fn visit_components<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Components)
where
    VisitorT: Visit<'openapi> + ?Sized,
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
        visit_reference_or_security_scheme(visitor, node);
    }
    for (_, node) in responses {
        visit_reference_or_response(visitor, node);
    }
    for (_, node) in parameters {
        visit_reference_or_parameter(visitor, node);
    }
    for (_, node) in examples {
        visit_reference_or_example(visitor, node);
    }
    for (_, node) in request_bodies {
        visit_reference_or_request_body(visitor, node);
    }
    for (_, node) in headers {
        visit_reference_or_header(visitor, node);
    }
    for (_, node) in schemas {
        visit_reference_or_schema(visitor, node);
    }
    for (_, node) in links {
        visit_reference_or_link(visitor, node);
    }
    for (_, node) in callbacks {
        visit_reference_or_callback(visitor, node);
    }
}

pub fn visit_reference_or_callback<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ReferenceOr<Callback>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_callback(visitor, node);
    }
}

pub fn visit_callback<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Callback)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    for (_, node) in node {
        visit_path_item(visitor, node);
    }
}

pub fn visit_reference_or_security_scheme<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ReferenceOr<SecurityScheme>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_security_scheme(visitor, node);
    }
}

pub fn visit_security_scheme<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi SecurityScheme,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    match node {
        SecurityScheme::APIKey {
            location,
            name: _,
            description: _,
        } => visit_api_key_location(visitor, location),
        SecurityScheme::HTTP {
            scheme: _,
            bearer_format: _,
            description: _,
        } => (),
        SecurityScheme::OAuth2 {
            flows,
            description: _,
        } => visit_oauth2_flows(visitor, flows),
        SecurityScheme::OpenIDConnect {
            open_id_connect_url: _,
            description: _,
        } => (),
    }
}

pub fn visit_oauth2_flows<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi OAuth2Flows)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let OAuth2Flows {
        implicit,
        password,
        client_credentials,
        authorization_code,
    } = node;
    if let Some(node) = implicit.as_ref() {
        visit_oauth2_flow(visitor, node);
    }
    if let Some(node) = password.as_ref() {
        visit_oauth2_flow(visitor, node);
    }
    if let Some(node) = client_credentials.as_ref() {
        visit_oauth2_flow(visitor, node);
    }
    if let Some(node) = authorization_code.as_ref() {
        visit_oauth2_flow(visitor, node);
    }
}

pub fn visit_oauth2_flow<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi OAuth2Flow)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_api_key_location<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi APIKeyLocation,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_paths<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Paths)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let Paths {
        paths,
        extensions: _,
    } = node;
    for (_, node) in paths {
        visit_reference_or_path_item(visitor, node);
    }
}

pub fn visit_reference_or_path_item<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ReferenceOr<PathItem>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_path_item(visitor, node);
    }
}

pub fn visit_path_item<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi PathItem)
where
    VisitorT: Visit<'openapi> + ?Sized,
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
    if let Some(node) = get.as_ref() {
        visit_operation(visitor, node);
    }
    if let Some(node) = put.as_ref() {
        visit_operation(visitor, node);
    }
    if let Some(node) = post.as_ref() {
        visit_operation(visitor, node);
    }
    if let Some(node) = delete.as_ref() {
        visit_operation(visitor, node);
    }
    if let Some(node) = options.as_ref() {
        visit_operation(visitor, node);
    }
    if let Some(node) = head.as_ref() {
        visit_operation(visitor, node);
    }
    if let Some(node) = patch.as_ref() {
        visit_operation(visitor, node);
    }
    if let Some(node) = trace.as_ref() {
        visit_operation(visitor, node);
    }
    for node in servers {
        visit_server(visitor, node);
    }
    for node in parameters {
        visit_reference_or_parameter(visitor, node);
    }
}

pub fn visit_operation<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Operation)
where
    VisitorT: Visit<'openapi> + ?Sized,
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
    if let Some(node) = external_docs.as_ref() {
        visit_external_documentation(visitor, node);
    }
    for node in parameters {
        visit_reference_or_parameter(visitor, node);
    }
    if let Some(node) = request_body.as_ref() {
        visit_reference_or_request_body(visitor, node);
    }
    visit_responses(visitor, responses);
    for node in servers {
        visit_server(visitor, node);
    }
}

pub fn visit_responses<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Responses)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let Responses {
        default,
        responses,
        extensions: _,
    } = node;
    if let Some(node) = default.as_ref() {
        visit_reference_or_response(visitor, node);
    }
    for (_, node) in responses {
        visit_reference_or_response(visitor, node)
    }
}

pub fn visit_reference_or_response<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ReferenceOr<Response>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_response(visitor, node);
    }
}
pub fn visit_response<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Response)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let Response {
        description: _,
        headers,
        content,
        links,
        extensions: _,
    } = node;
    for (_, node) in headers {
        visit_reference_or_header(visitor, node);
    }
    for (_, node) in content {
        visit_media_type(visitor, node);
    }
    for (_, node) in links {
        visit_reference_or_link(visitor, node);
    }
}

pub fn visit_reference_or_link<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ReferenceOr<Link>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_link(visitor, node);
    }
}

pub fn visit_link<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Link)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let Link {
        description: _,
        operation,
        request_body: _,
        parameters: _,
        server,
        extensions: _,
    } = node;
    visit_link_operation(visitor, operation);
    if let Some(node) = server.as_ref() {
        visit_server(visitor, node);
    }
}

pub fn visit_link_operation<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi LinkOperation,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_reference_or_request_body<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ReferenceOr<RequestBody>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_request_body(visitor, node);
    }
}

pub fn visit_request_body<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi RequestBody)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let RequestBody {
        description: _,
        content,
        required: _,
        extensions: _,
    } = node;
    for (_, node) in content {
        visit_media_type(visitor, node);
    }
}

pub fn visit_reference_or_parameter<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ReferenceOr<Parameter>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_parameter(visitor, node);
    }
}
pub fn visit_parameter<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Parameter)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    match node {
        Parameter::Query {
            parameter_data,
            allow_reserved: _,
            style,
            allow_empty_value: _,
        } => {
            visit_parameter_data(visitor, parameter_data);
            visit_query_style(visitor, style);
        }
        Parameter::Header {
            parameter_data,
            style,
        } => {
            visit_parameter_data(visitor, parameter_data);
            visit_header_style(visitor, style);
        }
        Parameter::Path {
            parameter_data,
            style,
        } => {
            visit_parameter_data(visitor, parameter_data);
            visit_path_style(visitor, style);
        }
        Parameter::Cookie {
            parameter_data,
            style,
        } => {
            visit_parameter_data(visitor, parameter_data);
            visit_cookie_style(visitor, style);
        }
    }
}

pub fn visit_cookie_style<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi CookieStyle)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_path_style<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi PathStyle)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_parameter_data<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ParameterData,
) where
    VisitorT: Visit<'openapi> + ?Sized,
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
    visit_parameter_schema_or_content(visitor, format);
    for (_, node) in examples {
        visit_reference_or_example(visitor, node);
    }
}

pub fn visit_parameter_schema_or_content<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ParameterSchemaOrContent,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    match node {
        ParameterSchemaOrContent::Schema(node) => visit_reference_or_schema(visitor, node),
        ParameterSchemaOrContent::Content(node) => {
            for (_, node) in node {
                visit_media_type(visitor, node);
            }
        }
    }
}

pub fn visit_media_type<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi MediaType)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let MediaType {
        schema,
        example: _,
        examples,
        encoding,
        extensions: _,
    } = node;
    if let Some(node) = schema.as_ref() {
        visit_reference_or_schema(visitor, node);
    }
    for (_, node) in examples {
        visit_reference_or_example(visitor, node);
    }
    for (_, node) in encoding {
        visit_encoding(visitor, node);
    }
}

pub fn visit_encoding<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Encoding)
where
    VisitorT: Visit<'openapi> + ?Sized,
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
        visit_reference_or_header(visitor, node);
    }
    if let Some(node) = style.as_ref() {
        visit_query_style(visitor, node);
    }
}

pub fn visit_query_style<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi QueryStyle)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_reference_or_header<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ReferenceOr<Header>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_header(visitor, node);
    }
}

pub fn visit_header<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Header)
where
    VisitorT: Visit<'openapi> + ?Sized,
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
    visit_header_style(visitor, style);
    visit_parameter_schema_or_content(visitor, format);
    for (_, node) in examples {
        visit_reference_or_example(visitor, node);
    }
}

pub fn visit_header_style<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi HeaderStyle)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_reference_or_example<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ReferenceOr<Example>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_example(visitor, node);
    }
}

pub fn visit_example<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Example)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_reference_or_schema<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ReferenceOr<Schema>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_schema(visitor, node);
    }
}

pub fn visit_schema<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Schema)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let Schema {
        schema_data,
        schema_kind,
    } = node;
    visit_schema_data(visitor, schema_data);
    visit_schema_kind(visitor, schema_kind);
}

pub fn visit_schema_data<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi SchemaData)
where
    VisitorT: Visit<'openapi> + ?Sized,
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
    if let Some(node) = external_docs.as_ref() {
        visit_external_documentation(visitor, node);
    }
    if let Some(node) = discriminator.as_ref() {
        visit_discriminator(visitor, node);
    }
}

pub fn visit_discriminator<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi Discriminator,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_schema_kind<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi SchemaKind)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    match node {
        SchemaKind::Type(node) => visit_type(visitor, node),
        SchemaKind::OneOf { one_of } => {
            for node in one_of {
                visit_reference_or_schema(visitor, node);
            }
        }
        SchemaKind::AllOf { all_of } => {
            for node in all_of {
                visit_reference_or_schema(visitor, node);
            }
        }
        SchemaKind::AnyOf { any_of } => {
            for node in any_of {
                visit_reference_or_schema(visitor, node);
            }
        }
        SchemaKind::Not { not } => visit_reference_or_schema(visitor, not),
        SchemaKind::Any(node) => visit_any_schema(visitor, node),
    }
}

pub fn visit_any_schema<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi AnySchema)
where
    VisitorT: Visit<'openapi> + ?Sized,
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
        visit_reference_or_box_schema(visitor, node);
    }
    if let Some(node) = additional_properties.as_ref() {
        visit_additional_properties(visitor, node);
    }
    if let Some(node) = items.as_ref() {
        visit_reference_or_box_schema(visitor, node);
    }
    for node in one_of {
        visit_reference_or_schema(visitor, node);
    }
    for node in all_of {
        visit_reference_or_schema(visitor, node);
    }
    for node in any_of {
        visit_reference_or_schema(visitor, node);
    }
    if let Some(node) = not.as_ref() {
        visit_reference_or_schema(visitor, node);
    }
}

pub fn visit_type<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Type)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    match node {
        Type::String(node) => visit_string_type(visitor, node),
        Type::Number(node) => visit_number_type(visitor, node),
        Type::Integer(node) => visit_integer_type(visitor, node),
        Type::Object(node) => visit_object_type(visitor, node),
        Type::Array(node) => visit_array_type(visitor, node),
        Type::Boolean {} => (),
    }
}

pub fn visit_array_type<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi ArrayType)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let ArrayType {
        items,
        min_items: _,
        max_items: _,
        unique_items: _,
    } = node;
    if let Some(node) = items.as_ref() {
        visit_reference_or_box_schema(visitor, node);
    }
}

pub fn visit_object_type<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi ObjectType)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let ObjectType {
        properties,
        required: _,
        additional_properties,
        min_properties: _,
        max_properties: _,
    } = node;
    for (_, node) in properties {
        visit_reference_or_box_schema(visitor, node);
    }
    if let Some(node) = additional_properties.as_ref() {
        visit_additional_properties(visitor, node);
    }
}

pub fn visit_reference_or_box_schema<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ReferenceOr<Box<Schema>>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    if let Item(node) = node {
        visit_schema(visitor, node);
    }
}

pub fn visit_additional_properties<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi AdditionalProperties,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    match node {
        AdditionalProperties::Any(_) => (),
        AdditionalProperties::Schema(node) => visit_reference_or_schema(visitor, node),
    }
}

pub fn visit_integer_type<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi IntegerType)
where
    VisitorT: Visit<'openapi> + ?Sized,
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
    visit_variant_or_unknown_or_empty_integer_format(visitor, format);
}

pub fn visit_variant_or_unknown_or_empty_integer_format<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi VariantOrUnknownOrEmpty<IntegerFormat>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visit_integer_format(visitor, node),
        VariantOrUnknownOrEmpty::Unknown(_) => (),
        VariantOrUnknownOrEmpty::Empty => (),
    }
}

pub fn visit_integer_format<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi IntegerFormat,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_number_type<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi NumberType)
where
    VisitorT: Visit<'openapi> + ?Sized,
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
    visit_variant_or_unknown_or_empty_number_format(visitor, format);
}

pub fn visit_variant_or_unknown_or_empty_number_format<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi VariantOrUnknownOrEmpty<NumberFormat>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visit_number_format(visitor, node),
        VariantOrUnknownOrEmpty::Unknown(_) => (),
        VariantOrUnknownOrEmpty::Empty => (),
    }
}

pub fn visit_number_format<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi NumberFormat)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_string_type<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi StringType)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let StringType {
        format,
        pattern: _,
        enumeration: _,
        min_length: _,
        max_length: _,
    } = node;
    visit_variant_or_unknown_or_empty_string_format(visitor, format);
}

pub fn visit_variant_or_unknown_or_empty_string_format<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi VariantOrUnknownOrEmpty<StringFormat>,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    match node {
        VariantOrUnknownOrEmpty::Item(node) => visit_string_format(visitor, node),
        VariantOrUnknownOrEmpty::Unknown(_) => (),
        VariantOrUnknownOrEmpty::Empty => (),
    }
}

pub fn visit_string_format<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi StringFormat)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_external_documentation<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ExternalDocumentation,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_server<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Server)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let Server {
        url: _,
        description: _,
        variables,
        extensions: _,
    } = node;
    if let Some(node) = variables.as_ref() {
        for (_, node) in node {
            visit_server_variable(visitor, node);
        }
    }
}

pub fn visit_server_variable<'openapi, VisitorT>(
    visitor: &mut VisitorT,
    node: &'openapi ServerVariable,
) where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_info<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Info)
where
    VisitorT: Visit<'openapi> + ?Sized,
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
    if let Some(node) = contact.as_ref() {
        visit_contact(visitor, node)
    }
    if let Some(node) = license.as_ref() {
        visit_license(visitor, node);
    }
}

pub fn visit_contact<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi Contact)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}

pub fn visit_license<'openapi, VisitorT>(visitor: &mut VisitorT, node: &'openapi License)
where
    VisitorT: Visit<'openapi> + ?Sized,
{
    let (_, _) = (visitor, node);
}
