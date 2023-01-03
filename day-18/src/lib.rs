use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct UnitCube {
    pub x: isize, // x..x+1
    pub y: isize, // y..y+1
    pub z: isize, // z..z+1
    pub faces: [[isize; 6]; 6],
}

impl PartialEq for UnitCube {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for UnitCube {}

impl Hash for UnitCube {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl UnitCube {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        let faces = Self::build_faces(x, y, z);
        Self { x, y, z, faces }
    }

    pub fn coords(&self) -> (isize, isize, isize) {
        (self.x, self.y, self.z)
    }

    pub fn from_str(s: &str) -> Self {
        let mut coords = s.split(',');
        let mut next_coord = || coords.next().unwrap().parse().unwrap();
        Self::new(next_coord(), next_coord(), next_coord())
    }

    // each face is:  [xa, ya, za, xb, yb, zb]
    // bottom left --> ----------  ---------- <-- top right
    pub fn build_faces(x: isize, y: isize, z: isize) -> [[isize; 6]; 6] {
        let (x0, y0, z0, x1, y1, z1) = (x, y, z, x + 1, y + 1, z + 1);
        [
            [x0, y0, z1, x1, y1, z1], // back
            [x0, y0, z0, x1, y1, z0], // front
            [x0, y1, z0, x1, y1, z1], // top
            [x0, y0, z0, x1, y0, z1], // bottom
            [x1, y0, z0, x1, y1, z1], // right
            [x0, y0, z0, x0, y1, z1], // left
        ]
    }

    // adjacent cubes within the given bounding cube in the same order as faces
    pub fn adjacent_cubes_within(&self, bounding_cube: &BoundingCube) -> [Option<UnitCube>; 6] {
        [
            (self.z + 1 <= bounding_cube.max_z).then_some(Self::new(self.x, self.y, self.z + 1)),
            (self.z - 1 >= bounding_cube.min_z).then_some(Self::new(self.x, self.y, self.z - 1)),
            (self.y + 1 <= bounding_cube.max_y).then_some(Self::new(self.x, self.y + 1, self.z)),
            (self.y - 1 >= bounding_cube.min_y).then_some(Self::new(self.x, self.y - 1, self.z)),
            (self.x + 1 <= bounding_cube.max_x).then_some(Self::new(self.x + 1, self.y, self.z)),
            (self.x - 1 >= bounding_cube.min_x).then_some(Self::new(self.x - 1, self.y, self.z)),
        ]
    }
}

#[derive(Debug)]
pub struct BoundingCube {
    pub min_x: isize,
    pub min_y: isize,
    pub min_z: isize,
    pub max_x: isize,
    pub max_y: isize,
    pub max_z: isize,
}

impl BoundingCube {
    pub fn new(cube: &UnitCube) -> Self {
        Self {
            min_x: cube.x - 1,
            min_y: cube.y - 1,
            min_z: cube.z - 1,
            max_x: cube.x + 1,
            max_y: cube.y + 1,
            max_z: cube.z + 1,
        }
    }

    pub fn from_cubes(cubes: &HashSet<UnitCube>) -> Self {
        assert!(cubes.len() > 0);
        let mut bounding_cube = Self::new(&cubes.iter().next().unwrap());
        for cube in cubes.iter() {
            bounding_cube.include(&cube);
        }
        bounding_cube
    }

    pub fn include(&mut self, cube: &UnitCube) {
        self.min_x = self.min_x.min(cube.x - 1);
        self.min_y = self.min_y.min(cube.y - 1);
        self.min_z = self.min_z.min(cube.z - 1);
        self.max_x = self.max_x.max(cube.x + 1);
        self.max_y = self.max_y.max(cube.y + 1);
        self.max_z = self.max_z.max(cube.z + 1);
    }
}
