use serde::{Serialize, Deserialize};
use super::{license, contact};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub version: String,
    pub contact: Option<contact::Contact>,
    pub license: Option<license::License>
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_info_new() {
        let l = license::License {
            name: "Apache-2.0".to_owned(),
            url: Some("https://www.apache.org/licenses/LICENSE-2.0.txt".to_owned())
        };
        let c = contact::Contact {
            name: Some("odra".to_owned()),
            url: Some("foo.bar".to_owned()),
            email: Some("foo@bar".to_owned())
        };

        let o = Info {
            title: "foobar-api".to_owned(),
            description: Some("foobar".to_owned()),
            terms_of_service: Some("https://foo.bar/terms".to_owned()),
            version: "v0.0.1".to_owned(),
            contact: Some(c),
            license: Some(l)
        };

        assert_eq!("foobar-api", o.title);
    }

    #[test]
    fn test_info_from_yaml() {
        let raw = "title: foobar-api
description: foobar
termsOfService: https://foo.bar/terms
version: v0.0.1
contact:
    name: odra
    url: foo.bar
    email: foo@bar
license:
    name: Apache-2.0
    url: https://www.apache.org/licenses/LICENSE-2.0.txt";

        let o: Info = serde_yaml::from_str(&raw).unwrap();
        let oc = o.contact.unwrap();
        let ol = o.license.unwrap();

        assert_eq!("foobar-api", o.title);
        assert_eq!("foobar", o.description.unwrap());
        assert_eq!("https://foo.bar/terms", o.terms_of_service.unwrap());
        assert_eq!("v0.0.1", o.version);
        assert_eq!("odra", oc.name.unwrap());
        assert_eq!("foo.bar", oc.url.unwrap());
        assert_eq!("foo@bar", oc.email.unwrap());
        assert_eq!("Apache-2.0", ol.name);
        assert_eq!("https://www.apache.org/licenses/LICENSE-2.0.txt", ol.url.unwrap());
    }

    #[test]
    fn test_info_from_yaml_partial() {
        let raw = "title: foobar-api
description: foobar
termsOfService: https://foo.bar/terms
version: v0.0.1";

        let o: Info = serde_yaml::from_str(&raw).unwrap();

        assert_eq!("foobar-api", o.title);
        assert_eq!("foobar", o.description.unwrap());
        assert_eq!("https://foo.bar/terms", o.terms_of_service.unwrap());
        assert_eq!("v0.0.1", o.version);
        assert_eq!(true, o.contact.is_none());
        assert_eq!(true, o.license.is_none());
    }

    #[test]
    fn test_info_from_json() {
        let raw = "{
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
}";

        let o: Info = serde_json::from_str(&raw).unwrap();
        let oc = o.contact.unwrap();
        let ol = o.license.unwrap();

        assert_eq!("foobar-api", o.title);
        assert_eq!("foobar", o.description.unwrap());
        assert_eq!("https://foo.bar/terms", o.terms_of_service.unwrap());
        assert_eq!("v0.0.1", o.version);
        assert_eq!("odra", oc.name.unwrap());
        assert_eq!("foo.bar", oc.url.unwrap());
        assert_eq!("foo@bar", oc.email.unwrap());
        assert_eq!("Apache-2.0", ol.name);
        assert_eq!("https://www.apache.org/licenses/LICENSE-2.0.txt", ol.url.unwrap());
    }

    #[test]
    fn test_info_from_json_partial() {
        let raw = "{
    \"title\": \"foobar-api\",
    \"description\": \"foobar\",
    \"termsOfService\": \"https://foo.bar/terms\",
    \"version\": \"v0.0.1\"
}";

        let o: Info = serde_json::from_str(&raw).unwrap();

        assert_eq!("foobar-api", o.title);
        assert_eq!("foobar", o.description.unwrap());
        assert_eq!("https://foo.bar/terms", o.terms_of_service.unwrap());
        assert_eq!("v0.0.1", o.version);
        assert_eq!(true, o.contact.is_none());
        assert_eq!(true, o.license.is_none());
    }
}