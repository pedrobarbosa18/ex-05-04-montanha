// Exercício 03 — Remoção condicional sem .retain()
//
// Complexidade:
//   Abordagem 1 — novo vetor:
//     O(n): percorremos uma vez e copiamos apenas os ímpares.
//
//   Abordagem 2 — in-place com swap_remove ou remoção direta:
//     O(n²): cada remove(i) desloca todos os elementos seguintes.
//     Usamos índice reverso para minimizar deslocamentos, mas o pior caso continua O(n²).

fn remover_pares_novo_vec(vec: &Vec<i32>) -> Vec<i32> {
    // O(n) — cria um novo vetor apenas com ímpares
    let mut resultado = Vec::new();
    for &x in vec {
        if x % 2 != 0 {
            resultado.push(x);
        }
    }
    resultado
}

fn remover_pares_inplace(vec: &mut Vec<i32>) {
    // O(n²) no pior caso — remove(i) desloca elementos
    // Iteramos de trás para frente para não pular elementos após a remoção
    let mut i = vec.len();
    while i > 0 {
        i -= 1;
        if vec[i] % 2 == 0 {
            vec.remove(i); // O(n) por remoção
        }
    }
}

fn main() {
    let original = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original: {:?}", original);

    // Abordagem 1 — novo vetor O(n)
    let filtrado = remover_pares_novo_vec(&original);
    println!("Novo vetor (O(n)):   {:?}", filtrado);

    // Abordagem 2 — in-place O(n²)
    let mut inplace = original.clone();
    remover_pares_inplace(&mut inplace);
    println!("In-place  (O(n²)):  {:?}", inplace);
}
