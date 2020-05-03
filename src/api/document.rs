use serde::{Serialize, Deserialize};
use super::{servers, meta};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Document {
    #[serde(default = "default_openapi")]
    pub openapi: String,
    pub info: meta::info::Info,
    pub servers: Vec<servers::Server>
}

fn default_openapi() -> String {
    "3.0.0".to_owned()
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::api::{meta, servers};
    use std::collections::HashMap;

    #[test]
    fn test_document_new() {
        let l = meta::license::License {
            name: "Apache-2.0".to_owned(),
            url: Some("https://www.apache.org/licenses/LICENSE-2.0.txt".to_owned())
        };

        let c = meta::contact::Contact {
            name: Some("odra".to_owned()),
            url: Some("foo.bar".to_owned()),
            email: Some("foo@bar".to_owned())
        };

        let i = meta::info::Info {
            title: "foobar-api".to_owned(),
            description: Some("foobar".to_owned()),
            terms_of_service: Some("https://foo.bar/terms".to_owned()),
            version: "v0.0.1".to_owned(),
            contact: Some(c),
            license: Some(l)
        };

        let mut vars: HashMap<String, servers::ServerVar> = HashMap::new();
        let allowed_values = vec!["8080".to_owned(), "8443".to_owned()];
        vars.insert("port".to_owned(), servers::ServerVar{values: Some(allowed_values), default: "8080".to_owned(), description: None});

        let s = servers::Server {
            url: "foo.bar".to_owned(),
            description: Some("foobar".to_owned()),
            variables: Some(vars)
        };

        let o = Document {
            openapi: "3.0.0".to_owned(),
            info: i,
            servers: vec![s]
        };

        let oi = o.info;
        let oc = oi.contact.unwrap();
        let ol = oi.license.unwrap();
        let os = o.servers;

        assert_eq!("3.0.0", o.openapi);
        assert_eq!("foobar-api", oi.title);
        assert_eq!("foobar", oi.description.unwrap());
        assert_eq!("https://foo.bar/terms", oi.terms_of_service.unwrap());
        assert_eq!("v0.0.1", oi.version);
        assert_eq!("odra", oc.name.unwrap());
        assert_eq!("foo.bar", oc.url.unwrap());
        assert_eq!("foo@bar", oc.email.unwrap());
        assert_eq!("Apache-2.0", ol.name);
        assert_eq!("https://www.apache.org/licenses/LICENSE-2.0.txt", ol.url.unwrap());
        assert_eq!(1, os.len());
        assert_eq!("foo.bar", &os[0].url);
        assert_eq!("foobar", os[0].description.as_ref().unwrap());
        assert_eq!("8080", os[0].variables.as_ref().unwrap()["port"].default);
        assert_eq!(vec!["8080".to_owned(), "8443".to_owned()], os[0].variables.as_ref().unwrap()["port"].values.clone().unwrap());
    }

    #[test]
    fn test_document_yaml() {
        let raw = "openapi: 3.0.0
info:
  title: foobar-api
  description: foobar
  termsOfService: https://foo.bar/terms
  version: v0.0.1
  contact:
    name: odra
    url: foo.bar
    email: foo@bar
  license:
    name: Apache-2.0
    url: https://www.apache.org/licenses/LICENSE-2.0.txt
servers:
  - url: foo.bar
    description: foobar
    variables:
      port:
        enum: ['8080', '8443']
        default: '8080'";
        
        let o: Document = serde_yaml::from_str(&raw).unwrap();
        let oi = o.info;
        let oc = oi.contact.unwrap();
        let ol = oi.license.unwrap();
        let os = o.servers;

        assert_eq!("3.0.0", o.openapi);
        assert_eq!("foobar-api", oi.title);
        assert_eq!("foobar", oi.description.unwrap());
        assert_eq!("https://foo.bar/terms", oi.terms_of_service.unwrap());
        assert_eq!("v0.0.1", oi.version);
        assert_eq!("odra", oc.name.unwrap());
        assert_eq!("foo.bar", oc.url.unwrap());
        assert_eq!("foo@bar", oc.email.unwrap());
        assert_eq!("Apache-2.0", ol.name);
        assert_eq!("https://www.apache.org/licenses/LICENSE-2.0.txt", ol.url.unwrap());
        assert_eq!(1, os.len());
        assert_eq!("foo.bar", &os[0].url);
        assert_eq!("foobar", os[0].description.as_ref().unwrap());
        assert_eq!("8080", os[0].variables.as_ref().unwrap()["port"].default);
        assert_eq!(vec!["8080".to_owned(), "8443".to_owned()], os[0].variables.as_ref().unwrap()["port"].values.clone().unwrap());   
    }

    #[test]
    fn test_document_json() {
        let raw = "{
   \"openapi\": \"3.0.0\",
   \"info\": {
      \"title\": \"foobar-api\",
      \"description\": \"foobar\",
      \"termsOfService\": \"https://foo.bar/terms\",
      \"version\": \"v0.0.1\",
      \"contact\": {
         \"name\": \"odra\",
         \"url\": \"foo.bar\",
         \"email\": \"foo@bar\"
      },
      \"license\": {
         \"name\": \"Apache-2.0\",
         \"url\": \"https://www.apache.org/licenses/LICENSE-2.0.txt\"
      }
   },
   \"servers\": [
      {
         \"url\": \"foo.bar\",
         \"description\": \"foobar\",
         \"variables\": {
            \"port\": {
               \"enum\": [
                  \"8080\",
                  \"8443\"
               ],
               \"default\": \"8080\"
            }
         }
      }
   ]
}";
        
        let o: Document = serde_json::from_str(&raw).unwrap();
        let oi = o.info;
        let oc = oi.contact.unwrap();
        let ol = oi.license.unwrap();
        let os = o.servers;

        assert_eq!("3.0.0", o.openapi);
        assert_eq!("foobar-api", oi.title);
        assert_eq!("foobar", oi.description.unwrap());
        assert_eq!("https://foo.bar/terms", oi.terms_of_service.unwrap());
        assert_eq!("v0.0.1", oi.version);
        assert_eq!("odra", oc.name.unwrap());
        assert_eq!("foo.bar", oc.url.unwrap());
        assert_eq!("foo@bar", oc.email.unwrap());
        assert_eq!("Apache-2.0", ol.name);
        assert_eq!("https://www.apache.org/licenses/LICENSE-2.0.txt", ol.url.unwrap());
        assert_eq!(1, os.len());
        assert_eq!("foo.bar", &os[0].url);
        assert_eq!("foobar", os[0].description.as_ref().unwrap());
        assert_eq!("8080", os[0].variables.as_ref().unwrap()["port"].default);
        assert_eq!(vec!["8080".to_owned(), "8443".to_owned()], os[0].variables.as_ref().unwrap()["port"].values.clone().unwrap());  
    }
}
