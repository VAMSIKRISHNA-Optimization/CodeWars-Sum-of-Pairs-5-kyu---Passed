// pub fn sum_pairs(ints: &[i32], s: i32) -> Option<(i32, i32)> 
// {
//     let combos: Vec<((i32,usize), (i32,usize))> = ints.iter().enumerate()
//                                                     .flat_map(|(i, &v1)| 
//                                                     {
//                                                         ints[i + 1..].iter().enumerate() 
//                                                             .map(move |(j, &v2)| 
//                                                             {
//                                                                 let actual_j = j + i + 1; 
//                                                                 ((v1, i), (v2, actual_j))
//                                                             })
//                                                     })
//                                                     .filter(|((v1, _), (v2, _))| v1 + v2 == s)
//                                                     .collect();
    
//     let min_pair = combos.iter()
//                   .min_by_key(|&((_, _), (_, idx2))| idx2);
    
//     //println!("{:?}", min_pair.unwrap().0);            
//     if min_pair.is_some()
//     {
//         Some( (min_pair.unwrap().0.0, min_pair.unwrap().1.0 ) )
//     }
//     else
//     {
//         None
//     }
    
//     // None
    
// }


use std::collections::HashSet;
pub fn sum_pairs(ints: &[i32], s: i32) -> Option<(i32, i32)> 
{
    let mut seen = HashSet::new();

    for &num in ints {
        let complement = s - num;
        if seen.contains(&complement) {
            // Found the pair! Because we are iterating from left to right,
            // the first pair we find is guaranteed to have the smallest 
            // second index.
            return Some((complement, num));
        }
        seen.insert(num);
    }

    None
    
}
