#[derive(Debug)]
pub struct Url {
    scheme: Scheme,
    host: String,
    port: u16,
    path: String,
    query: Vec<(String, String)>,
    fragment: String,
}

#[derive(Debug, PartialEq)]
pub enum Scheme {
    HTTP,
    HTTPS,
}

impl Scheme {
    fn default_port(&self) -> u16 {
        match self {
            Self::HTTP => 80,
            Self::HTTPS => 443,
        }
    }
}

impl From<&str> for Url {
    fn from(value: &str) -> Self {
        let (value, scheme) = if let Some(res) = value.strip_prefix("http://") {
            (res, Scheme::HTTP)
        } else if let Some(res) = value.strip_prefix("https://") {
            (res, Scheme::HTTPS)
        } else {
            (value, Scheme::HTTP)
        };

        let path_start = value.find("/");
        let port_start = value.find(":");

        let (host, port, value): (&str, u16, &str) = match (path_start, port_start) {
            (Some(path), Some(port)) => {
                if port < path {
                    ("", 0, value)
                } else {
                    (&value[..path], scheme.default_port(), &value[path+1..])
                }
            }
            (None, Some(port)) => (
                &value[..port],
                value[port+1..].parse().unwrap_or(scheme.default_port()),
                "",
            ),
            _ => (value, scheme.default_port(), ""),
        };

        Self {
            scheme,
            host: host.to_string(),
            port,
            path: value.to_string(),
            query: Vec::new(),
            fragment: "".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn url_http() {
        let uri = "http://test.com";
        let url = Url::from(uri);

        println!("{}", uri);
        println!("{:#?}", url);

        assert!(url.scheme == Scheme::HTTP);
        assert!(url.host == "test.com");
        assert!(url.port == 80);
        assert!(url.path.is_empty());
        assert!(url.query.is_empty());
        assert!(url.fragment.is_empty());
    }

    #[test]
    fn url_https() {
        let uri = "https://test.com";
        let url = Url::from(uri);

        println!("{}", uri);
        println!("{:#?}", url);

        assert!(url.scheme == Scheme::HTTPS);
        assert!(url.host == "test.com");
        assert!(url.port == 443);
        assert!(url.path.is_empty());
        assert!(url.query.is_empty());
        assert!(url.fragment.is_empty());
    }

    #[test]
    fn url_with_port() {
        let uri = "http://test.com:3000";
        let url = Url::from(uri);

        println!("{}", uri);
        println!("{:#?}", url);

        assert!(url.scheme == Scheme::HTTP);
        assert!(url.host == "test.com");
        assert!(url.port == 3000);
        assert!(url.path.is_empty());
        assert!(url.query.is_empty());
        assert!(url.fragment.is_empty());
    }
}
