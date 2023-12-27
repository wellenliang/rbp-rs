use ulid::{Generator, Ulid};

use crate::{IdGeneratorMut, IdGeneratorError};

pub struct UlidGenerator(Generator);

impl IdGeneratorMut<Ulid> for UlidGenerator {
    type Error = IdGeneratorError;

    fn generate(&mut self) -> Result<Ulid, Self::Error> {
        let result = self.0.generate();

        match result {
            Ok(ulid) => Ok(ulid),
            Err(_) => Err(IdGeneratorError::UlidError),
        }
    }
}
