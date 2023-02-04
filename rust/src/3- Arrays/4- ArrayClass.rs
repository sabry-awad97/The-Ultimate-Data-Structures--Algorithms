struct Array {
    data: Vec<i32>,
    size: usize,
}

impl Array {
    fn new() -> Array {
        Array {
            data: vec![0; 10],
            size: 0,
        }
    }

    fn add(&mut self, value: i32) {
        if self.size == self.data.len() {
            self.resize();
        }
        self.data[self.size] = value;
        self.size += 1;
    }

    fn insert(&mut self, index: usize, value: i32) {
        if self.size == self.data.len() {
            self.resize();
        }
        for i in (index..self.size).rev() {
            self.data[i + 1] = self.data[i];
        }
        self.data[index] = value;
        self.size += 1;
    }

    fn remove_at(&mut self, index: usize) {
        for i in index..self.size - 1 {
            self.data[i] = self.data[i + 1];
        }
        self.size -= 1;
    }

    fn index_of(&self, value: i32) -> i32 {
        for i in 0..self.size {
            if self.data[i] == value {
                return i as i32;
            }
        }
        -1
    }

    fn display(&self) {
        for i in 0..self.size {
            println!("{}", self.data[i]);
        }
    }

    fn resize(&mut self) {
        let mut new_data = vec![0; self.data.len() * 2];
        for i in 0..self.data.len() {
            new_data[i] = self.data[i];
        }
        self.data = new_data;
    }
}

fn main() {
    let mut array = Array::new();
    array.add(1);
    array.add(2);
    array.add(3);
    array.display();
    array.insert(1, 4);
    array.display();
    array.remove_at(1);
    array.display();
    println!("{}", array.index_of(3));
}
