struct Apple  {
    fresh: bool,
    size: f64,
}

fn raw_get_average_price_of_apples(apples: Vec<Apple>) -> f64 {
    let mut total_price: f64 = 0.0;
    for  apple in apples.iter() {
        if (apple.fresh) {
            total_price += apple.size * 2.0;
        } else {
            total_price += apple.size * 0.5;
        }
    }
    total_price / apples.len() as f64
}

fn get_average_price_of_apples(apples: Vec<Apple>) -> f64 {
    let total_price = get_price_of_apples(&apples);
    let total_count = get_length_of_apples(&apples);
    total_price / total_count
}

fn get_price_of_apples(apples: &Vec<Apple>) -> f64 {
    let mut total_price: f64 = 0.0;
    for apple in apples.iter() {
        total_price += get_price_of_apple(apple);
    }
    total_price
}

fn get_price_of_apple(apple: &Apple) -> f64 {
    if (apple.fresh) {
        apple.size * 2.0
    } else {
        apple.size * 0.5
    }
}

fn get_length_of_apples(apples: &Vec<Apple>) -> f64 {
    apples.len() as f64
}


