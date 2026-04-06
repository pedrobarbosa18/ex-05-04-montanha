// Exercício 02 — Contador de ocorrências
//
// Complexidade:
//   - Iteração pelo Vec<char>: O(n)
//   - Busca linear no vetor de contagens: O(k) por elemento, onde k = letras únicas
//   - Total: O(n * k) — aceitável para alfabetos pequenos

fn contar_ocorrencias(letras: &Vec<char>) -> Vec<(char, usize)> {
    let mut contagens: Vec<(char, usize)> = Vec::new();

    for &c in letras {
        if c == ' ' {
            continue;
        }

        let c = c.to_lowercase().next().unwrap();

        // Busca linear se a letra já existe — O(k)
        let mut encontrado = false;
        for par in &mut contagens {
            if par.0 == c {
                par.1 += 1;
                encontrado = true;
                break;
            }
        }

        if !encontrado {
            contagens.push((c, 1));
        }
    }

    contagens
}

fn main() {
    let frase = "estruturas de dados";
    let letras: Vec<char> = frase.chars().collect();

    println!("Frase: \"{}\"", frase);
    println!("Contagem de letras:");

    let mut resultado = contar_ocorrencias(&letras);
    resultado.sort_by_key(|par| par.0);

    for (letra, qtd) in resultado {
        println!("  '{}' => {}", letra, qtd);
    }
}
