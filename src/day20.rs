// code partially taken from here because i was super stuck today:
// https://gist.github.com/miseran/c8da49e394975f0e2a297e1fa0fefbf5

use itertools::{iproduct, Itertools};
use std::collections::{HashMap, BTreeSet};
use std::iter::FromIterator;
use std::convert::TryInto;

const FILE: &str = "inputs/day20.txt";

const BLOCKWIDTH: usize = 10;
const FIELDWIDTH: usize = 12;

const MONSTER: [(usize, usize); 15] = [(1, 0), (2, 1), (2, 4), (1, 5), (1, 6), (2, 7), (2, 10), (1, 11), (1, 12), (2, 13), (2, 16), (1, 17), (0, 18), (1, 18), (1, 19)];

const EDGE_N: usize = 0;
const EDGE_E: usize = 1;
const EDGE_S: usize = 2;
const EDGE_W: usize = 3;

type Bitmap = Vec<Vec<bool>>;

#[derive(Debug)]
struct Tile {
    id: u64,
    image: Bitmap,
    edges: [u64; 4],
}

impl Tile {
    fn parse(s: &str) -> Tile {
        let bytes = s.as_bytes();
        let id = s[5..9].parse().unwrap();
        let mut image = vec![vec![false; BLOCKWIDTH]; BLOCKWIDTH];
        for (i, j) in iproduct!(0..BLOCKWIDTH, 0..BLOCKWIDTH) {
            if bytes[(BLOCKWIDTH + 1) * (i + 1) + j] == b'#' {
                image[i][j] = true;
            }
        }

        /*
        let n_edge = (0..10)
            .fold(0u64, |acc, i| if image[0][i] { (acc << 1) + 1 } else { acc << 1 });
        let s_edge = (0..10)
            .fold(0u64, |acc, i| if image[BLOCKWIDTH - 1][i] { (acc << 1) + 1 } else { acc << 1 });
        let e_edge = (0..10)
            .fold(0u64, |acc, i| if image[i][0] { (acc << 1) + 1 } else { acc << 1 });
        let w_edge = (0..10)
            .fold(0u64, |acc, i| if image[i][BLOCKWIDTH - 1] { (acc << 1) + 1 } else { acc << 1 });
*/
        let edges = image_edges(&image);
        Tile {
            id,
            image,
            edges,
            //edges: [n_edge, e_edge, s_edge, w_edge],
        }
    }

    fn rotated(&self, north: usize, west: usize) -> Tile {
        let image = rotate_image(&self.image, north, west);
        /*
        let n_edge = (0..10)
            .fold(0u64, |acc, i| if image[0][i] { (acc << 1) + 1 } else { acc << 1 });
        let s_edge = (0..10)
            .fold(0u64, |acc, i| if image[BLOCKWIDTH - 1][i] { (acc << 1) + 1 } else { acc << 1 });
        let e_edge = (0..10)
            .fold(0u64, |acc, i| if image[i][0] { (acc << 1) + 1 } else { acc << 1 });
        let w_edge = (0..10)
            .fold(0u64, |acc, i| if image[i][BLOCKWIDTH - 1] { (acc << 1) + 1 } else { acc << 1 });
*/
        let edges = image_edges(&image);
        Tile {
            id: self.id,
            image,
            //edges: [n_edge, e_edge, s_edge, w_edge],
            edges
        }
    }
}

/**
source: https://gist.github.com/miseran/c8da49e394975f0e2a297e1fa0fefbf5
*/
fn image_edges(image: &Bitmap) -> [u64; 4] {
    let mut edges = [(0, 0); 4];
    let len = image.len();
    for i in 0..len {
        if image[0][i] {
            edges[EDGE_N].0 += 1<<i;
            edges[EDGE_N].1 += 1<<(len-i-1);
        }
        if image[len-1][i] {
            edges[EDGE_S].0 += 1<<i;
            edges[EDGE_S].1 += 1<<(len-i-1);
        }
        if image[i][0] {
            edges[EDGE_W].0 += 1<<i;
            edges[EDGE_W].1 += 1<<(len-i-1);
        }
        if image[i][len-1] {
            edges[EDGE_E].0 += 1<<i;
            edges[EDGE_E].1 += 1<<(len-i-1);
        }
    }
    edges.iter().map(|&(a, b)| a.min(b)).collect::<Vec<_>>().try_into().unwrap()
}

/**
source: https://gist.github.com/miseran/c8da49e394975f0e2a297e1fa0fefbf5
*/
fn edge_matches(edge: u64, target: Option<u64>, counts: &HashMap<u64, u64>) -> bool {
    Some(edge) == target || (target.is_none() && counts[&edge] == 1)
}

