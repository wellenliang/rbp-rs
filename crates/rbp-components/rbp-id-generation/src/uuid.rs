use uuid::{Builder, Uuid};

use crate::{IdGenerator, IdGeneratorError};

#[derive(Debug)]
pub struct UUIDGenerator {
    builder: Builder,
}

impl UUIDGenerator {
    pub fn new(builder: Builder) -> Self {
        Self { builder }
    }

    pub fn builder(&self) -> &Builder {
        &self.builder
    }

    pub fn builder_mut(&mut self) -> &mut Builder {
        &mut self.builder
    }
}

impl IdGenerator<Uuid> for UUIDGenerator {
    type Error = IdGeneratorError;

    fn generate(self) -> Result<Uuid, Self::Error> {
        let builder = self.builder;
        let uuid = builder.into_uuid();
        Ok(uuid)
    }
}
