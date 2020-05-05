use std::fs;
use lib::api::meta::document::Document;

#[test]
fn it_parses() {
     let contents = fs::read_to_string("tests/fixtures/petstore.yaml")
        .expect("Something went wrong reading the file");

    let doc: Document = serde_yaml::from_str(&contents).unwrap();

    let oi = doc.info;
    let oc = oi.contact.unwrap();
    let ol = oi.license.unwrap();
    let os = doc.servers;

    assert_eq!("3.0.0", doc.openapi);
    assert_eq!("Swagger Petstore", oi.title);
    assert_eq!(true, oi.description.is_none());
    assert_eq!(true, oi.terms_of_service.is_none());
    assert_eq!("1.0.0", oi.version);
    assert_eq!("odra", oc.name.unwrap());
    assert_eq!("foo.bar", oc.url.unwrap());
    assert_eq!("foo@bar", oc.email.unwrap());
    assert_eq!("MIT", ol.name);
    assert_eq!(true, ol.url.is_none());
    assert_eq!(1, os.len());
    assert_eq!("http://petstore.swagger.io/v1", &os[0].url);
    assert_eq!(true, os[0].description.is_none());
}