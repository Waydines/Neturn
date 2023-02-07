use crate::dados::Partido;
use crate::dados::ASSENTOS;
use crate::Cédula;
use strum::IntoEnumIterator;

/* Excrutínio é o termo técnico pra contagem de votos
   Recebe os votos de todo mundo e transforma em uma lista de quantos assentos cada partido recebeu */
pub fn escrutínio(mut cédulas: Vec<Cédula>) -> Vec<(Partido, u8)> {
    // Número de votos que cada partido recebeu
    let mut votos = vec![];
    // Quantos assentos no parlamento já tem dono
    let mut reservas = 0.0;
    // Número de assentos que cada partido recebeu
    let mut assentos = vec![];
    /* Quantos votos um partido precisa para ganhar um assento
       https://pt.wikipedia.org/wiki/Quociente_eleitoral */
    let quociente_e = cédulas.len() as f32 / ASSENTOS;

    // Inicia a lista de votos com cada partido tendo 0 votos
    for partido in Partido::iter() {
        /* O u8 do lado do zero define que o número é 8 bits e não muda nada no número de votos
           como Waydines é pequena demais pra um partido receber mais de 256 votos, o número ser 8 bits não afeta nada
           resumidamente só serve pro programa usar menos RAM
           https://rust-br.github.io/rust-book-pt-br/ch03-02-data-types.html#tipos-inteiros */
        votos.push((partido, 0u8))
    }

    // Vê a preferência número 1 de cada cédula e adiciona um voto para o partido dessa posição
    for cédula in &cédulas {
        // Encontra a preferência número 1
        let &partido = cédula.vec.first().unwrap();

        votos[partido as usize].1 += 1; // Adiciona o voto
    }

    // Calcula quantos assentos cada partido recebeu baseado no número de votos
    for (partido, voto) in &votos {
        /* Calcula o quociente partidário, que é o número de assentos que o partido recebeu
           Número de votos recebidos pelo partido dividido pelo quociente eleitoral */
        let quociente_p = *voto as f32 / quociente_e;

        // Reserva os assentos pro partido
        assentos.push((*partido, quociente_p.round() as u8));
        reservas += quociente_p.round();
    }

    // Se ainda tiverem assentos vazios o sistema começa a distribuir os votos baseado na preferência das cédulas
    while reservas < ASSENTOS {
        // Faz a lista ficar em ordem crescente de votos
        votos.sort_by(|(_, a), (_, b)| a.cmp(b));

        // Deleta o menor partido e limpa as reservas para recalcular
        let (menor_partido, _) = votos[0];
        votos.remove(0);
        assentos.clear();
        reservas = 0.0;

        // Deleta o menor partido das cédulas e da o voto pra preferência mais próxima da pessoa
        for cédula in &mut cédulas {
            /* Se o partido eliminado não estiver na lista de preferência da cédula, pula pra próxima
               se a cédula não tiver nenhum partido válido faz o mesmo
               ou se o partido eliminado não for a preferência número 1 da cédula faz a mesma coisa também */
            if !cédula.vec.contains(&menor_partido)
                || cédula.vec.is_empty()
                || cédula.vec.first() != Some(&menor_partido)
            {
                continue;
            }

            // Remove o partido da lista de preferências
            cédula.vec.remove(0);

            // Distribui o voto pra nova preferência número 1, que no caso seria a preferência mais próxima
            let pos = votos
                .iter()
                .position(|x| x.0 == *cédula.vec.first().unwrap())
                .unwrap();
            votos[pos].1 += 1;

            break;
        }

        /* Recalcula o número de assentos que cada partido recebeu
           se ainda tiverem assentos vazios, repete o processo inteiro até que todos os assentos estejam reservados */
        for (partido, voto) in &votos {
            let quociente_p = *voto as f32 / quociente_e;

            assentos.push((*partido, quociente_p.round() as u8));
            reservas += quociente_p.round();
        }
    }

    // Depois de tudo, libera os resultados
    assentos
}
