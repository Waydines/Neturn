use crate::dados::Partido;
use std::collections::HashSet;

/* A cédula é quem representa o seu voto
   ela segura um vetor, que é uma lista de valores, no caso, de partidos
   essa lista são os partidos posicionados na sua ordem de preferência */
#[derive(Clone, PartialEq, Eq)]
pub struct Cédula {
    pub vec: Vec<Partido>,
}

impl Cédula {
    // Função matriz, serve para poder criar uma cédula
    pub fn new(preferências: &[Partido]) -> Self {
        /* Um set é uma lista sem valores repetidos
           por exemplo, vamos dizer que você tem uma lista de números e nesta lista o número 2 aparece mais de uma vez
           em um set isso não seria possível */
        let mut set = HashSet::new();

        /* Testa se você colocou no mínimo 3 partidos na sua cédula
           ou se você colocou algum partido repetido
           se esse for o caso, o programa vai enviar um erro */
        if preferências.len() < 3 || !preferências.iter().all(|x| set.insert(*x)) {
            panic!()
        }

        // Se tudo estiver certinho, gera sua cédula
        Self {
            vec: preferências.into(),
        }
    }
}

/* Macro que facilita minha vida
   serve pra gerar cédulas de um jeito fácil */
#[macro_export]
macro_rules! céd {
    ($($preferências:expr),+ $(,)?) => (
        Cédula::new(&[$($preferências),+])
    )
}
