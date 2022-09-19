pub enum LayoutDirection {
    UP,
    RIGHT,
}

pub struct LayoutOptions {
    pub graph_direction: LayoutDirection,
    pub commit_hist_dist: i32,
    pub branch_dist: i32,
    pub commit_radius: i32,
}

impl LayoutOptions {
    pub fn default() -> LayoutOptions {
        LayoutOptions {
            graph_direction: LayoutDirection::UP,
            commit_hist_dist: 3,
            branch_dist: 2,
            commit_radius: 1,
        }
    }
}
