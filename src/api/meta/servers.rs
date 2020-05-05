use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerVar {
    #[serde(rename = "enum")]
    pub values: Option<Vec<String>>,
    pub default: String,
    pub description: Option<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
    pub variables: Option<HashMap<String, ServerVar>>
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_server_new() {
        let mut vars: HashMap<String, ServerVar> = HashMap::new();
        vars.insert("port".to_owned(), ServerVar{values: None, default: "8080".to_owned(), description: None});

        let o = Server {
            url: "foo.bar".to_owned(),
            description: Some("foobar".to_owned()),
            variables: Some(vars)
        };

        assert_eq!("foo.bar", o.url);
        assert_eq!("foobar", o.description.unwrap());
        assert_eq!("8080", o.variables.unwrap()["port"].default);
    }

    #[test]
    fn test_server_yaml() {
        let raw = "url: foo.bar
description: foobar
variables:
    port:
        enum: ['8080', '8443']
        default: '8080'";

        let o: Server = serde_yaml::from_str(&raw).unwrap();
        let v = o.variables.clone().unwrap();
        let values = &v["port"].values.clone().unwrap();

        assert_eq!("foo.bar", o.url);
        assert_eq!("foobar", o.description.unwrap());
        assert_eq!("8080", &v["port"].default);
        assert_eq!(&vec!["8080", "8443"], values);
    }

    #[test]
    fn test_server_json() {
        let raw = "{
    \"url\": \"foo.bar\",
    \"description\": \"foobar\",
    \"variables\": {
        \"port\": {
            \"enum\": [\"8080\", \"8443\"],
            \"default\": \"8080\"
        }
    }
}";

        let o: Server = serde_json::from_str(&raw).unwrap();
        let v = o.variables.clone().unwrap();
        let values = &v["port"].values.clone().unwrap();

        assert_eq!("foo.bar", o.url);
        assert_eq!("foobar", o.description.unwrap());
        assert_eq!("8080", &v["port"].default);
        assert_eq!(&vec!["8080", "8443"], values);
    }
}