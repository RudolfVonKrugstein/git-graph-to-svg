struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

enum CommitForm {
    Circle,
    Box,
    Triangle,
}

/** Style of git diagrams, like colors, line style e.t.c.*/
struct Style {
    pub commit_form: CommitForm,
    pub fill_color: Color,
    pub outline_color: Color,
}