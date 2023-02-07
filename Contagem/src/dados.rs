use strum_macros::EnumIter;

// Nomes dos partidos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Partido {}

// Número de assentos no parlamento
pub const ASSENTOS: f32 = 5.0;
