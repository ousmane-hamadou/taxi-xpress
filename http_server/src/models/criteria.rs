use uuid::Uuid;

pub struct Criteria<'r> {
    pub origin: &'r Uuid,
    pub destination: &'r Uuid,
}
