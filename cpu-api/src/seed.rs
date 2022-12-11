use rand::random;

pub fn seed() -> i32 {
    rand::random()
}

pub fn randint(low: i32, high: i32) -> i32 {
    low + (random::<i32>() * (high - low + 1))
}

pub fn choice<T>(list: &Vec<T>) -> &T {
    //  pick a random element from a list
    let index = randint(0, list.len() as i32 - 1) as usize;
    &list[index]
}