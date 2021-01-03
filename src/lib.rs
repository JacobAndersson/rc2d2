extern crate cpython;
use cpython::{py_fn, py_module_initializer, PyResult, Python};
mod eval;
mod search;
#[allow(unused)]
mod utils;

py_module_initializer!(rc2d2, |py, m| {
    m.add(py, "__doc__", "Chess engine in rust")?;
    m.add(
        py,
        "find_best_move",
        py_fn!(py, find_best_move(uci_moves: &str, depth: u8)),
    )?;
    Ok(())
});

fn find_best_move(_py: Python, moves: &str, depth: u8) -> PyResult<String> {
    let mv = utils::find_best_move(moves, depth);
    return Ok(mv);
}
