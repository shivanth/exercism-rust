use std::collections::VecDeque;
pub fn lsp(series: &str, count: u32) -> Result<u32, &str> {
    println!("Starting test.... with sereis {}",series);
    if count == 0 {
        return Ok(1);
    }

    let mut buffer: VecDeque<u32> = VecDeque::new();
    let mut product = 0;
    let mut product_max = 0;
    let mut n =0;
    for c in series.chars() {
        if product_max < product && buffer.len() == (count as usize){
            product_max = product;
        }
        n += 1;
        if !c.is_digit(10) {
            return Err("Not a valid input");
        }
        let i = c.to_digit(10).unwrap();
        println!("{}",i);
        if i ==0 {

            product = 0;
            buffer.clear();
            continue;
        }
        if buffer.len() < (count as usize) {
            buffer.push_back(i);
            if product ==0 {
                product = i;
            }
            else{
                product *= i;
            }

        } else {
            if product_max < product {
                product_max = product;
            }
            buffer.push_back(i);
            let a = buffer.pop_front().unwrap();


            product_max = match (product * i) / a {
                x if x > product_max => x,
                _ => product_max,
            };
            product = (product * i)/a;
        }
        println!("product:{}",product);
        println!("Product_max:{}",product_max);
        println!("buffer:{:?}",buffer);

    }
    if n < count {
        return Err("Not enough numbers");
    }

    if product_max < product && buffer.len() == (count as usize){
        product_max = product;
    }

    Ok(product_max)
}
