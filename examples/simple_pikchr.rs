use git_graph_to_svg::options::layout::LayoutOptions;
use git_graph_to_svg::{parse_graph, print_pikchr};

fn main() {
    // Create the model
    let model = parse_graph("main1:A B C D\nmain2:<E F G>".to_string()).unwrap();
    println!(
        "{}",
        print_pikchr(&model, &LayoutOptions::default()).unwrap()
    );
}
