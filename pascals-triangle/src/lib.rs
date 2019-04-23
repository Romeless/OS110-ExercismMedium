// https://exercism.io/tracks/rust/exercises/pascals-triangle

pub struct PascalsTriangle {
	triangle : Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut grand_vec = PascalsTriangle {triangle: Vec::new()};
		
		for i in 0..row_count {
			let mut vec = Vec::new();
			
			for j in 0..i+1 {
				if j == 0 {vec.push(1);}
				else if j == i {vec.push(1);}
				else {
					vec.push(grand_vec.triangle[(i-1) as usize][(j-1) as usize] 
					+ grand_vec.triangle[(i-1) as usize][j as usize]);
				}
			}
			
			grand_vec.triangle.push(vec);
		}
		
		grand_vec
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.to_vec()
    }
}
