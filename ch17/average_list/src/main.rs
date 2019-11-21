pub struct AverageValues {
    values: Vec<i32>,
    average: f64,
}

impl AverageValues {
    pub fn add(&mut self, v: i32) {
        self.values.push(v);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.values.pop();
        match result {
            Some(v) => {
                self.update_average();
                Some(v)
            }
            None => None,
        }
    }

    pub fn update_average(&mut self) {
        let sum: i32 = self.values.iter().sum();
        self.average = sum as f64 / self.values.len() as f64;
    }

    pub fn average(&self) -> f64 {
        self.average
    }
}

fn main() {
    let mut a = AverageValues {
        average: 0.0,
        values: vec![],
    };
    a.add(5);
    a.add(6);
    println!("Average: {}", a.average());
}
