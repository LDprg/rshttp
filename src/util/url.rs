#[derive(Debug)]
#[allow(dead_code)]
pub struct Url<'a> {
    scheme: Scheme,
    host: &'a str,
    port: u16,
    path: &'a str,
    query: Query<'a>,
    fragment: &'a str,
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

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Query<'a> {
    data: Vec<(&'a str, &'a str)>,
}

impl<'a> From<&'a str> for Query<'a> {
    fn from(value: &'a str) -> Self {
        let value: Vec<&str> = value.split("&").collect();
        let data = value
            .iter()
            .map(|i| {
                let value_start = i.find('=');
                let value: (&str, &str) = match value_start {
                    Some(val) => (&i[..val], &i[val + 1..]),
                    None => (i, ""),
                };
                value
            })
            .collect();
        Self { data }
    }
}

#[allow(dead_code)]
impl<'a> Query<'a> {
    const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn to_vec(self) -> Vec<(&'a str, &'a str)> {
        self.data
    }
}

impl<'a> From<&'a str> for Url<'a> {
    fn from(value: &'a str) -> Self {
        let (value, scheme) = if let Some(res) = value.strip_prefix("http://") {
            (res, Scheme::HTTP)
        } else if let Some(res) = value.strip_prefix("https://") {
            (res, Scheme::HTTPS)
        } else {
            (value, Scheme::HTTP)
        };

        let path_start = value.find('/');
        let port_start = value.find(':');

        let (host, port, value): (&str, u16, &str) = match (path_start, port_start) {
            (Some(path), Some(port)) => {
                if port < path {
                    (
                        &value[..port],
                        value[port + 1..path]
                            .parse()
                            .unwrap_or(scheme.default_port()),
                        &value[path..],
                    )
                } else {
                    (&value[..path], scheme.default_port(), &value[path..])
                }
            }
            (None, Some(port)) => (
                &value[..port],
                value[port + 1..].parse().unwrap_or(scheme.default_port()),
                "",
            ),
            (Some(path), None) => (&value[..path], scheme.default_port(), &value[path..]),
            (None, None) => (value, scheme.default_port(), ""),
        };

        let fragment_start = value.find('#');

        let (fragment, value): (&str, &str) = match fragment_start {
            Some(frag) => (&value[frag + 1..], &value[..frag]),
            None => ("", value),
        };

        let query_start = value.find('?');

        let (query, path): (Query, &str) = match query_start {
            Some(query) => (Query::from(&value[query + 1..]), &value[..query]),
            None => (Query::default(), value),
        };

        Self {
            scheme,
            host,
            port,
            path,
            query,
            fragment,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_url_http() {
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
    fn parse_url_https() {
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
    fn parse_url_with_port() {
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

    #[test]
    fn parse_url_with_path() {
        let uri = "https://a.test.com/test";
        let url = Url::from(uri);

        println!("{}", uri);
        println!("{:#?}", url);

        assert!(url.scheme == Scheme::HTTPS);
        assert!(url.host == "a.test.com");
        assert!(url.port == 443);
        assert!(url.path == "/test");
        assert!(url.query.is_empty());
        assert!(url.fragment.is_empty());
    }

    #[test]
    fn parse_url_with_port_and_path() {
        let uri = "https://a.test.com:7888/test";
        let url = Url::from(uri);

        println!("{}", uri);
        println!("{:#?}", url);

        assert!(url.scheme == Scheme::HTTPS);
        assert!(url.host == "a.test.com");
        assert!(url.port == 7888);
        assert!(url.path == "/test");
        assert!(url.query.is_empty());
        assert!(url.fragment.is_empty());
    }

    #[test]
    fn parse_url_with_fragment() {
        let uri = "https://a.test.com:7888/test#hallo";
        let url = Url::from(uri);

        println!("{}", uri);
        println!("{:#?}", url);

        assert!(url.scheme == Scheme::HTTPS);
        assert!(url.host == "a.test.com");
        assert!(url.port == 7888);
        assert!(url.path == "/test");
        assert!(url.query.is_empty());
        assert!(url.fragment == "hallo");
    }

    #[test]
    fn parse_url_with_query() {
        let uri = "https://a.test.com:7888/test?test=abc";
        let url = Url::from(uri);

        println!("{}", uri);
        println!("{:#?}", url);

        assert!(url.scheme == Scheme::HTTPS);
        assert!(url.host == "a.test.com");
        assert!(url.port == 7888);
        assert!(url.path == "/test");
        assert!(url.query.to_vec() == vec!(("test", "abc")));
        assert!(url.fragment.is_empty());
    }

    #[test]
    fn parse_url_with_queries() {
        let uri = "https://a.test.com:7888/test?test=abc&test1=well";
        let url = Url::from(uri);

        println!("{}", uri);
        println!("{:#?}", url);

        assert!(url.scheme == Scheme::HTTPS);
        assert!(url.host == "a.test.com");
        assert!(url.port == 7888);
        assert!(url.path == "/test");
        assert!(url.query.to_vec() == vec!(("test", "abc"), ("test1", "well")));
        assert!(url.fragment.is_empty());
    }

    #[test]
    fn parse_url_with_query_and_fragment() {
        let uri = "https://a.test.com:7888/test?test=abc#frag";
        let url = Url::from(uri);

        println!("{}", uri);
        println!("{:#?}", url);

        assert!(url.scheme == Scheme::HTTPS);
        assert!(url.host == "a.test.com");
        assert!(url.port == 7888);
        assert!(url.path == "/test");
        assert!(url.query.to_vec() == vec!(("test", "abc")));
        assert!(url.fragment == "frag");
    }

    #[test]
    fn parse_url_with_fragment_no_query() {
        let uri = "https://a.test.com:7888/test/abc#frag?test=abc";
        let url = Url::from(uri);

        println!("{}", uri);
        println!("{:#?}", url);

        assert!(url.scheme == Scheme::HTTPS);
        assert!(url.host == "a.test.com");
        assert!(url.port == 7888);
        assert!(url.path == "/test/abc");
        assert!(url.query.is_empty());
        assert!(url.fragment == "frag?test=abc");
    }
}
