// DO NOT TRY TO READ THIS FILE, EVEN I CANT UNDERSTAND IT PROPERLY!

use std::usize;

use crate::DrawableBox;
use crate::artworks;

pub type VerticalRange = (usize, usize);
pub type HorizontalRange = (usize, usize);

pub type Coordinate = (usize, usize);

pub struct Cursor {
	pub coordinate_on_board: (i32, i32),
	pub coordinate_on_screen: (i32, i32),
}

pub fn get_drawable_coordinate_layout(vertical_range: VerticalRange, horizontal_range: HorizontalRange)
-> DrawableBox {
    let num_cols = horizontal_range.1 - horizontal_range.0 + 1;
    let num_rows = vertical_range.1 - vertical_range.0 + 1;
    let max_width = artworks::TILE_WIDTH + num_cols * artworks::TILE_WIDTH;
    let max_height = artworks::TILE_HEIGHT + num_rows * artworks::TILE_HEIGHT;
    let estimated_capacity = max_width * max_height + max_height;
    let mut art = String::with_capacity(estimated_capacity);
    let mut x_axis_line = String::with_capacity(max_width);
    x_axis_line.push_str(&" ".repeat(artworks::TILE_WIDTH));

    for longitude in horizontal_range.0..=horizontal_range.1 {
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

    for latitude in vertical_range.0..=vertical_range.1 {
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

    DrawableBox {
        constraint: (max_height, max_width),
        offset: (0, 0),
        show_boundary_line: false,
        art,
    }
}

pub fn get_drawable_caro_board(
    player1_moves: Vec<Coordinate>, player2_moves: Vec<Coordinate>,
    vertical_range: VerticalRange, horizontal_range: HorizontalRange)
-> DrawableBox {
    // let num_cols = horizontal_range.1 - horizontal_range.0 + 1;
    // let num_rows = vertical_range.1 - vertical_range.0 + 1;
    // let max_width = num_cols * artworks::TILE_WIDTH;
    // let max_height = num_rows * artworks::TILE_HEIGHT;
    // let mut art = String::new();

    // let mut odd_line = String::new();
    // odd_line.push('+');
    // odd_line.push_str(&"---+".repeat(num_cols));
    // odd_line.push('\n');
    // let mut even_line = String::new();
    // even_line.push('|');
    // even_line.push_str(&"   |".repeat(num_cols));
    // even_line.push('\n');

    // art.push_str(&odd_line);
    // for _ in 0..num_rows {
    //     art.push_str(&even_line);
    //     art.push_str(&odd_line);
    // }

    // let get_pos_from_caro_pos = |coor: Coordinate| -> usize {
    //     todo!()
    // };

    // for coor in player1_moves {
    //     art.get(get_pos_from_caro_pos(coor)) = 'X';
    // }
    // for coor in player2_moves {
    //     art.get(get_pos_from_caro_pos(coor)) = 'O';
    // }

    // DrawableBox {
    //     constraint: (max_height-1, max_width-1),
    //     offset: (1, 1),
    //     show_boundary_line: true,
    //     art,
    // }

    let num_cols = horizontal_range.1 - horizontal_range.0 + 1;
    let num_rows = vertical_range.1 - vertical_range.0 + 1;

    // Calculate actual dimensions of the drawn board
    // Each column is TILE_WIDTH characters wide, plus 1 for the rightmost '|' or '+'
    let board_width_chars = artworks::TILE_WIDTH * num_cols + 1;
    // Each row is TILE_HEIGHT characters tall, plus 1 for the bottommost '+' line
    let board_height_lines = artworks::TILE_HEIGHT * num_rows + 1;

    // Create a 2D grid (conceptually) using a Vec<char> to allow easy modification
    // Each line in the final string will have `board_width_chars` characters + newline
    let line_len_with_newline = board_width_chars + 1; // +1 for '\n'
    let mut char_grid: Vec<char> = Vec::with_capacity(board_height_lines * line_len_with_newline);

    // --- 1. Draw the empty grid structure ---

    // Build the horizontal separator line (e.g., "+---+---+")
    let mut horizontal_sep_line = String::with_capacity(board_width_chars);
    horizontal_sep_line.push('+');
    for _ in 0..num_cols {
        horizontal_sep_line.push_str(&"-".repeat(artworks::TILE_WIDTH-1));
        horizontal_sep_line.push('+');
    }
    horizontal_sep_line.push('\n'); // Add newline here

    // Build the empty content line (e.g., "|   |   |")
    let mut empty_content_line = String::with_capacity(board_width_chars);
    empty_content_line.push('|');
    for _ in 0..num_cols {
        empty_content_line.push_str(&" ".repeat(artworks::TILE_WIDTH-1));
        empty_content_line.push('|');
    }
    empty_content_line.push('\n'); // Add newline here

    // Append the lines to the char_grid
    for _ in 0..num_rows {
        char_grid.extend(horizontal_sep_line.chars());
        char_grid.extend(empty_content_line.chars());
    }
    // Add the final horizontal separator line
    char_grid.extend(horizontal_sep_line.chars());


    // --- 2. Implement get_char_index for mapping Coordinates to char_grid indices ---
    // This is the core logic for placing X/O.
    // Given a (row, col) from `player_moves` (0-indexed relative to range start),
    // calculate its position in the flat `char_grid`.

    let char_grid_len = char_grid.len();

    let get_char_index = |coord: Coordinate| -> Option<usize> {
        let r = coord.0; // Row index (relative to game board)
        let c = coord.1; // Column index (relative to game board)

        // Check if coordinate is within the drawing range
        if r < vertical_range.0 || r > vertical_range.1 ||
           c < horizontal_range.0 || c > horizontal_range.1 {
            return None; // Coordinate is outside the drawable range
        }

        // Adjust coordinates to be 0-indexed relative to the drawn board's top-left corner
        let drawn_row = r - vertical_range.0;
        let drawn_col = c - horizontal_range.0;

        // Calculate the starting line index for this row's content
        // Each full row block (like '+---+\n|   |\n') takes TILE_HEIGHT lines
        // plus one for the top border.
        let line_offset = (drawn_row * artworks::TILE_HEIGHT) + 1; // +1 to skip the initial board border

        // Calculate the starting column index for this cell's content
        // Each column is TILE_WIDTH characters wide ('| X ').
        // We want the middle of the cell, so +2 from the start of the cell block.
        // E.g., for "| X |", 'X' is at index 2 (0-indexed).
        let col_offset = (drawn_col * artworks::TILE_WIDTH) + (artworks::TILE_WIDTH / 2);

        // Calculate the flat index in `char_grid`
        // (line_index * characters_per_line_including_newline) + column_index_within_line
        let index = (line_offset * line_len_with_newline) + col_offset;

        // Ensure the calculated index is within the bounds of char_grid
        if index < char_grid_len {
            Some(index)
        } else {
            None
        }
    };


    // --- 3. Place player moves onto the grid ---

    for coor in player1_moves {
        if let Some(idx) = get_char_index(coor) {
            char_grid[idx] = 'X';
        }
    }
    for coor in player2_moves {
        if let Some(idx) = get_char_index(coor) {
            char_grid[idx] = 'O';
        }
    }

    // --- 4. Convert char_grid to String ---
    let art: String = char_grid.into_iter().collect();

    DrawableBox {
        // Constraint should reflect the actual generated dimensions
        constraint: (board_height_lines-2, board_width_chars-2),
        offset: (1, 1), // Default to (0,0) unless specific offset is needed for drawing context
        show_boundary_line: true,
        art,
    }
}
