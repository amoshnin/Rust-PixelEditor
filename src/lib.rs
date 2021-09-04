use wasm_bindgen::prelude::*;
use im::Vector;
use std::iter::FromIterator;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[wasm_bindgen]
#[derive(Clone)] // Immutable datastructure allows for cheap/quick cloning
pub struct Image {
    width: usize,
    height: usize,
    cells: Vector<Rgb>
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Self {
        let cells = Vector::from_iter((0..width * height).map(|_| Rgb { r:200, g: 200, b: 255 } ));
        Self { width, height, cells }
    }

    pub fn cells(&self) -> Vec<u8> {
        self.cells
            .iter()
            .map(|&rgb| vec![rgb.r, rgb.g, rgb.b])
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn brush(&self, x: usize, y: usize, color: Vec<u8>) -> Image {
        let index = (y * self.width) + x;
        let new_cells = self.cells.update(index, Rgb { r: color[0], g: color[1], b: color[2] });
        Self { width: self.width, height: self.height, cells: new_cells }
    }
}

struct UndoQueue<T: Clone> {
    queue: Vec<T>,
    index: usize,
}

impl<T: Clone> UndoQueue<T> {
    pub fn new(entry: T) -> Self {
        Self { queue: vec![entry], index: 0 }
    }

    pub fn current(&self) -> T {
        self.queue[self.index].clone()
    }

    pub fn push(&mut self, entry: T) {
        self.queue.truncate(self.index + 1);
        self.queue.push(entry);
        self.index += 1;
    }


    pub fn redo(&mut self) {
        if self.index < (self.queue.len() - 1) {
            self.index += 1;
        }
    }

    pub fn undo(&mut self) {
        if self.index >= 1 {
            self.index -= 1;
        }
    }
}

#[wasm_bindgen]
struct InternalState {
    undo_queue: UndoQueue<Image>
}

#[wasm_bindgen]
impl InternalState {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Self {
        Self { undo_queue: UndoQueue::new(Image::new(width, height)) }
    }

    pub fn image(&self) -> Image {
        self.undo_queue.current()
    }

    pub fn brush(&mut self, x: usize, y: usize, color: Vec<u8>) {
        let image = self.undo_queue.current();
        let new_image = image.brush(x, y, color);
        self.undo_queue.push(new_image);
    }

    pub fn redo(&mut self) {
        self.undo_queue.redo()
    }

    pub fn undo(&mut self) {
        self.undo_queue.undo()
    }
 }