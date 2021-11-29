// 题目链接：https://exercism.org/tracks/rust/exercises/sublist

#[derive(Debug, PartialEq)]
enum SUBLIST_COMPARISION {
    EQUAL,
    SUBLIST,
    SUPERLIST,
    UNEQUAL,
}

// 理解错题意了
fn sublist_wrong(a: &Vec<i32>, b: &Vec<i32>) -> SUBLIST_COMPARISION {
    let mut a_in_b: bool = true;
    for element in a.iter() {
        if b.iter().any(|&x| x == *element) {
            continue;
        } else {
            a_in_b = false;
            break;
        }
    }

    let mut b_in_a: bool = true;
    for element in b.iter() {
        if a.iter().any(|&x| x == *element) {
            continue;
        } else {
            b_in_a = false;
            break;
        }
    }

    if a_in_b && b_in_a {
        return SUBLIST_COMPARISION::EQUAL;
    } else if a_in_b && !b_in_a {
        return SUBLIST_COMPARISION::SUBLIST;
    } else if !a_in_b && b_in_a {
        return SUBLIST_COMPARISION::SUPERLIST;
    } else {
        return SUBLIST_COMPARISION::UNEQUAL;
    }
}

// 正确做法
// windows函数牛的：https://doc.rust-lang.org/std/primitive.slice.html#method.windows
fn sublist(a: &Vec<i32>, b: &Vec<i32>) -> SUBLIST_COMPARISION {
    use SUBLIST_COMPARISION::*;
    let a_len = a.len();
    let b_len = b.len();

    if a_len > b_len {
        if a.windows(b_len).any(|x| x == b) {
            return SUPERLIST;
        } else {
            return UNEQUAL;
        }
    } else if a_len < b_len {
        if b.windows(a_len).any(|x| x == a) {
            return SUBLIST;
        } else {
            return UNEQUAL;
        }
    } else {
        if a == b {
            return EQUAL;
        } else {
            return UNEQUAL;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sublist_test() {
        let v1 = vec![1,2,3];
        let v2 = vec![1,2,3,4,5];
        let v3 = vec![3,4,5];
        let v4 = vec![3,4];
        let v5 = vec![1,2,4];
        let v6 = vec![2,3,4];

        assert_eq!(sublist(&v1, &v2), SUBLIST_COMPARISION::SUBLIST);
        assert_eq!(sublist(&v3, &v2), SUBLIST_COMPARISION::SUBLIST);
        assert_eq!(sublist(&v4, &v2), SUBLIST_COMPARISION::SUBLIST);
        assert_eq!(sublist(&v1, &v1), SUBLIST_COMPARISION::EQUAL);
        assert_eq!(sublist(&v2, &v6), SUBLIST_COMPARISION::SUPERLIST);
        assert_eq!(sublist(&v5, &v2), SUBLIST_COMPARISION::UNEQUAL);
    }
}

// 这个solution也太妙了：https://exercism.org/tracks/rust/exercises/sublist/solutions/alireza4050