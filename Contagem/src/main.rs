use cedula::Cédula;
use contagem::escrutínio;
use dados::Partido::*;

mod cedula;
mod contagem;
mod dados;

// Simplesmente roda o código
fn main() {
    let resultados = escrutínio(vec![]);

    for (partido, votos) in resultados {
        println!("{partido:?} ({votos})")
    }
}
