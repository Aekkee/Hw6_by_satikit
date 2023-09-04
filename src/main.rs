fn main() {
    println!("Hello, world!");
}

fn min_max_avg(v: &[i64]) -> (i64,i64,f64) {
    if v.is_empty() {
        return (0,0,0.0)
    }
    let mut least = 0;
    let mut iold = v[0];
    for i in v {
        least = *i.min(&iold);
        iold = least;
    }
    let mut most = 0;
    let mut iold = v[0];
    for i in v {
        most = *i.max(&iold);
        iold = most;
    }
    let mut total = 0;
    for i in v {
        total += *i
    }
    (least, most, (total as f64 / v.len() as f64))
}

#[test]
fn test_min_max_avg() {
    assert_eq!(min_max_avg(&[]), (0,0,0.0));
    assert_eq!(min_max_avg(&[1,1,1,1]), (1,1,1.0));
    assert_eq!(min_max_avg(&[2, 11, 3, 4, 7]), (2,11,5.4));
    assert_eq!(min_max_avg(&[15, 40, 20, 10, -10,0]), (-10, 40, 12.5));
}

fn cal_partial_sums(v: &[i64]) -> Vec<i64> {
    if v.is_empty() {
        return Vec::new()
    }
    let mut result: Vec<i64> = Vec::new();
    let mut iold: i64 = 0;
    for i in v {
        iold += *i;
        result.push(iold);
    }
    return result
}

#[test]
fn test_cal_partial_sums() {
    assert_eq!(cal_partial_sums(&[]), []);
    assert_eq!(cal_partial_sums(&[1,1,1,1]), [1,2,3,4]);
    assert_eq!(cal_partial_sums(&[2, 11, 3, 4, 7]), [2, 13, 16, 20, 27]);
    assert_eq!(cal_partial_sums(&[15, 40, 20, 10, -10,0]), [15, 55, 75, 85, 75, 75]);
}

fn pack_number_tuples3(a: &[i64], b: &[i64], c: &[i64]) -> Vec<(i64,i64,i64)> {
    if a.is_empty() && b.is_empty() && c.is_empty() {
        return Vec::new()
    }
    let mut result = Vec::new();
    let mut count: usize = 0;
    while a.get(count) != None || b.get(count) != None || c.get(count) != None {
        let (mut x, mut y, mut z) = (0,0,0);
        if a.get(count) != None {
            x = a[count];
        }
        if b.get(count) != None {
            y = b[count];
        }
        if c.get(count) != None {
            z = c[count];
        }
        result.push((x,y,z));
        count += 1
    }
    result
}

#[test]
fn test_pack_number_tuples3() {
    assert_eq!(pack_number_tuples3(&[], &[], &[]), []);
    assert_eq!(pack_number_tuples3(&[1, 2], &[4, 3], &[5]),  [(1, 4, 5), (2, 3, 0)]);
    assert_eq!(pack_number_tuples3(&[1], &[4], &[5]),  [(1, 4, 5)]);
    assert_eq!(pack_number_tuples3(&[1,2,3], &[4], &[5]), [(1, 4, 5), (2, 0, 0), (3, 0, 0)]);
    assert_eq!(pack_number_tuples3(&[1], &[2,3,4], &[5]), [(1, 2, 5), (0, 3, 0), (0, 4, 0)]);
    assert_eq!(pack_number_tuples3(&[1], &[2,7], &[3, 4, 5]), [(1, 2, 3), (0, 7, 4), (0, 0, 5)]);
}

fn pack_number_tuples3_v2(a: &[i64], b: &[i64], c: &[i64]) -> Vec<(i64,i64,i64)> {
    if a.is_empty() && b.is_empty() && c.is_empty() {
        return Vec::new()
    }
    let mut result = Vec::new();
    let mut count: usize = 0;
    while a.get(count) != None && b.get(count) != None && c.get(count) != None {
        let (mut x, mut y, mut z) = (0,0,0);
        if a.get(count) != None {
            x = a[count];
        }
        if b.get(count) != None {
            y = b[count];
        }
        if c.get(count) != None {
            z = c[count];
        }
        result.push((x,y,z));
        count += 1
    }
    result
}

#[test]
fn test_pack_number_tuples3_v2() {
    assert_eq!(pack_number_tuples3_v2(&[], &[], &[]), []);
    assert_eq!(pack_number_tuples3_v2(&[1, 2], &[4, 3], &[5]),  [(1, 4, 5)]);
    assert_eq!(pack_number_tuples3_v2(&[1, 2], &[4, 3], &[]),  []);
    assert_eq!(pack_number_tuples3_v2(&[1, 2], &[4, 3], &[5, 9, 0, 8]),  [(1, 4, 5), (2, 3, 9)]);
    assert_eq!(pack_number_tuples3_v2(&[1, 2, 9], &[4, 3, 5], &[5, 9, 0, 8]),  [(1, 4, 5), (2, 3, 9), (9, 5, 0)]);
}

fn unpack_number_tuples(v: &[(i64,i64)]) -> (Vec<i64>,Vec<i64>) {
    if v.is_empty() {
        return (Vec::new(),Vec::new())
    }
    let mut v1: Vec<i64> = Vec::new();
    let mut v2: Vec<i64> = Vec::new();
    for i in v {
        v1.push(i.0);
        v2.push(i.1);
    }
    (v1,v2)
}

#[test]
fn test_unpack_number_tuples() {
    assert_eq!(unpack_number_tuples(&[]), (vec![], vec![]));
    assert_eq!(unpack_number_tuples(&[(1, 4), (3, 2), (2,1)]), (vec![1, 3, 2], vec![4, 2, 1]));
    assert_eq!(unpack_number_tuples(&[(1, 4), (3, 2), (2,1), (6,7)]), (vec![1, 3, 2, 6], vec![4, 2, 1, 7]));
}

fn unpack_number_tuples3(v: &[(i64, i64, i64)]) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    if v.is_empty() {
        return ([].to_vec(),[].to_vec(),[].to_vec())
    }
    let mut v1: Vec<i64> = Vec::new();
    let mut v2: Vec<i64> = Vec::new();
    let mut v3: Vec<i64> = Vec::new();
    for i in v {
        v1.push(i.0);
        v2.push(i.1);
        v3.push(i.2);
    }
    (v1,v2,v3)
}

#[test]
fn test_unpack_number_tuples3() {
    assert_eq!(unpack_number_tuples3(&[]), (vec![], vec![], vec![]));
    assert_eq!(unpack_number_tuples3(&[(1, 4, 5)]), (vec![1], vec![4], vec![5]));
    assert_eq!(unpack_number_tuples3(&[(1, 4, 5), (2, 2, 1)]), (vec![1, 2], vec![4, 2], vec![5, 1]));
    assert_eq!(unpack_number_tuples3(&[(1, 4, 5), (2, 2, 1), (0, 0, 0)]), (vec![1, 2, 0], vec![4, 2, 0], vec![5, 1, 0]));
}