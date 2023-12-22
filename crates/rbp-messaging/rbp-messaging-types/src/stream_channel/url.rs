use url::Url;

/// pulsar+ssl://localhost:6750
/// pulsar://localhost:6750
/// kafka://localhost:6750
/// kafka+ssl://localhost:6750
/// rocketmq://localhost:6750
/// rocketmq+ssl://localhost:6750
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamChannelUrl {
    nodes: Vec<Url>,
}

impl StreamChannelUrl {
    ///
    pub fn new(nodes: Vec<Url>) -> Self {
        Self { nodes }
    }

    /// current message bus contain given url
    pub fn contain_url(&self, url: &Url) -> bool {
        for inner_url in self.nodes.iter() {
            if inner_url == url {
                return true;
            }
        }

        false
    }
}
