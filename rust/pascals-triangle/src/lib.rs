pub struct PascalsTriangle {
    pascal_triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut pascal_triangle = Vec::new();
        for i in 0..row_count as usize {
            let factorials = factorial(i as u32);
            let v = (0..=i)
                .map(|j| factorials[i] / (factorials[j] * factorials[i - j]))
                .collect();
            pascal_triangle.push(v)
        }

        Self { pascal_triangle }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.pascal_triangle.to_vec()
    }
}

pub fn factorial(num: u32) -> Vec<u32> {
    let mut factorials = vec![1];
    (1..=num).for_each(|i| factorials.push(factorials.last().unwrap() * i));
    factorials
}
