use std::i64;

#[derive(Copy, Clone, PartialEq)]
enum TileType {
    START,
    END,
    NORMAL
}

#[derive(Copy, Clone)]
struct Tile {
    visited: usize,
    tile_type: TileType
}

struct Position {
    x: usize,
    y: usize
}

fn main() {
    let fingerprint = "fc:94:b0:c1:e5:b0:98:7c:58:43:99:76:97:ee:9f:b7";

    let symbols: [char; 15] = [' ', '.', 'o', '+', '=', '*', 'B', 'O', 'X', '@', '%', '&', '#', '/', '^'];
    let mut floor: [[Tile; 9]; 17] = [[Tile{visited: 0, tile_type: TileType::NORMAL}; 9]; 17];
    let mut pos = Position{x: 8, y: 4};
    floor[pos.x][pos.y].tile_type = TileType::START;

    let byte_strs = fingerprint.split(':');

    for str in byte_strs {
        let deci = i64::from_str_radix(str, 16).expect("not a hex value");
        let binary_str = format!("{deci:08b}");
        let binary_str_bytes = binary_str.as_bytes();

        for i in (0..4).rev() {
            let f = binary_str_bytes[i*2] as char;
            let s = binary_str_bytes[i*2+1] as char;

            if f == '0' && s == '0' { //top left
                if pos.y != 0 {
                    pos.y -= 1;
                }
                if pos.x != 0 {
                    pos.x -= 1;
                }
            } else if f == '0' && s == '1' { //top right
                if pos.y != 0 {
                    pos.y -= 1;
                }
                if pos.x != 16 {
                    pos.x += 1;
                }
            } else if f == '1' && s == '0' { //bottom left
                if pos.y != 8 {
                    pos.y += 1;
                }
                if pos.x != 0 {
                    pos.x -= 1;
                }
            } else { //11 bottom right
                if pos.y != 8 {
                    pos.y += 1;
                }
                if pos.x != 16 {
                    pos.x += 1;
                }
            }
            floor[pos.x][pos.y].visited += 1;
        }
    }

    floor[pos.x][pos.y].tile_type = TileType::END;

    println!("+---[CODINGAME]---+");
    for j in 0..9 {
        let mut str = String::from("|");
        for i in 0..17 {
            if floor[i][j].tile_type == TileType::START {
                str += "S";
            } else if floor[i][j].tile_type == TileType::END {
                str += "E";
            } else {
                str += symbols[floor[i][j].visited % 15].to_string().as_str();
            };
        }
        println!("{str}|");
    }
    println!("+-----------------+");

}
