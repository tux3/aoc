use anyhow::Result;
use ndarray::Array2;
use std::collections::HashMap;
use std::convert::identity;
use std::path::Path;

pub type Pos = (usize, usize);

pub struct Grid {
    // For each plot on the grid, the number of its region
    pub regions: Array2<usize>,
}

impl Grid {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let grid = std::fs::read_to_string(path)?;
        let shape = grid.chars().position(|c| c == '\n').unwrap();
        let grid = Array2::from_shape_vec(
            (shape, shape),
            grid.trim()
                .chars()
                .filter(|&c| c != '\n')
                .collect::<Vec<_>>(),
        )?;
        let regions = Self::number_regions(&grid);
        Ok(Self { regions })
    }

    // [left, right, down, up]
    fn neighbors(size: usize, (y, x): Pos) -> impl Iterator<Item = Pos> {
        [
            (x > 0).then(|| (y, x - 1)),
            (x < size - 1).then(|| (y, x + 1)),
            (y < size - 1).then(|| (y + 1, x)),
            (y > 0).then(|| (y - 1, x)),
        ]
        .into_iter()
        .filter_map(identity)
    }

    fn flood_fill(grid: &Array2<char>, regions: &mut Array2<usize>, (y, x): Pos, region: usize) {
        regions[(y, x)] = region;
        for pos in Self::neighbors(grid.ncols(), (y, x)) {
            if grid[(y, x)] == grid[pos] && regions[pos] == 0 {
                Self::flood_fill(grid, regions, pos, region)
            }
        }
    }

    fn number_regions(grid: &Array2<char>) -> Array2<usize> {
        let mut regions = Array2::default(grid.raw_dim());

        let mut region_idx = 1;
        for ((y, x), _val) in grid.indexed_iter() {
            if regions[(y, x)] == 0 {
                Self::flood_fill(grid, &mut regions, (y, x), region_idx);
                region_idx += 1;
            }
        }
        regions
    }

    // [left, right, down, up]
    fn matching_neighbors(&self, (y, x): Pos) -> [usize; 4] {
        let val = self.regions[(y, x)];
        [
            (x > 0 && self.regions[(y, x - 1)] == val) as usize,
            (x < self.regions.ncols() - 1 && self.regions[(y, x + 1)] == val) as usize,
            (y < self.regions.nrows() - 1 && self.regions[(y + 1, x)] == val) as usize,
            (y > 0 && self.regions[(y - 1, x)] == val) as usize,
        ]
    }

    fn num_corners(&self, (y, x): Pos) -> usize {
        let val = self.regions[(y, x)];
        let neigh = self.matching_neighbors((y, x));
        let num_neigh: usize = neigh.iter().sum();
        if num_neigh == 0 {
            4 // A square by itself
        } else if num_neigh == 1 {
            2 // A U-turn
        } else if num_neigh == 2 {
            match neigh {
                [1, 1, 0, 0] | [0, 0, 1, 1] => 0, // A straight line
                [l, r, d, u] => {
                    let concavity = (self.regions[(y + d - u, x + r - l)] != val) as usize;
                    1 + concavity
                }
            }
        } else if num_neigh == 3 {
            // The simple edge    X
            // If it looks like: XXX
            // Then we have no neighbor below, but two concave corners above (left and right)
            match neigh {
                [l, r, 1, 1] => {
                    (self.regions[(y + 1, x + r - l)] != val) as usize
                        + (self.regions[(y - 1, x + r - l)] != val) as usize
                }
                [1, 1, d, u] => {
                    (self.regions[(y + d - u, x - 1)] != val) as usize
                        + (self.regions[(y + d - u, x + 1)] != val) as usize
                }
                _ => unreachable!(),
            }
        } else {
            // Four neighbours, but we must still detect any concave corners
            itertools::iproduct!([y - 1, y + 1], [x - 1, x + 1])
                .map(|(y, x)| (self.regions[(y, x)] != val) as usize)
                .sum()
        }
    }

    pub fn fence_prices(&self) -> (usize, usize) {
        let mut areas = HashMap::<usize, usize>::new();
        let mut perimeter = HashMap::<usize, usize>::new();
        let mut bulk_perimeter = HashMap::<usize, usize>::new();

        // I can compute the boundary of a region by all the plots that don't have 4 neighbors
        // And I think there are as many edges as there are corners!

        for (pos, &region) in self.regions.indexed_iter() {
            *areas.entry(region).or_default() += 1;
            *perimeter.entry(region).or_default() +=
                4 - self.matching_neighbors(pos).iter().sum::<usize>();
            *bulk_perimeter.entry(region).or_default() += self.num_corners(pos);
        }
        let price = perimeter.iter().map(|(k, v)| v * areas[k]).sum();
        let bulk_price = bulk_perimeter.iter().map(|(k, v)| v * areas[k]).sum();
        (price, bulk_price)
    }
}
