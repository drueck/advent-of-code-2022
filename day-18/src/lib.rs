pub struct UnitCube {
    x: usize, // x-1..x
    y: usize, // y-1..y
    z: usize, // z-1..z
}

impl UnitCube {
    pub fn from_str(s: &str) -> Self {
        let mut coords = s.split(',');
        let mut next_coord = || coords.next().unwrap().parse().unwrap();

        Self {
            x: next_coord(),
            y: next_coord(),
            z: next_coord(),
        }
    }

    // each face is:  [xa, ya, za, xb, yb, zb]
    // bottom left --> ----------  ---------- <-- top right
    pub fn faces(&self) -> [[usize; 6]; 6] {
        let (x0, y0, z0, x1, y1, z1) = (self.x, self.y, self.z, self.x + 1, self.y + 1, self.z + 1);
        [
            [x0, y0, z0, x1, y1, z0],
            [x0, y0, z1, x1, y1, z1],
            [x0, y1, z0, x1, y1, z1],
            [x0, y0, z0, x1, y0, z1],
            [x0, y0, z0, x0, y1, z1],
            [x1, y0, z0, x1, y1, z1],
        ]
    }
}
