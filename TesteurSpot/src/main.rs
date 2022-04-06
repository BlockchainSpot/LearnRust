// Test

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut items = Vec::new();

    for _value in 0..10 {
        items.push(rng.gen::<u8>());
    }

    let length = items.len() -1;

    triRapide( &mut items, 0, length);

    for value in items.iter() {
        println!("{}", value);
    }
}

fn triRapide(tab: &mut Vec<u8>, startIndex: usize, endIndex: usize) {

    if startIndex < endIndex {

        let mut rng = rand::thread_rng();

        let mut pivotIndex = rng.gen_range(startIndex..endIndex);

        pivotIndex = partitionner(tab, startIndex, endIndex, pivotIndex);

        if pivotIndex > startIndex {
            triRapide(tab, startIndex, pivotIndex - 1);
        }
        if pivotIndex < endIndex {
            triRapide(tab, pivotIndex + 1, endIndex);
        }
    }
}

fn partitionner(tab: &mut Vec<u8>, startIndex:usize, endIndex:usize, pivotIndex:usize) -> usize {

    intervertir(tab, pivotIndex, endIndex);

    let mut finalPivotIndex = startIndex;

    for index in startIndex..(endIndex) {

        if tab[index] <= tab[endIndex] {

            intervertir(tab, index , finalPivotIndex);

            finalPivotIndex += 1 ;
        }

    }

    intervertir( tab, finalPivotIndex, endIndex);

    return finalPivotIndex

}

fn intervertir(tab: &mut Vec<u8>, index1: usize, index2: usize) {
    let temp = tab[index1];
    tab[index1] = tab[index2];
    tab[index2] = temp;
}

