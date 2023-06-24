// A WebAssembly implementation of Conway's Game of Life.

use wasm_bindgen::{prelude::*, JsCast};

const NUM_CELLS_HORIZONTAL: usize = 100;
const NUM_CELLS_VERTICAL: usize = 100;
const SIZE_CELL: usize = 10;
const TIME_STEP: f64 = 100.0;

#[derive(PartialEq)]
enum CellState {
    Live,
    Dead,
}

struct Cell {
    x: f64,
    y: f64,
    state: CellState,
}

impl Cell {
    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        match self.state {
            CellState::Live => {
                context.set_fill_style(&JsValue::from_str("rgb(0,0,0)"));
            }
            CellState::Dead => {
                context.set_fill_style(&JsValue::from_str("rgb(255,255,255)"));
            }
        }
        context.fill_rect(self.x, self.y, SIZE_CELL as f64, SIZE_CELL as f64);
        // context.stroke_rect(self.x, self.y, SIZE_CELL as f64, SIZE_CELL as f64);
    }
}

struct Data {
    cells: Vec<Cell>,
    context: web_sys::CanvasRenderingContext2d,
    request_animation_frame_handle: gloo_render::AnimationFrame,
    timestamp: f64,
}

static mut DATA: Option<Data> = None;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have window");
    let document = window.document().expect("should have window");

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_width((NUM_CELLS_HORIZONTAL * SIZE_CELL) as u32);
    canvas.set_height((NUM_CELLS_VERTICAL * SIZE_CELL) as u32);
    document.body().unwrap().append_child(&canvas)?;

    let context = canvas
        .get_context("2d")?
        .expect("should have 2d context")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let mut cells = Vec::with_capacity(NUM_CELLS_HORIZONTAL * NUM_CELLS_VERTICAL);
    for i in 0..NUM_CELLS_HORIZONTAL {
        for j in 0..NUM_CELLS_VERTICAL {
            cells.push(Cell {
                x: (i * SIZE_CELL) as f64,
                y: (j * SIZE_CELL) as f64,
                state: if js_sys::Math::random() >= 0.5 {
                    CellState::Live
                } else {
                    CellState::Dead
                },
            });
        }
    }
    draw(&cells, &context);

    let request_animation_frame_handle = gloo_render::request_animation_frame(on_animation_frame);

    unsafe {
        DATA = Some(Data {
            cells,
            context,
            request_animation_frame_handle,
            timestamp: 0.0,
        });
    }

    Ok(())
}

fn draw(cells: &[Cell], context: &web_sys::CanvasRenderingContext2d) {
    cells.iter().for_each(|cell| cell.draw(context));
}

fn count_neighbours(cells: &[Cell], idx: usize) -> usize {
    let mut result = 0;
    let (x, y) = (idx % NUM_CELLS_HORIZONTAL, idx / NUM_CELLS_HORIZONTAL);
    for i in (if x > 0 { x - 1 } else { x })..=(if x < NUM_CELLS_HORIZONTAL - 1 {
        x + 1
    } else {
        x
    }) {
        for j in
            (if y > 0 { y - 1 } else { y })..=(if y < NUM_CELLS_VERTICAL - 1 { y + 1 } else { y })
        {
            let idx_neighbour = j * NUM_CELLS_HORIZONTAL + i;
            if idx_neighbour != idx && cells[idx_neighbour].state == CellState::Live {
                result += 1;
            }
        }
    }
    result
}

fn update(cells: &[Cell]) -> Vec<Cell> {
    cells
        .iter()
        .enumerate()
        .map(|(idx, cell)| Cell {
            // Update cell state using the condensed rules of Conway's Game of Life
            state: match cell.state {
                CellState::Live => match count_neighbours(cells, idx) {
                    2 | 3 => CellState::Live,
                    _ => CellState::Dead,
                },
                CellState::Dead => match count_neighbours(cells, idx) {
                    3 => CellState::Live,
                    _ => CellState::Dead,
                },
            },
            // Copy other cell values
            ..*cell
        })
        .collect()
}

fn on_animation_frame(timestamp: f64) {
    let data = unsafe { DATA.as_mut().unwrap() };

    if timestamp - data.timestamp >= TIME_STEP {
        data.timestamp = timestamp;
        data.cells = update(&data.cells);
        draw(&data.cells, &data.context);
    }

    data.request_animation_frame_handle = gloo_render::request_animation_frame(on_animation_frame);
}
