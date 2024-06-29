use piston_window::*;
use std::time::{Duration, Instant};
//use rand::prelude::*;

const PIECES: [(&'static str, [Block; 4]); 7] = [
    ("square1", [
        Block {pos: Pos { x: 5, y: 0 } },
        Block {pos: Pos { x: 5, y: 1 } },
        Block {pos: Pos { x: 6, y: 0 } },
        Block {pos: Pos { x: 6, y: 1 } },
    ]),

    ("left1", [
        Block {pos: Pos { x: 5, y: 0 } },
        Block {pos: Pos { x: 5, y: 1 } },
        Block {pos: Pos { x: 6, y: 1 } },
        Block {pos: Pos { x: 7, y: 1 } },
    ]),

    ("light1", [
        Block {pos: Pos { x: 4, y: 1 } },
        Block {pos: Pos { x: 5, y: 1 } },
        Block {pos: Pos { x: 6, y: 1 } },
        Block {pos: Pos { x: 6, y: 0 } },
    ]),

    ("zig1", [
        Block {pos: Pos { x: 5, y: 1 } },
        Block {pos: Pos { x: 6, y: 1 } },
        Block {pos: Pos { x: 6, y: 0 } },
        Block {pos: Pos { x: 7, y: 0 } },
    ]),

    ("zag1", [
        Block {pos: Pos { x: 4, y: 0 } },
        Block {pos: Pos { x: 5, y: 0 } },
        Block {pos: Pos { x: 5, y: 1 } },
        Block {pos: Pos { x: 6, y: 1 } },
    ]),

    ("pole1", [
        Block {pos: Pos { x: 4, y: 0 } },
        Block {pos: Pos { x: 5, y: 0 } },
        Block {pos: Pos { x: 6, y: 0 } },
        Block {pos: Pos { x: 7, y: 0 } },
    ]),

    ("tee1", [
        Block {pos: Pos { x: 4, y: 1 } },
        Block {pos: Pos { x: 5, y: 1 } },
        Block {pos: Pos { x: 5, y: 0 } },
        Block {pos: Pos { x: 6, y: 1 } },
    ])
    // Add other pieces here...
];

#[derive(Clone)]
struct World {
    blocks: Vec<Block>,
}

#[derive(Clone)]
struct Pos {
    x: i16,
    y: i16,
}

#[derive(Clone)]
struct Block {
    //me_piece: bool,
    pos: Pos,
    // a 1x1 square
}

#[derive(Clone)]
struct Piece {
    variant: &'static str,
    blocks: [Block; 4],
    //rotation_style: &str,
    // Piece is 4 blocks
}

fn update(mut piece: Piece) -> Piece {
    for block in piece.blocks.iter_mut() {
        block.pos.y += 1;
    }
    piece
}

fn update_game(mut game: World, mut piece: Piece, mut next_piece: Piece, all_pieces: [Piece; 7]) -> (World, Piece, Piece) {
    let mut fallen = false;
    for block in piece.clone().blocks.iter() {
        for ground in game.blocks.iter() {
            if block.pos.y == ground.pos.y && block.pos.x == ground.pos.x {
                fallen = true;
                break;
            }
            if block.pos.y == 19 {
                fallen = true;
                break;
            }
        }
    }
    if fallen {
        for block in piece.clone().blocks.iter_mut() {
            block.pos.y -= 1;
            game.blocks.push(block.clone());
        }
        piece = next_piece;
        next_piece = all_pieces[(rand::random::<u8>()%7) as usize].clone();
    } else {
        piece = update(piece);
        let mut fallen = false;
        for block in piece.clone().blocks.iter() {
            for ground in game.blocks.iter() {
                if block.pos.y == ground.pos.y && block.pos.x == ground.pos.x {
                    fallen = true;
                    break;
                }
            }
            if block.pos.y == 19 {
                fallen = true;
                break;
            }
        }
        if fallen {
            for block in piece.clone().blocks.iter_mut() {
                block.pos.y -= 1;
                game.blocks.push(block.clone());
            }
            piece = next_piece;
            next_piece = all_pieces[(rand::random::<u8>()%7) as usize].clone();
        }
    }
    (game, piece, next_piece)
}

fn create_pieces() -> [Piece; 7] {
    PIECES.map(|(variant, blocks)| Piece {
        variant: variant,
        blocks,
    })
}

fn move_left(mut piece: Piece, game: World) -> Piece {
    for block in piece.blocks.iter_mut() {
        if block.pos.x > 0 {
            block.pos.x -= 1;
        }
    }
    let mut overlap = false;
    for block in piece.clone().blocks.iter() {
        for ground in game.blocks.iter() {
            if block.pos.y == ground.pos.y && block.pos.x == ground.pos.x {
                overlap = true;
                break;
            }
        }
        if block.pos.x == 0 {
            overlap = true;
            break;
        }
    }
    if overlap {
        piece = move_right(piece, game);
    }
    piece
}

fn move_right(mut piece: Piece, game: World) -> Piece {
    for block in piece.blocks.iter_mut() {
        block.pos.x += 1;
    }
    let mut overlap = false;
    for block in piece.clone().blocks.iter() {
        for ground in game.blocks.iter() {
            if block.pos.y == ground.pos.y && block.pos.x == ground.pos.x {
                overlap = true;
                break;
            }
        }
        if block.pos.x == 11 {
            overlap = true;
            break;
        }
    }
    if overlap {
        piece = move_left(piece, game);
    }
    piece
}

fn turn_clock(mut piece: Piece) -> Piece {
    match piece.variant {
        "square1" => {
        }
        "left1" => {
            piece.blocks[0].pos.x += 2;
            piece.blocks[1].pos.x += 1;
            piece.blocks[1].pos.y -= 1;
            piece.blocks[3].pos.x -= 1;
            piece.blocks[3].pos.y += 1;
            piece.variant = "left2";
        }
        "left2" => {
            piece.blocks[0].pos.y += 2;
            piece.blocks[1].pos.x += 1;
            piece.blocks[1].pos.y += 1;
            piece.blocks[3].pos.x -= 1;
            piece.blocks[3].pos.y -= 1;
            piece.variant = "left3";
        }
        "left3" => {
            piece.blocks[0].pos.x -= 2;
            piece.blocks[1].pos.x -= 1;
            piece.blocks[1].pos.y += 1;
            piece.blocks[3].pos.x += 1;
            piece.blocks[3].pos.y -= 1;
            piece.variant = "left4";
        }
        "left4" => {
            piece.blocks[0].pos.y -= 2;
            piece.blocks[1].pos.x -= 1;
            piece.blocks[1].pos.y -= 1;
            piece.blocks[3].pos.x += 1;
            piece.blocks[3].pos.y += 1;
            piece.variant = "left1";
        }
        "light1" => {
            piece.blocks[0].pos.x += 1;
            piece.blocks[0].pos.y -= 1;
            piece.blocks[2].pos.x -= 1;
            piece.blocks[2].pos.y += 1;
            piece.blocks[3].pos.y += 2;
            piece.variant = "light2";
        }
        "light2" => {
            piece.blocks[0].pos.x += 1;
            piece.blocks[0].pos.y += 1;
            piece.blocks[2].pos.x -= 1;
            piece.blocks[2].pos.y -= 1;
            piece.blocks[3].pos.x -= 2;
            piece.variant = "light3";
        }
        "light3" => {
            piece.blocks[0].pos.x -= 1;
            piece.blocks[0].pos.y += 1;
            piece.blocks[2].pos.x += 1;
            piece.blocks[2].pos.y -= 1;
            piece.blocks[3].pos.y -= 2;
            piece.variant = "light4";
        }
        "light4" => {
            piece.blocks[0].pos.x -= 1;
            piece.blocks[0].pos.y -= 1;
            piece.blocks[2].pos.x += 1;
            piece.blocks[2].pos.y += 1;
            piece.blocks[3].pos.x += 2;
            piece.variant = "light1";
        }
        "zig1" => {
            piece.blocks[0].pos.y -= 2;
            piece.blocks[1].pos.x -= 1;
            piece.blocks[1].pos.y -= 1;
            piece.blocks[3].pos.x -= 1;
            piece.blocks[3].pos.y += 1;
            piece.variant = "zig2";
        }
        "zig2" => {
            piece.blocks[0].pos.x += 2;
            piece.blocks[1].pos.x += 1;
            piece.blocks[1].pos.y -= 1;
            piece.blocks[3].pos.x -= 1;
            piece.blocks[3].pos.y -= 1;
            piece.variant = "zig3";
        }
        "zig3" => {
            piece.blocks[0].pos.y += 2;
            piece.blocks[1].pos.x += 1;
            piece.blocks[1].pos.y += 1;
            piece.blocks[3].pos.x += 1;
            piece.blocks[3].pos.y -= 1;
            piece.variant = "zig4";
        }
        "zig4" => {
            piece.blocks[0].pos.x -= 2;
            piece.blocks[1].pos.x -= 1;
            piece.blocks[1].pos.y += 1;
            piece.blocks[3].pos.x += 1;
            piece.blocks[3].pos.y += 1;
            piece.variant = "zig1";
        }
        "zag1" => {
            piece.blocks[0].pos.x += 2;
            piece.blocks[1].pos.x += 1;
            piece.blocks[1].pos.y += 1;
            piece.blocks[3].pos.x -= 1;
            piece.blocks[3].pos.y += 1;
            piece.variant = "zag2";
        }
        "zag2" => {
            piece.blocks[0].pos.y += 2;
            piece.blocks[1].pos.x -= 1;
            piece.blocks[1].pos.y += 1;
            piece.blocks[3].pos.x -= 1;
            piece.blocks[3].pos.y -= 1;
            piece.variant = "zag3";
        }
        "zag3" => {
            piece.blocks[0].pos.x -= 2;
            piece.blocks[1].pos.x -= 1;
            piece.blocks[1].pos.y -= 1;
            piece.blocks[3].pos.x += 1;
            piece.blocks[3].pos.y -= 1;
            piece.variant = "zag4";
        }
        "zag4" => {
            piece.blocks[0].pos.y -= 2;
            piece.blocks[1].pos.x += 1;
            piece.blocks[1].pos.y -= 1;
            piece.blocks[3].pos.x += 1;
            piece.blocks[3].pos.y += 1;
            piece.variant = "zag1";
        }
        "pole1" => {
            piece.blocks[0].pos.x += 2;
            piece.blocks[0].pos.y -= 1;
            piece.blocks[1].pos.x += 1;
            piece.blocks[2].pos.y += 1;
            piece.blocks[3].pos.x -= 1;
            piece.blocks[3].pos.y += 2;
            piece.variant = "pole2";
        }
        "pole2" => {
            piece.blocks[0].pos.x += 1;
            piece.blocks[0].pos.y += 2;
            piece.blocks[1].pos.y += 1;
            piece.blocks[2].pos.x -= 1;
            piece.blocks[3].pos.x -= 2;
            piece.blocks[3].pos.y -= 1;
            piece.variant = "pole3";
        }
        "pole3" => {
            piece.blocks[0].pos.x -= 2;
            piece.blocks[0].pos.y += 1;
            piece.blocks[1].pos.x -= 1;
            piece.blocks[2].pos.y -= 1;
            piece.blocks[3].pos.x += 1;
            piece.blocks[3].pos.y -= 2;
            piece.variant = "pole4";
        }
        "pole4" => {
            piece.blocks[0].pos.x -= 1;
            piece.blocks[0].pos.y -= 2;
            piece.blocks[1].pos.y -= 1;
            piece.blocks[2].pos.x += 1;
            piece.blocks[3].pos.x += 2;
            piece.blocks[3].pos.y += 1;
            piece.variant = "pole1";
        }
        "tee1" => {
            piece.blocks[0].pos.x += 1;
            piece.blocks[0].pos.y += 1;
            piece.variant = "tee2";
        }
        "tee2" => {
            piece.blocks[0].pos.x -= 1;
            piece.blocks[0].pos.y -= 1;
            piece.blocks[2].pos.y += 2;
            piece.variant = "tee3";
        }
        "tee3" => {
            piece.blocks[3].pos.x -= 1;
            piece.blocks[3].pos.y -= 1;
            piece.variant = "tee4";
        }
        "tee4" => {
            piece.blocks[3].pos.x += 1;
            piece.blocks[3].pos.y += 1;
            piece.blocks[2].pos.y -= 2;
            piece.variant = "tee1";
        }
        _ => {
        }
    }
    piece
}

fn turn_left(mut piece: Piece, game: World) -> Piece {
    piece = turn_clock(turn_clock(turn_clock(piece.clone())));

    let mut overlap = false;
    for block in piece.clone().blocks.iter() {
        for ground in game.blocks.iter() {
            if block.pos.y == ground.pos.y && block.pos.x == ground.pos.x {
                overlap = true;
                break;
            }
            if block.pos.y < 0 || block.pos.x < 0 || block.pos.x > 10 {
                overlap = true;
                break;
            }
        }
        if block.pos.x == 0 {
            overlap = true;
            break;
        }
    }
    if overlap {
        piece = turn_right(piece, game);
    }
    piece
}

fn turn_right(mut piece: Piece, game: World) -> Piece {
    piece = turn_clock(piece.clone());

    let mut overlap = false;
    for block in piece.clone().blocks.iter() {
        for ground in game.blocks.iter() {
            if block.pos.y == ground.pos.y && block.pos.x == ground.pos.x {
                overlap = true;
                break;
            }
            if block.pos.y < 0 || block.pos.x < 0 || block.pos.x > 10 {
                overlap = true;
                break;
            }
        }
        if block.pos.x == 0 {
            overlap = true;
            break;
        }
    }
    if overlap {
        piece = turn_left(piece, game);
    }
    piece
}

fn main() {
    let square_size: i16 = 10;
    let mut window: PistonWindow = WindowSettings::new("Piston Window Example", [640, 480]).exit_on_esc(true).build().expect("YOU BAD AT CODE");
    let square = [0.0, 0.0, square_size.into(), square_size.into()]; // x, y, width, height
    let mut world = World {blocks: vec![]};
    
    
    let pieces = create_pieces();

    
    
    let mut piece = pieces[(rand::random::<u8>()%7) as usize].clone();
    let mut next_piece = pieces[(rand::random::<u8>()%7) as usize].clone();


    //let pieceDrops = [4, 5, 2, 5, 2, 5, 7, 2, 4, 5, 1, 7, 1, 5, 7, 3, 2, 5, 7, 7, 2, 5, 5, 6, 2, 3, 7, 3, 5, 3, 3, 5, 4, 4, 1, 2, 7, 2, 1, 1, 4, 2, 3, 5, 2, 5, 5, 6, 3, 6, 5, 1, 4, 5, 2, 4, 2, 2, 7, 7, 3, 6, 3, 1, 4, 1, 5, 3, 1, 2, 4, 4, 4, 1, 2, 2, 7, 2, 6, 1, 1, 7, 6, 6, 3, 6, 1, 6, 7, 1, 5, 7, 2, 6, 5, 6, 4, 7, 7, 6, 4, 3, 5, 5, 7, 6, 5, 3, 5, 3, 3, 2, 6, 7, 6, 4, 5, 7, 5, 2, 1, 7, 6, 3, 7, 5, 1, 4, 7, 6, 3, 3, 4, 7, 4, 5, 3, 6, 4, 4, 4, 1, 1, 4, 2, 7, 1, 6, 6, 3, 7, 2, 1, 1, 7, 5, 7, 4, 2, 4, 4, 1, 6, 2, 1, 4, 5, 1, 3, 3, 5, 3, 5, 3, 2, 1, 2, 5, 5, 2, 7, 5, 3, 5, 4, 3, 7, 1, 4, 1, 5, 3, 6, 2, 1, 4, 2, 5, 5, 6];
    let mut last_update = Instant::now();
    let update_interval = Duration::from_secs(1); // Update every second

    
    while let Some(event) = window.next() {
        if let Some(_) = event.update_args() {
            let now = Instant::now();
            if now.duration_since(last_update) >= update_interval {
                (world, piece, next_piece) = update_game(world.clone(), piece.clone(), next_piece.clone(), pieces.clone());
                for block in world.blocks.iter() {
                    if block.pos.y == 0 {
                        window.set_should_close(true)
                    }
                }
                last_update = now;
            }
            let mut lines = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            for block in world.blocks.iter() {
                lines[block.pos.y as usize] += 1;
            }
            let mut layers_to_remove = vec![];
            let mut y = 0;
            for line in lines {
                if line == 10 {
                    layers_to_remove.push(y)
                }
                y += 1
            }
            for y in layers_to_remove.clone() {
                world.blocks.retain(|x| x.pos.y != y);
            }
            for block in world.blocks.iter_mut() {
                let mut fall_dist = 0;
                for y in layers_to_remove.clone() {
                    if block.pos.y < y {
                        fall_dist += 1;
                    }
                }
                block.pos.y += fall_dist;
            }
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::A => piece = move_left(piece.clone(), world.clone()),
                Key::D => piece = move_right(piece.clone(), world.clone()),
                Key::S => (world, piece, next_piece) = update_game(world.clone(), piece.clone(), next_piece.clone(), pieces.clone()),
                Key::Q => piece = turn_left(piece.clone(), world.clone()),
                Key::E => piece = turn_right(piece.clone(), world.clone()),
                _ => {}
            }
        }


        // Draw the window's contents
        window.draw_2d(&event, |c, g, _| {
            clear([0.0, 0.0, 1.0, 1.0], g); // Clear the screen with white color
            
            
            for block in world.blocks.iter() {
                let transform = c.transform.trans((block.pos.x*square_size).into(), (block.pos.y*square_size).into());
                // Draw the square:
                // color, box, transform, graphics backend
                rectangle([1.0, 0.0, 0.0, 1.0], square, transform, g);
            }
            for block in piece.blocks.iter() {
                let transform = c.transform.trans((block.pos.x*square_size).into(), (block.pos.y*square_size).into());
                // Draw the square:
                // color, box, transform, graphics backend
                rectangle([1.0, 0.0, 0.0, 1.0], square, transform, g);
            }
        });
    }
}