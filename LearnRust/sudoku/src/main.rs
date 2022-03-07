
// Partie 1 : afficher une grille de Sudoku
fn print_grid(grid: &[[u8; 9]; 9]) {
    println!("");
    // parcourir à l'intérieur du tableauu
    for l in 0..9 {
        if l !=0 && l % 3 == 0 {
                println!("-------+-------+-------");
            }
        for c in 0..9 {
            if c !=0 && c % 3 == 0 {
                print!(" |");
            }
            match grid[l][c] {
                0 => print!("  "),
                n => print!(" {}", n)
            }  
        }
        println!();
    }
}

// partie 2 : tester une proposition pour une case
fn is_possible(grid: &[[u8; 9]; 9], row:usize, col:usize, num: u8) -> bool {
    for i in 0..9 {
        if grid[row][i]==num {
            return false;
        }
    }
    for i in 0..9 {
        if grid[i][col]==num {
            return false;
        }
    }
    let x0 = (col / 3) * 3;
    let y0 = (row / 3) * 3;
    for y in 0..3 {
        for x in 0..3 {
            if grid[y0 + y][x0 + x] == num {
                return false;
            }
        }
    }
    return true;
}

// partie 3 : explorer les grilles possibles et retourner les solutions
fn solve(grid: &[[u8; 9]; 9]) -> Vec<[[u8; 9]; 9]> {
    // initialisation du tableau des solutions trouvées
    let mut sols_founded: Vec<[[u8; 9]; 9]> = Vec::new();
    
    for y in 0..9 {
        for x in 0..9 {
            if grid[y][x] == 0 {
                for i in 1..=9 {
                    if is_possible(grid, y, x, i) {
                        let mut new_grid = grid.clone();
                        new_grid[y][x] = i;
                        
                        let sols = solve(&new_grid);
                        sols_founded.extend(sols);
                    }
                }
                return sols_founded;
            }
        }
    }
    sols_founded.push(*grid);
    return sols_founded;
}

fn main() {
    let grid: [[u8; 9]; 9] = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    print_grid(&grid);

    // for i in 1..=9 {
    //     if is_possible(&grid, 0, 2, i){
    //         print!(" {}", i);
    //     }
    // }

    let results = solve(&grid);    
    
    for r in &results { 
        print_grid(&r); 
    }

    println!("{} solution(s).", results.len());
}
