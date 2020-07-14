#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn get_variants_with_no_repetition(x: i64, y: i64) -> Vec<i64> {
    let mut result = Vec::<i64>::new();
    for n0 in x..y {
        for n1 in x..y {
            if n1 != n0 {
                for n2 in x..y {
                    if n2 != n0 && n2 != n1 {
                        for n3 in x..y {
                            if n3 != n0 && n3 != n1 && n3 != n2{
                                for n4 in x..y {
                                    if n4 != n0 && n4 != n1 && n4 != n2 && n4 != n3 {
                                        result.push(n0 + n1 * 10 + n2 * 100 + n3 * 1000 + n4 * 10000);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

