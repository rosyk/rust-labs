fn transpose_matrix(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed_matrix = vec![vec![0; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed_matrix[j][i] = matrix[i][j];
        }
    }

    transposed_matrix
}

#[derive(Debug, Clone)]
struct Employee {
    position: String,
    salary: f64,
    start_date: String,
}

impl Employee {
    fn new(position: String, salary: f64, start_date: String) -> Employee {
        Employee {
            position,
            salary,
            start_date,
        }
    }
    
    fn raise_salary(&mut self, percentage: f64) {
        self.salary *= (1.0 + percentage / 100.0);
    }
    
    fn change_position(&mut self, new_position: String) {
        self.position = new_position;
    }
    
    fn years_of_service(&self, current_date: String) -> f64 {
        let start_year: i32 = self.start_date.split('-').next().unwrap().parse().unwrap();
        let current_year: i32 = current_date.split('-').next().unwrap().parse().unwrap();

        (current_year - start_year) as f64
    }
    
    fn is_salary_above(&self, threshold: f64) -> bool {
        self.salary > threshold
    }
    
    fn get_info(&self) -> String {
        format!(
            "Job title: {}, Salary: {:.2}, Date of star working: {}",
            self.position, self.salary, self.start_date
        )
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let transposed = transpose_matrix(&matrix);
    println!("matrix: {:?}", matrix);
    println!("transposed matrix: {:?}", transposed);
    
    let mut employee = Employee::new("Engineer".to_string(), 50000.0, "2020-01-15".to_string());

    println!("Employee info: {}", employee.get_info());

    employee.raise_salary(15.0);
    println!("Salary after promotion: {:.2}", employee.salary);

    employee.change_position("Senior Engineer".to_string());
    println!("New position: {}", employee.position);

    let years = employee.years_of_service("2024-01-01".to_string());
    println!("Work experience: {:.1} years", years);

    let above_threshold = employee.is_salary_above(55000.0);
    println!("Is salary more than 55000? {}", above_threshold);

    println!("Full employee info (Debug): {:?}", employee);
}
