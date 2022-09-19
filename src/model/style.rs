struct RGBA {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

pub struct Style {
    commit_fill_color: RGBA,
    commit_outline_color: RGBA,
    commit_text_color: RGBA,
    parent_color: RGBA,
    branch_fill_color: RGBA,
    branch_outline_color: RGBA,
    branch_line_color: RGBA,
    branch_text_color: RGBA,
}

impl Style {
    pub fn default() -> Style {
        Style {
            commit_fill_color: RGBA {
                red: 0xFF,
                green: 0xFF,
                blue: 0xFF,
                alpha: 0xFF,
            },
            commit_outline_color: RGBA {
                red: 0x00,
                green: 0x00,
                blue: 0x00,
                alpha: 0xFF,
            },
            commit_text_color: RGBA {
                red: 0x00,
                green: 0x00,
                blue: 0x00,
                alpha: 0xFF,
            },
            parent_color: RGBA {
                red: 0x00,
                green: 0x00,
                blue: 0x00,
                alpha: 0xFF,
            },
            branch_fill_color: RGBA {
                red: 0xFF,
                green: 0xFF,
                blue: 0xFF,
                alpha: 0xFF,
            },
            branch_outline_color: RGBA {
                red: 0x00,
                green: 0x00,
                blue: 0x00,
                alpha: 0xFF,
            },
            branch_line_color: RGBA {
                red: 0x00,
                green: 0x00,
                blue: 0x00,
                alpha: 0xFF,
            },
            branch_text_color: RGBA {
                red: 0x00,
                green: 0x00,
                blue: 0x00,
                alpha: 0xFF,
            },
        }
    }
}
