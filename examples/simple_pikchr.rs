use git_graph_to_pikchr::{parse_graph, print_pikchr};

fn main() {
// Create the model
    let model = parse_graph("main: A".to_string()).unwrap();
    println!(print_pikchr(&model, LayoutOptions::default()));
}
