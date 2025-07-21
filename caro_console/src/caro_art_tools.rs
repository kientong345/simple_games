// DO NOT TRY TO READ THIS FILE, EVEN I CANT UNDERSTAND IT PROPERLY!

use std::usize;

use crate::output::DrawableBox;
use crate::artworks;

pub type VerticalRange = (usize, usize);
pub type HorizontalRange = (usize, usize);

pub type Coordinate = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub struct BoardPosition {
    pub base: Coordinate,
    pub vertical_range: VerticalRange,
    pub horizontal_range: HorizontalRange,
}

pub fn get_drawable_coordinate_layout(board_position: &BoardPosition)
-> Vec<DrawableBox> {
    let num_cols = board_position.horizontal_range.1 - board_position.horizontal_range.0 + 1;
    let num_rows = board_position.vertical_range.1 - board_position.vertical_range.0 + 1;
    let max_width = artworks::TILE_WIDTH + num_cols * artworks::TILE_WIDTH;
    let max_height = artworks::TILE_HEIGHT + num_rows * artworks::TILE_HEIGHT;
    let estimated_capacity = max_width * max_height + max_height;
    let mut art = String::with_capacity(estimated_capacity);
    let mut x_axis_line = String::with_capacity(max_width);
    x_axis_line.push_str(&" ".repeat(artworks::TILE_WIDTH));

    for longitude in board_position.horizontal_range.0..= board_position.horizontal_range.1 {
        let s = longitude.to_string();
        let digit_num = s.len();
        let front_align_len = (artworks::TILE_WIDTH - digit_num) / 2;
        let back_align_len = artworks::TILE_WIDTH - digit_num - front_align_len;
        x_axis_line.push_str(&" ".repeat(front_align_len));
        x_axis_line.push_str(&s);
        x_axis_line.push_str(&" ".repeat(back_align_len));
    }
    art.push_str(&x_axis_line);
    art.push('\n');

    for _ in 1..artworks::TILE_HEIGHT {
        art.push_str(&" ".repeat(max_width));
        art.push('\n');
    }

    for latitude in board_position.vertical_range.0..= board_position.vertical_range.1 {
        let s = latitude.to_string();
        let digit_num = s.len();
        // this code suck, if you try to read it, then get the fk up
        let front_align_len: usize;
        let align_thing: bool; // dont try to understand!, i've just tried to deal with some weird edge cases
        if digit_num >= 4 {
            front_align_len = artworks::TILE_WIDTH - digit_num;
            align_thing = false;
        } else {
            front_align_len = artworks::TILE_WIDTH - digit_num - 1;
            align_thing = true;
        }
        art.push_str(&" ".repeat(front_align_len));
        art.push_str(&s);
        if align_thing {
            art.push(' ');
        }
        art.push_str(&" ".repeat(num_cols * artworks::TILE_WIDTH));
        art.push('\n');

        for _ in 1..artworks::TILE_HEIGHT {
            art.push_str(&" ".repeat(artworks::TILE_WIDTH));
            art.push_str(&" ".repeat(num_cols * artworks::TILE_WIDTH));
            art.push('\n');
        }
    }

    let layout = DrawableBox {
        coordinate: (board_position.base.0, board_position.base.1),
        constraint: (max_height, max_width),
        offset: (0, 0),
        show_boundary_line: false,
        art,
    };

    // Build tile net
    let mut tile_net_art = String::with_capacity(estimated_capacity);
    let odd_line = format!("+{}+\n", "---+".repeat(num_cols));
    let even_line = format!("|{}|\n", "   |".repeat(num_cols));
    
    tile_net_art.push_str(&odd_line);
    for _ in 0..num_rows {
        tile_net_art.push_str(&even_line);
        tile_net_art.push_str(&odd_line);
    }

    let tile_net = DrawableBox {
        coordinate: (board_position.base.0+2, board_position.base.1+5),
        constraint: (max_height-1-artworks::TILE_HEIGHT, max_width-1-artworks::TILE_WIDTH),
        offset: (1, 1),
        show_boundary_line: true,
        art: tile_net_art,
    };

    vec![layout, tile_net]
}

fn board_to_screen_coordinate(board_position: &BoardPosition, board_coordinate: Coordinate)
-> Coordinate {
    (
        board_position.base.0 + (board_coordinate.0-board_position.vertical_range.0+1)*artworks::TILE_HEIGHT - 1,
        board_position.base.1 + (board_coordinate.1-board_position.horizontal_range.0+1)*artworks::TILE_WIDTH,
    )
}

pub fn get_drawable_cursor(board_position: &BoardPosition, board_coordinate: Coordinate)
-> DrawableBox {
    let coordinate = board_to_screen_coordinate(board_position, board_coordinate);
    DrawableBox {
        coordinate,
        constraint: (artworks::TILE_HEIGHT+1, artworks::TILE_WIDTH+1),
        offset: (0, 0),
        show_boundary_line: false,
        art: artworks::CARO_TILE.to_string(),
    }
}

pub fn get_drawable_x_moves(board_position: &BoardPosition, moves_set: Vec<Coordinate>)
-> Vec<DrawableBox> {
    let mut drawable_set = Vec::new();
    for (latitude, longtitude) in moves_set {
        if latitude >= board_position.vertical_range.0 && latitude <= board_position.vertical_range.1 &&
            longtitude >= board_position.horizontal_range.0 && longtitude <= board_position.horizontal_range.1 {
            let (latitude, longtitude) = board_to_screen_coordinate(board_position, (latitude, longtitude));
            drawable_set.push(
                DrawableBox {
                    coordinate: (latitude+1, longtitude+2),
                    constraint: (1, 1),
                    offset: (0, 0),
                    show_boundary_line: false,
                    art: artworks::X_SYM.to_string(),
                }
            );
        }
    }
    drawable_set
}

pub fn get_drawable_o_moves(board_position: &BoardPosition, moves_set: Vec<Coordinate>)
-> Vec<DrawableBox> {
    let mut drawable_set = Vec::new();
    for (latitude, longtitude) in moves_set {
        if latitude >= board_position.vertical_range.0 && latitude <= board_position.vertical_range.1 &&
            longtitude >= board_position.horizontal_range.0 && longtitude <= board_position.horizontal_range.1 {
            let (latitude, longtitude) = board_to_screen_coordinate(board_position, (latitude, longtitude));
            drawable_set.push(
                DrawableBox {
                    coordinate: (latitude+1, longtitude+2),
                    constraint: (1, 1),
                    offset: (0, 0),
                    show_boundary_line: false,
                    art: artworks::O_SYM.to_string(),
                }
            );
        }
    }
    drawable_set
}