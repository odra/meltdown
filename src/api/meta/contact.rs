use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contact_new() {
        let o = Contact {
            name: Some("odra".to_owned()),
            url: Some("foo.bar".to_owned()),
            email: Some("foo@bar".to_owned())
        };

        assert_eq!("odra", o.name.unwrap());
        assert_eq!("foo.bar", o.url.unwrap());
        assert_eq!("foo@bar", o.email.unwrap());
    }

    #[test]
    fn test_contact_yaml() {
        let raw = "name: odra
url: foo.bar
email: foo@bar";

        let o: Contact = serde_yaml::from_str(&raw).unwrap();

        assert_eq!("odra", o.name.unwrap());
        assert_eq!("foo.bar", o.url.unwrap());
        assert_eq!("foo@bar", o.email.unwrap());
    }

    #[test]
    fn test_contact_yaml_partial() {
        let raw = "name: odra
email: foo@bar";

        let o: Contact = serde_yaml::from_str(&raw).unwrap();

        assert_eq!("odra", o.name.unwrap());
        assert_eq!(true, o.url.is_none());
        assert_eq!("foo@bar", o.email.unwrap());
    }

    #[test]
    fn test_contact_json() {
        let raw = "{
    \"name\": \"odra\",
    \"url\": \"foo.bar\",
    \"email\": \"foo@bar\"
}";

        let o: Contact = serde_json::from_str(&raw).unwrap();

        assert_eq!("odra", o.name.unwrap());
        assert_eq!("foo.bar", o.url.unwrap());
        assert_eq!("foo@bar", o.email.unwrap());
    }

    #[test]
    fn test_contact_json_partial() {
        let raw = "{
    \"name\": \"odra\",
    \"url\": \"foo.bar\"
}";

        let o: Contact = serde_json::from_str(&raw).unwrap();

        assert_eq!("odra", o.name.unwrap());
        assert_eq!("foo.bar", o.url.unwrap());
        assert_eq!(true, o.email.is_none());
    }
}
