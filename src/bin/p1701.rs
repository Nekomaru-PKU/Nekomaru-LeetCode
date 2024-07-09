mod solution {
    pub fn main(customers: Vec<Vec<i32>>) -> f64 {
        let mut now = 0;
        let mut sum = 0;
        for customer in &customers {
            let arrival = customer[0];
            let time = customer[1];
            sum += ((now - arrival).max(0) + time) as i64;
            now = now.max(arrival) + time;
        }
        sum as f64 / customers.len() as f64
    }
}

fn main() {
    fn as_input(arr: &[[i32; 2]]) -> Vec<Vec<i32>> {
        arr.iter().map(|x| x.to_vec()).collect()
    }

    assert_eq!(solution::main(as_input(&[[1,2],[2,5],[4,3]])), 5.0);
    assert_eq!(solution::main(as_input(&[[5,2],[5,4],[10,3],[20,1]])), 3.25);
}