/**
North is the edge index that should end up on north, likewise west.
source: https://gist.github.com/miseran/c8da49e394975f0e2a297e1fa0fefbf5
*/
fn rotate_image(image: &Bitmap, north: usize, west: usize) -> Bitmap {
    let len = image.len();
    let mut new = vec![vec![false; len]; len];
    for (r, c) in iproduct!(0..len, 0..len) {
        new[r][c] = match (north, west) {
            (EDGE_N, EDGE_W) => image[r][c],
            (EDGE_E, EDGE_N) => image[c][len-r-1],
            (EDGE_S, EDGE_E) => image[len-r-1][len-c-1],
            (EDGE_W, EDGE_S) => image[len-c-1][r],
            (EDGE_W, EDGE_N) => image[c][r],
            (EDGE_N, EDGE_E) => image[r][len-c-1],
            (EDGE_S, EDGE_W) => image[len-r-1][c],
            (EDGE_E, EDGE_S) => image[len-c-1][len-r-1],
            _ => panic!("Bad rotation."),
        }
    }
    new
}

pub fn main() {
    let text = std::fs::read_to_string(FILE).expect("Bad file");

    let tiles = text.split("\n\n")
        .map(|s| {
            let tile = Tile::parse(s);
            (tile.id, tile)
        }).collect::<HashMap<_,_>>();

    let edge_counts = tiles.iter().fold(HashMap::new(), |mut dict, (_,tile)| {
        for edge in tile.edges.iter() {
            if let Some(n) = dict.get_mut(edge) {
                *n += 1;
            } else {
                dict.insert(*edge, 1);
            }
        }

        dict
    });

    eprintln!("{:#?}", edge_counts.len());
    let mut grid: [[Option<Tile>; FIELDWIDTH];FIELDWIDTH] = Default::default();
    let mut unused = tiles.keys().map(|k| *k).collect::<BTreeSet<_>>();

    for (r,c) in iproduct!(0..FIELDWIDTH, 0..FIELDWIDTH) {
        let north = if r==0 { None } else {
            if let Some(tile) = grid[r - 1][c].as_ref() {
                Some(tile.edges[EDGE_S])
            } else {
                None
            }
        };

        let west = if c==0 { None } else {
            if let Some(tile) = grid[r][c - 1].as_ref() {
                Some(tile.edges[EDGE_E])
            } else {
                None
            }
        };

        eprint!("{:?}", (r,c));

        let (tile, e, d) = unused.iter()
            .map(|id| tiles.get(id).unwrap())
            .find_map(|tile| {
                let e = tile.edges.iter().enumerate()
                    .find(|(i, edge)| edge_matches(**edge, north, &edge_counts))
                    .map(|(i, edge)| i)?;

                let d = [1,3].iter().find(|d| {
                    let orth_edge = tile.edges[(e+((**d) as usize))%4];
                    edge_matches(orth_edge, west, &edge_counts)
                }).map(|x| (*x) as usize)?;

                Some((tile, e,d))
            }).unwrap();

        eprintln!(" {}", tile.id);

        unused.remove(&tile.id);
        grid[r][c] = Some(tile.rotated(e, (e+d)%4));
    }

    let nw = grid.get(0).unwrap().get(0).unwrap().as_ref().unwrap().id;
    let ne = grid.get(0).unwrap().get(FIELDWIDTH - 1).unwrap().as_ref().unwrap().id;
    let sw = grid.get(FIELDWIDTH - 1).unwrap().get(0).unwrap().as_ref().unwrap().id;
    let se = grid.get(FIELDWIDTH - 1).unwrap().get(FIELDWIDTH - 1).unwrap().as_ref().unwrap().id;
    println!("DAY 20, PART 1: {}", nw * ne * sw * se);

    let mut assembled: Bitmap = vec![vec![false; FIELDWIDTH*(BLOCKWIDTH-2)]; FIELDWIDTH*(BLOCKWIDTH-2)];
    for (r, c, br, bc) in iproduct!(0..FIELDWIDTH, 0..FIELDWIDTH, 1..BLOCKWIDTH-1, 1..BLOCKWIDTH-1) {
        assembled[r*(BLOCKWIDTH-2) + br - 1][c*(BLOCKWIDTH-2) + bc - 1] = grid[r][c].as_ref().unwrap().image[br][bc];
    }

    let num_hashes = iproduct!(0..assembled.len(), 0..assembled.len())
        .filter(|(r,c)| assembled[*r][*c]).count();

    let ps = [EDGE_N, EDGE_E, EDGE_S, EDGE_W, EDGE_S, EDGE_E, EDGE_N];
    let monsters = ps.iter().tuple_windows::<(_,_)>()
        .find_map(|(n, w)| {
            let img = rotate_image(&assembled, *n, *w);
            let monsters = iproduct!(0..(assembled.len()-2), 0..(assembled.len()-19))
                .filter(|(r,c)|
                MONSTER.iter().all(|(mr, mc)| img[r+mr][c+mc])
            ).count();

            if monsters > 0 { Some(monsters) } else { None }
        }).unwrap();

    println!("DAY 20, PART 2: {:?}", num_hashes - MONSTER.len()*monsters);
}