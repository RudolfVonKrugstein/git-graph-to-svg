use git_graph_to_pikchr::{parse_graph, print_pikchr};
use git_graph_to_pikchr::options::layout::LayoutOptions;

fn main() {
// Create the model
    let model = parse_graph("main1:A B C D\nmain2:<E F G>".to_string()).unwrap();
    println!("{}", print_pikchr(&model, &LayoutOptions::default()).unwrap());
}
