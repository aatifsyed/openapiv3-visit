use openapiv3::OpenAPI;
use openapiv3_visit::Visit;

fn openapi() -> OpenAPI {
    serde_yaml::from_str(include_str!("examples/spotify.yml")).unwrap()
}

#[test]
fn deserialize() {
    openapi();
}

#[test]
fn visit() {
    #[derive(Debug, Default)]
    struct Endpoints<'a> {
        endpoints: Vec<&'a str>,
    }
    impl<'openapi> Visit<'openapi> for Endpoints<'openapi> {
        fn visit_paths(&mut self, node: &'openapi openapiv3::Paths) {
            for (path, node) in &node.paths {
                self.endpoints.push(path);
                openapiv3_visit::visit_reference_or_path_item(self, node);
            }
        }
    }
    let openapi = openapi();
    let mut visitor = Endpoints::default();

    visitor.visit_openapi(&openapi);
    println!("{:#?}", visitor);

    assert_eq!(openapi.paths.paths.len(), visitor.endpoints.len(),)
}
