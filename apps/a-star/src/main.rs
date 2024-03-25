mod breadth_first_search;
mod dijkstra_search;
mod utils {
    pub mod common;
    pub mod constants;
}

fn main() {
    breadth_first_search::main();
    dijkstra_search::main();
}
