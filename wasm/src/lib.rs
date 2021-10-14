/*
 * @Author: Lin Bowen
 * @Date: 2021-10-12 21:04:24
 * @LastEditTime: 2021-10-15 02:49:33
 * @LastEditors: Lin Bowen
 * @Description:
 * @FilePath: \wasm\src\lib.rs
 */
use js_sys::*;
use md5;
use rand::Rng;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::*;

#[wasm_bindgen(js_name = RustMD5)]
pub fn hasher(data: &str) -> String {
    let digest = md5::compute(data);
    let res = format!("{:x}", digest);
    res
}

#[derive(Debug)]
pub struct UnionSet {
    set: Vec<i32>,
}
impl UnionSet {
    pub fn new(size: i32) -> UnionSet {
        let mut union_set = UnionSet { set: Vec::new() };
        for _ in 0..size {
            union_set.set.push(-1)
        }
        union_set
    }
    fn union(&mut self, root1: usize, root2: usize) {
        if self.set[root1] < self.set[root2] {
            self.set[root2] = root1 as i32
        } else {
            if self.set[root1] == self.set[root2] {
                self.set[root2] -= 1
            }
            self.set[root1] = root2 as i32
        }
    }
    fn find_set(&mut self, x: usize) -> i32 {
        if self.set[x] < 0 {
            return x as i32;
        } else {
            let val = self.find_set(self.set[x] as usize);
            self.set[x] = val;
            return val;
        }
    }
    fn same_set(&mut self, x: i32, y: i32) -> bool {
        return self.find_set(x as usize) == self.find_set(y as usize);
    }
    fn union_element(&mut self, x: i32, y: i32) {
        let x = self.find_set(x as usize);
        let y = self.find_set(y as usize);
        self.union(x as usize, y as usize)
    }
}

