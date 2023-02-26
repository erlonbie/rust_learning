fn main() {
    println!("Hello, world!");

    let mut arr = [2, 1, 4, 5, 3];
    quicksort(&mut arr);
    println!("{:?}", arr);
}

fn quicksort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let pivot = partition(arr);
        quicksort(&mut arr[0..pivot]);
        quicksort(&mut arr[pivot + 1..]);
    }
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr.len() - 1;
    let mut i = 0;

    for j in 0..pivot {
        if arr[j] < arr[pivot] {
            // se o elemento é menor que o pivô, troque-o com o elemento da posição "i" e incremente a posição "i" em 1.
            arr.swap(i, j); // isso significa que todos os elementos à esquerda de "i" são menores que o pivô.
            i += 1; // e todos os elementos à direita são maiores que o pivô.
        } // isso garante que a partição está correta no final da iteração.
    }

    arr.swap(i, pivot);

    i // retorna a posição do pivô depois de todas as trocas de elementos terem sido feitas

    // (ou seja, a partir daqui, os elementos à esquerda serão menores que o pivô e os à direita serão maiores)

    // assim, podemos usar essa posição para dividir recursivamente nosso vetor em part
}
