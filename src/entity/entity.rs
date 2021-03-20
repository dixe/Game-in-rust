#[derive(Debug, Clone)]
pub enum EntityType {
    Simple(usize),
    Complex(ComplexEntity)
}

#[derive(Debug, Clone)]
pub struct ComplexEntity {
    pub id : usize,
    pub sub_entities: Vec<usize>
}
