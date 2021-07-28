struct Grid {
    data: Vec<bool>,
    height: usize,
    width: usize,
}

struct Point(usize, usize);

impl Grid {
    // Returns a new grid of size height * width filled with false
    fn new(height: usize, width: usize) -> Grid {
        Grid {
            data: vec![false; height * width],
            height,
            width,
        }
    }

    // Retrieve the value in the grid at location
    fn get(&self, location: Point) -> bool {
        if location.0 < self.width && location.1 < self.height {
            self.data[location.0 + location.1 * self.width]
        } else {
            panic!(
                "x has to be lower than {} and y should be lower than {}",
                self.width, self.height
            );
        }
    }

    // Replace the value at the given location inside of the grid with another value
    fn set(&mut self, location: Point, value: bool) {
        if location.0 < self.width && location.1 < self.height {
            self.data[location.0 + location.1 * self.width] = value;
        } else {
            panic!(
                "x has to be lower than {} and y should be lower than {}",
                self.width, self.height
            );
        }
    }

    // Convert the data to a string that can be displayed
    fn display(&self) -> String {
        let mut board: String = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                board.push(if self.get(Point(x, y)) { '@' } else { '.' });
            }
            board.push('\n');
        }
        board
    }

    // Compute the next level of the grid
    fn next_level(&mut self) {
        // Save changes to be applied later (death -> life / life -> death)
        let mut changes: Vec<Point> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                // Count neighbors
                let neighbors = self.count_neighbors(Point(x, y));

                // Save the change if needed
                if !self.get(Point(x, y)) && neighbors == 3
                    || self.get(Point(x, y)) && !(neighbors == 2 || neighbors == 3)
                {
                    changes.push(Point(x, y));
                }
            }
        }

        // Apply the changes to the board
        for change in changes {
            self.set(
                Point(change.0, change.1),
                !self.get(Point(change.0, change.1)),
            );
        }
    }

    // Count the number of living neighbors of a cell at location
    fn count_neighbors(&self, location: Point) -> usize {
        let mut count = 0;

        // top
        if location.1 > 0 && self.get(Point(location.0, location.1 - 1)) {
            count += 1;
        }

        // bottom
        if location.1 < self.height - 1 && self.get(Point(location.0, location.1 + 1)) {
            count += 1;
        }

        // left
        if location.0 > 0 && self.get(Point(location.0 - 1, location.1)) {
            count += 1;
        }

        // right
        if location.0 + 1 < self.width && self.get(Point(location.0 + 1, location.1)) {
            count += 1;
        }

        // top left
        if location.1 > 0 && location.0 > 0 && self.get(Point(location.0 - 1, location.1 - 1)) {
            count += 1;
        }

        // top right (x + 1, y - 1)
        if location.0 + 1 < self.width
            && location.1 > 0
            && self.get(Point(location.0 + 1, location.1 - 1))
        {
            count += 1;
        }

        // bottom left (x - 1, y + 1)
        if location.0 > 0
            && location.1 < self.height - 1
            && self.get(Point(location.0 - 1, location.1 + 1))
        {
            count += 1;
        }

        // bottom right (x + 1, y + 1)
        if location.0 + 1 < self.width
            && location.1 < self.height - 1
            && self.get(Point(location.0 + 1, location.1 + 1))
        {
            count += 1;
        }

        count
    }
}

fn main() {
    // Instantiate the board
    let mut board = Grid::new(10, 20);

    // Create test structures
    // Toad
    board.set(Point(0, 1), true);
    board.set(Point(1, 1), true);
    board.set(Point(2, 1), true);
    board.set(Point(1, 2), true);
    board.set(Point(2, 2), true);
    board.set(Point(3, 2), true);

    // Beacon
    board.set(Point(5, 6), true);
    board.set(Point(5, 7), true);
    board.set(Point(6, 6), true);
    board.set(Point(7, 9), true);
    board.set(Point(8, 9), true);
    board.set(Point(8, 8), true);

    // Run the game
    for level in 1..21 {
        println!("Level {}", level);
        print!("{}", board.display());
        board.next_level();
    }
}
