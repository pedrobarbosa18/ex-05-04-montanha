// Exercício 01 — Inversão com Vec
//
// Complexidade:
//   - Inverter usando push/pop: O(n)
//     Passamos por cada elemento uma vez para empilhar e uma vez para desempilhar.

fn inverter(vec: Vec<i32>) -> Vec<i32> {
    let mut pilha: Vec<i32> = Vec::new();

    // Empilha todos os elementos — O(n)
    for x in vec {
        pilha.push(x);
    }

    let mut resultado: Vec<i32> = Vec::new();

    // Desempilha na ordem inversa — O(n)
    while let Some(topo) = pilha.pop() {
        resultado.push(topo);
    }

    resultado
}

fn main() {
    let original = vec![1, 2, 3, 4, 5];
    println!("Original:  {:?}", original);

    let invertido = inverter(original);
    println!("Invertido: {:?}", invertido);
}
