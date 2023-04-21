#![allow(dead_code)]

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}

fn ex1() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec2(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}

fn ex2() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec2(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn ex3() {
    let mut vec1 = fill_vec3();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec3() -> Vec<i32> {
    let mut vec = Vec::new();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn ex4() {
    let mut x = 100;

    let y = &mut x;
    *y += 100;

    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}

fn ex5() {
    let data = "Rust is great!".to_string();

    let ch = get_char(&data);
    println!("The last character is {}", ch);

    string_uppercase(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        ex1();
        ex2();
        ex3();
        ex4();
    }

    #[test]
    fn test_ex5() {
        ex5();
    }
}
