use super::color::Color;

#[derive(Debug, Clone)]
pub struct Array {
    cells: Vec<Vec<Color>>,
    height: usize,
}

impl Array {
    pub fn new(height: usize) -> Array {
        Array {
            cells: Vec::new(),
            height,
        }
    }

    pub fn with_dim(width: usize, height: usize) -> Array {
        Array {
            cells: Vec::with_capacity(width),
            height,
        }
    }

    pub fn add_line(&mut self, line: Vec<Color>) {
        if line.len() != self.height {
            panic!("mismatch height");
        }

        self.cells.push(line);
    }

    pub fn height(&self) -> usize {
        self.height
    }

    #[allow(dead_code)]
    pub fn show_array(&self) {
        let w = self.cells.len();
        let h = self.height;

        for y in 0..h {
            for x in 0..w {
                let c = &self.cells[x][y];
                let w = (c.r + c.g + c.b) / 3.0;
                if w < 0.25 {
                    print!(" ");
                } else if w < 0.5 {
                    print!("░");
                } else if w < 0.75 {
                    print!("▒");
                } else if w < 0.9 {
                    print!("▓");
                } else {
                    print!("█");
                }
            }
            println!();
        }
    }
}

impl super::Imageable for Array {
    fn dim(&self) -> (usize, usize) {
        (self.cells.len(), self.height)
    }

    fn get(&self, x: usize, y: usize) -> &super::color::Color {
        &self.cells[x][y]
    }
}

impl AsRef<Vec<Vec<Color>>> for Array {
    fn as_ref(&self) -> &Vec<Vec<Color>> {
        &self.cells
    }
}
