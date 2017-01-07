pub struct PascalsTriangle{
    n:u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle{n:row_count}
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result:Vec<Vec<u32>> = Vec::new();
        for i in 0..self.n {
            let mut row :Vec<u32> = Vec::new();
            for j in 0..i+1 {
                row.push(factorial(i)/(factorial(j)*factorial(i - j)));
            }
            result.push(row);
        }
        result
    }

}


fn factorial(n:u32) -> u32{
    let mut result = 1;
    if n == 0 {
        return 1;
    }
    for i in 1..n+1{
        result *= i;
    }
    result
}
