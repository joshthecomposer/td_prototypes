fn main() {
    let matrix = Vec::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    let rows = matrix.len();
    let columns = matrix[0].len();
    dbg!(rows, columns);

    let mut linear_vec: Vec<i32> = vec![];
    //process the array into one flattened array.
    for row in &matrix {
        for &item in row {
            linear_vec.push(item);
        }
    }

    dbg!(&linear_vec);
    //reprocess the array back into a matrix, using the stored rows and columns values.
    let rematrixed: Vec<Vec<i32>> = linear_vec
        .chunks(columns)
        .map(|chunk| chunk.to_vec())
        .collect();

    dbg!(&rematrixed);
}
