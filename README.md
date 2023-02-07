
<h1 align="center">Neturn</h1>
<p align="center">
 <a href="https://github.com/Waydines/Neturn/blob/main/LICENSE"><img src="https://img.shields.io/badge/licen%C3%A7a-EUPL--1.2-blue?style=for-the-badge&labelColor=000"/></a>
</p>

Neturn é uma urna virtual que usa uma mistura bem única entre o método preferencial (mais especificamente, o de [voto único transferível](https://pt.wikipedia.org/wiki/Voto_%C3%BAnico_transfer%C3%ADvel)) e o [método proporcional](https://pt.wikipedia.org/wiki/Representa%C3%A7%C3%A3o_proporcional) e o repositório consiste de duas partes:
- **Contagem:** que é responsável pelo processo de [escrutínio](https://pt.wikipedia.org/wiki/Vota%C3%A7%C3%A3o#Escrut%C3%ADnio) da eleição
- **Bot:** que é responsável por registrar os votos das pessoas por meio de um Bot do Discord (atualmente incompleto)

# Como funciona?
Para o fortalecimento da democracia, é muito importante ser transparente no processo eleitoral para que não haja nenhuma [dúvida sobre a integridade da democracia](https://pt.wikipedia.org/wiki/Manifesta%C3%A7%C3%B5es_golpistas_no_Brasil_ap%C3%B3s_as_elei%C3%A7%C3%B5es_de_2022), por isso me esforcei para explicar todo o processo eleitoral por meio de comentários no código e fazendo uma explicação por texto aqui mesmo.

## Escrutínio / Contagem
A parte principal do processo e a que eu ([Cupertino](https://github.com/CupertinoWasTaken)) decidi programar primeiro, todo o texto a seguir explica o código presente na pasta [`Contagem`](https://github.com/Waydines/Neturn/tree/main/Contagem/src).

Depois das eleições, os votos de todos estarão salvos em um arquivo de forma completamente anônima, é importante ressaltar que os votos no nosso sistema eleitoral funcionam de um jeito completamente diferente do que estamos acostumados, pois eu me esforcei muito para encontrar métodos de eleição que representassem de forma mais aproximada possível a maioria e o método preferencial foi a melhor opção que encontrei, que funciona de forma completamente diferente do [sistema majoritário](https://pt.wikipedia.org/wiki/Sistema_majorit%C3%A1rio) que é usado por exemplo no Brasil e nos Estados Unidos.

No método preferencial, os eleitores listam os nomes dos partidos políticos em ordem de preferência, então imagine que nós temos uma eleição com três partidos, o Trabalhista, o Social-Democrata e o Neoliberal e eu odeio o Trabalhista e quero votar no Neoliberal, mas tenho certeza que o partido que eu gosto não tem chance de vencer o Trabalhista, então, por meio do método preferencial, eu posso votar no partido que eu gosto (o Neoliberal) e ao mesmo tempo posso votar no que acho menos pior e tem mais chance de vencer, que seria o Social-Democrata, fazendo com que eu me sinta muito mais livre de votar na opção que eu realmente quero invés de votar só no que eu acho menos pior e que tem mais chance de vencer as eleições, isso faz com que os resultados das eleições realmente representem a opinião da maioria.

Depois de ler o arquivo onde os votos estão salvos, o programa vai ler apenas a primeira preferência dos eleitores e contar este voto e é agora que o sistema proporcional entra em ação, será calculado quantos assentos cada partido recebeu baseado em seu número de votos, para isso teremos que dar uma olhada na matemática:
$$Q_e = \frac {V_v} {C}$$
Sendo $Q_e$ o Quociente Eleitoral, $V_v$ o número de pessoas que votaram e $C$ o número de assentos a serem preenchidos, depois de calcular o Quociente Eleitoral, é preciso calcular o Quociente Partidário ( $Q_p$ ) que é o número de assentos que o partido vai receber:
$$Q_p = \frac {V_p} {Q_e}$$
Sendo $Q_p$ o Quociente Partidário que é o número de assentos que o partido recebeu, $Q_e$ o Quociente Eleitoral e $V_p$ o número de votos que o partido recebeu.
Se a soma de todos os $Q_p$ for menor que o $C$ (número de assentos), o partido com menor $V_p$ (quantidade de votos) vai ser eliminado da eleição e todos que votaram nesse partido como primeira preferência terão seu voto transferido para sua próxima preferência, se a pessoa não tiver uma próxima preferência seu voto não será transferido à nenhum partido.

E todo esse processo se repete até que a soma de todos os $Q_p$ seja igual a $C$, quando a soma finalmente for igual a $C$, o programa libera os resultados da eleição e o processo acaba.

## Bot
O Bot do Discord só serve como um jeito fácil dos eleitores registrarem seu voto, essa parte do código ainda não está completa e nem se quer presente no repositório, logo não tem explicação do código, apenas como eu quero que funcione, tudo a seguir está sujeito a mudanças extremas.

O Bot por meio de um comando vai gerar uma mensagem invisível para todos exceto o próprio eleitor, nesta mensagem, o eleitor vai digitar os partidos que quer votar em ordem de preferência, depois de algumas horas desde sua inicialização, a eleição será encerrada e o bot vai salvar os votos de todos os eleitores em um arquivo, sem citar o responsável pelos votos em nenhum momento. Para evitar que uma pessoa vote duas vezes, o Bot vai salvar o ID de usuário do eleitor, criptografado com [criptografia irreversível](https://pt.wikipedia.org/wiki/Fun%C3%A7%C3%A3o_hash) (mais especificamente, [Argon2id](https://en.wikipedia.org/wiki/Argon2)) na memória RAM, fazendo impossível que o nome do eleitor seja acessível por alguém ou pelo próprio software e só sirva para impedir votos duplos. 

# Resumo da licença
## Permissões
- O software e derivados dele podem ser (re)distribuídos para uso comercial
- O software pode ser redistribuído por qualquer um e pode ser modificado por qualquer um
- Os contribuídores tem total direitos de patente do projeto
- O software pode ser usado de forma privada por quem quiser

## Condições
- Sempre que redistribuído o software tem que continuar com seu código público para todos
- Uma cópia da licença e dos direitos autorais originais devem estar inclusos na redistribuição
- Todos que tem acesso/usam o software, tem que também ter o direito de receber uma cópia do código-fonte
- Toda redistribuição deve ser pública sobre a mesma licença
- Qualquer modificações feitas ao software original por meio de recomendações ou redistribuições devem ser documentadas

## Limitações
- Ninguém tem garantia de direito de marca sobre o software
- Ninguém é responsável por danos que possam ser causados pelo software
- A licença não garante qualquer tipo de garantia à ninguém