#[test]
fn unionSet_test() {
    let mut unionSet = UnionSet::new(20);
    unionSet.union(0, 4);

    println!("new");
    assert!(unionSet.set[0] == 4, "new :{:?}", unionSet);

    println!("same_set");
    assert!(unionSet.same_set(0, 4), "same_set");

    println!("union_element");
    unionSet.union_element(4, 10);
    assert!(unionSet.same_set(0, 10), "union_element");
}
#[wasm_bindgen]
pub struct RustMaze {
    columns: u32,
    rows: u32,
    cells: u32,
    linked_map: HashMap<i32, Vec<i32>>,
    union_sets: UnionSet,
    canvas: HtmlCanvasElement,
}
#[wasm_bindgen]
impl RustMaze {
    #[wasm_bindgen(constructor)]
    pub fn new(columns: u32, rows: u32, canvas: HtmlCanvasElement) -> RustMaze {
        let cells = columns * rows;
        let union_sets = UnionSet::new(cells as i32);
        let linked_map: HashMap<i32, Vec<i32>> = HashMap::new();
        return RustMaze {
            columns,
            rows,
            cells,
            union_sets,
            linked_map,
            canvas,
        };
    }
    pub fn pick_random_cell_pairs(&self) -> Vec<i32> {
        let cell = rand::thread_rng().gen_range(0..self.cells);
        let mut neibor_cells: Vec<u32> = Vec::new();
        let row = (cell / self.columns) >> 0;
        let column = cell % self.rows;

        if row != 0 {
            neibor_cells.push(cell - self.columns)
        }

        if row != self.rows - 1 {
            neibor_cells.push(cell + self.columns)
        }

        if column != 0 {
            neibor_cells.push(cell - 1)
        }

        if column != self.columns - 1 {
            neibor_cells.push(cell + 1)
        }
        let index = rand::thread_rng().gen_range(0..neibor_cells.len());
        vec![cell as i32, neibor_cells[index] as i32]
    }
    pub fn linked_to_first_cell(&mut self) -> bool {
        for i in 1..self.cells {
            if !self.union_sets.same_set(0, i as i32) {
                return false;
            }
        }
        return true;
    }
    pub fn add_linked_map(&mut self, x: i32, y: i32) {
        if !self.linked_map.contains_key(&x) {
            self.linked_map.insert(x, Vec::new());
        }
        if !self.linked_map.contains_key(&y) {
            self.linked_map.insert(y, Vec::new());
        }
        if !self.linked_map.get(&x).unwrap().contains(&y) {
            self.linked_map.get_mut(&x).unwrap().push(y);
        }
        if !self.linked_map.get(&y).unwrap().contains(&x) {
            self.linked_map.get_mut(&y).unwrap().push(x);
        }
    }
    pub fn generate(&mut self) {
        while !self.linked_to_first_cell() {
            let cell_pairs = self.pick_random_cell_pairs();
            if !self
                .union_sets
                .same_set(cell_pairs[0] as i32, cell_pairs[1] as i32)
            {
                self.union_sets
                    .union_element(cell_pairs[0] as i32, cell_pairs[1] as i32);
                self.add_linked_map(cell_pairs[0], cell_pairs[1]);
            }
        }
    }
    pub fn draw(&self) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvasBuffer = document.create_element("canvas").unwrap();
        let canvasBuffer: web_sys::HtmlCanvasElement = canvasBuffer
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        canvasBuffer.set_width(self.canvas.width());
        canvasBuffer.set_height(self.canvas.height());
        let cellWidth = canvasBuffer.width() / self.columns;
        let cellHeight = canvasBuffer.height() / self.rows;
        let ctx = canvasBuffer
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        ctx.translate(0.5, 0.5).unwrap();
        for i in 0..self.cells {
            let row = (i / self.columns) >> 0;
            let column = i % self.columns;
            if column != self.columns - 1
                && (!self.linked_map.contains_key(&(i as i32))
                    || !self
                        .linked_map
                        .get(&(i as i32))
                        .unwrap()
                        .contains(&((i + 1) as i32)))
            {
                ctx.move_to(
                    (((column + 1) * cellWidth) >> 0) as f64,
                    ((row * cellHeight) >> 0) as f64,
                );
                ctx.line_to(
                    (((column + 1) * cellWidth) >> 0) as f64,
                    ((row + 1) * cellHeight >> 0) as f64,
                );
            }
            if row != self.rows - 1
                && (!self.linked_map.contains_key(&(i as i32))
                    || !self
                        .linked_map
                        .get(&(i as i32))
                        .unwrap()
                        .contains(&((i + self.columns) as i32)))
            {
                ctx.move_to(
                    ((column * cellWidth) >> 0) as f64,
                    ((row + 1) * cellHeight >> 0) as f64,
                );
                ctx.line_to(
                    (((column + 1) * cellWidth) >> 0) as f64,
                    ((row + 1) * cellHeight >> 0) as f64,
                );
            }
        }
        ctx.move_to(0.0, 0.0);
        ctx.line_to(canvasBuffer.width() as f64 - 0.5, 0.0);
        ctx.line_to(
            canvasBuffer.width() as f64 - 0.5,
            canvasBuffer.height() as f64 - 0.5,
        );
        ctx.line_to(0.0, canvasBuffer.height() as f64 - 0.5);
        ctx.line_to(0.0, 0.0);
        ctx.stroke();
        self.canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap()
            .draw_image_with_html_canvas_element(&canvasBuffer, 0.0, 0.0)
            .unwrap();
    }
}

#[wasm_bindgen_test]
fn maze_test() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    //pick_random_cell_pairs
    let mut maze = RustMaze::new(10, 10, canvas);
    let pair = maze.pick_random_cell_pairs();
    assert!(
        pair[0] == pair[1] + 1
            || pair[0] == pair[1] - 1
            || pair[0] == pair[1] - 10
            || pair[0] - 10 == pair[1],
        "{:?}",
        pair
    );

    //linked_to_first_cell
    for i in 1..maze.union_sets.set.len() {
        maze.union_sets.set[i] = 0
    }
    println!("{:?}", maze.union_sets.set);
    assert!(maze.linked_to_first_cell());

    //generate
    for i in 0..maze.union_sets.set.len() {
        maze.union_sets.set[i] = -1
    }
    maze.generate();
    println!("{:?}", maze.union_sets.set);
    assert!(maze.linked_to_first_cell());
}
