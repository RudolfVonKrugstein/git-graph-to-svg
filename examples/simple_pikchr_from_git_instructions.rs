use git_graph_to_pikchr::{parse_git_instructions, print_pikchr};
use git_graph_to_pikchr::model::View;
use git_graph_to_pikchr::options::layout::LayoutOptions;

fn main() {
// Create the model
    let state = parse_git_instructions(
        r###"
        branch(main)
        commit(A)
        commit(B)
        branch(feature/x)
        commit(C)
        checkout(feature/x)
        commit(D)
        checkout(main)
        commit(E)
        merge(F,feature/x)
        "###
    ).unwrap();
    let view = View::from_state(&state);
    println!("{}", print_pikchr(&view, &LayoutOptions::default()).unwrap());
}
