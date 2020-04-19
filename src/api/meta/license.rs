use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct License {
    pub name: String,
    pub url: Option<String>
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_license_new() {
        let o = License {
            name: "Apache-2.0".to_owned(),
            url: Some("https://www.apache.org/licenses/LICENSE-2.0.txt".to_owned())
        };
        assert_eq!("Apache-2.0", o.name);
        assert_eq!("https://www.apache.org/licenses/LICENSE-2.0.txt", o.url.unwrap());
    }

    #[test]
    fn test_licence_yaml() {
        let yaml = "name: Apache-2.0
url: https://www.apache.org/licenses/LICENSE-2.0.txt";

        let o: License = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!("Apache-2.0", o.name);
        assert_eq!("https://www.apache.org/licenses/LICENSE-2.0.txt", o.url.unwrap());
    }

    #[test]
    fn test_licence_yaml_partial() {
        let yaml = "name: Apache-2.0";

        let o: License = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!("Apache-2.0", o.name);
        assert_eq!(true, o.url.is_none());
    }

    #[test]
    fn test_licence_json() {
        let json = "{
    \"name\": \"Apache-2.0\",
    \"url\": \"https://www.apache.org/licenses/LICENSE-2.0.txt\"
}";

        let o: License = serde_json::from_str(&json).unwrap();

        assert_eq!("Apache-2.0", o.name);
        assert_eq!("https://www.apache.org/licenses/LICENSE-2.0.txt", o.url.unwrap());
    }

    #[test]
    fn test_licence_json_partial() {
        let json = "{
    \"name\": \"Apache-2.0\"
}";

        let o: License = serde_json::from_str(&json).unwrap();

        assert_eq!("Apache-2.0", o.name);
        assert_eq!(true, o.url.is_none());
    }
}