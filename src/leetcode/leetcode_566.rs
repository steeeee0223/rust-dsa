fn fill(mut mat: Vec<Vec<i32>>, c: i32, count: i32, entry: i32) -> (Vec<Vec<i32>>, i32) {
    if count % c == 0 {
        mat.push(vec![]);
    }
    mat.last_mut().unwrap().push(entry);

    println!("mat={:?}", mat);
    return (mat, count + 1);
}

pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    if (mat.len() * mat[0].len()) as i32 != r * c {
        return mat;
    }

    let res = mat
        .iter()
        .flatten()
        .fold((vec![], 0), |(mat, count), &entry| {
            fill(mat, c, count, entry)
        });

    res.0
}

pub fn matrix_reshape2(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    if (mat.len() * mat[0].len()) as i32 != r * c {
        return mat;
    }
    mat.iter()
        .flat_map(|vec| vec.iter().copied())
        .collect::<Vec<i32>>()
        .chunks(c as usize)
        .map(|arr| arr.to_vec())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn to_2d_vec<T: Clone>(mat: &[&[T]]) -> Vec<Vec<T>> {
        mat.iter().map(|&arr| arr.to_vec()).collect()
    }

    #[test_case(&[&[1,2],&[3,4]], 1, 4,
         &[&[1,2,3,4]]; "case 1")]
    #[test_case(&[&[1,2],&[3,4]], 2, 4,
            &[&[1,2],&[3,4]]; "case 2")]
    fn test_matrix_reshape(mat: &[&[i32]], r: i32, c: i32, expected: &[&[i32]]) {
        let result = matrix_reshape(to_2d_vec(mat), r, c);
        assert_eq!(result, to_2d_vec(expected));
    }
}
