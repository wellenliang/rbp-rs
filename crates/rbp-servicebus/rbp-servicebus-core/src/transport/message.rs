use crate::message::header::Header;


/// the message coming from the transport
/// 来自传输层的原始消息
#[derive(Debug, Clone)]
pub struct IncomingMessage {
    id: String,
    native_id: String,
    header: Header,
    body: Vec<u8>
}

impl IncomingMessage {
    pub fn new(id: String, native_id: String, header: Header, body: Vec<u8>) -> Self {
        Self {
            id,
            native_id,
            header,
            body
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn native_id(&self) -> &str {
        &self.native_id
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    pub fn body(&self) -> Vec<u8> {
        self.body.clone()
    }
}

#[derive(Debug, Clone)]
pub struct OutgoingMessage {
    id: String,
    header: Header,
    body: Vec<u8>,
}

impl OutgoingMessage {
    pub fn new(id: String, header: Header, body: Vec<u8>) -> OutgoingMessage {
        Self {  id, header, body }
    }

    pub fn id(&self) -> &str { &self.id }


    pub fn header(&self) -> &Header { &self.header }

    pub fn body(&self) -> Vec<u8> {
        self.body.clone()
    }

    pub fn with_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }
}
