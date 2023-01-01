use ggez::graphics::{self, Rect, Mesh, MeshBuilder, Color, Canvas, DrawMode, DrawParam, FillOptions, StrokeOptions, FontData, Text, TextFragment, TextAlign, TextLayout, PxScale, LineJoin};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::mint::{Vector2, Point2};
use ggez::input::keyboard::KeyCode;
use ggez::input::mouse::{self, MouseButton};
use std::io::{Write, Read};
mod html_parser;
use std::str;
use html_parser::word_detection;

// // a space for constants
// player
const STARTER_COORDS: Point2<f32> = ORIGIN_POINT;
const PLAYER_HB_W: f32 = 45f32;
const PLAYER_HB_RATIO: f32 = 16f32 / 16f32;
const PLAYER_HB_H: f32 = PLAYER_HB_W * PLAYER_HB_RATIO;
const PLAYER_HB_W_HALF: f32 = PLAYER_HB_W * 0.5;
const PLAYER_HB_H_HALF: f32 = PLAYER_HB_H * 0.5;
const ADDITIONAL_COORD_VALUE: f32 = 0f32;
const PLAYER_DECELERATION: f32 = 0.33;
const MIN_ALLOWED_ACCEL: f32 = 0.5;
const JUMP_STRENGTH: f32 = -2000f32;
const MAX_PLAYER_CLIPPING_DISTANCE: f32 = 10f32;
// in a format of (x, y)
const ORIGIN_POINT: Point2<f32> = new_point2((0f32,0f32));
const EDITOR_CURSOR_DIMS: f32 = 10f32;
// tiles
const TILE_SIZE: f32 = 50f32;
const TILE_SIZE_HALF: f32 = TILE_SIZE * 0.5;


// speed and such
const GRAVITY: f32 = 75f32;
const DEFAULT_TILE_NUM: u16 = 1;
// etc
const TILE_STR_REGULAR: &str = "Regular";
const TILE_STR_HOSTILE: &str = "Hostile";
const TILE_STR_PHYSICS: &str = "Physics";

// TODO a space for functions i implemented
// fn new_player(ctx: &mut Context){

// }
const fn new_point2(coords: (f32, f32)) -> Point2<f32>{
    let (x, y): (f32, f32) = coords;
    Point2 {
        x,
        y,
    }
}
const fn new_vec2(coords: (f32, f32)) -> Vector2<f32>{
    let (x, y): (f32, f32) = coords;
    Vector2 {
        x,
        y
    }
}
const fn vec_to_point2(vector: Vector2<f32>) -> Point2<f32>{
    Point2 {
        x: vector.x,
        y: vector.y,
    }
}
const fn point2_to_vec(point2: Point2<f32>) -> Vector2<f32>{
    Vector2 {
        x: point2.x,
        y: point2.y,
    }
}
fn jump(ctx: &mut Context, player: &mut PlayerStruct, dt: f32) {
// just a check to see if a player can jump. if so, jump
    if frame_key_check(ctx, KeyCode::Space) && player.can_jump{
        player.move_object(new_vec2((0f32, -1f32)));
        player.accelaration.y = JUMP_STRENGTH * dt;
        if player.jump_count > 0{
            player.jump_count -= 1;
        } else {
            player.can_jump = false;
        }
//         refer to blocking_collision_check() to see where player.jump_count increments
    }
}
fn teleport_player(player: &mut PlayerStruct, movement_vec: Vector2<f32>) {
    let (x_mov, y_mov): (f32, f32) = (movement_vec.x, movement_vec.y);
    player.coords.x = x_mov + PLAYER_HB_W_HALF;
    player.coords.y = y_mov + PLAYER_HB_H_HALF;
    player.sprite.0.x = x_mov;
    player.sprite.0.y = y_mov;
}
// just key checks
fn frame_key_check(ctx: &mut Context, key: KeyCode) -> bool {
    ctx.keyboard.is_key_just_pressed(key)
}
// just key checks
fn key_check(ctx: &mut Context, key: KeyCode) -> bool {
    ctx.keyboard.is_key_pressed(key)
}
fn slow_player_down(player: &mut PlayerStruct) {
// slows player down, always active in update() unless in different mode
    if player.accelaration.x > 0f32 {
        if player.accelaration.x.abs() - MIN_ALLOWED_ACCEL <= 0f32 {
            player.accelaration.x = 0f32;
        } else {
            player.accelaration.x -= PLAYER_DECELERATION;
        }
    } else {
        if player.accelaration.x.abs() - MIN_ALLOWED_ACCEL  <= 0f32 {
            player.accelaration.x = 0f32;
        } else {
            player.accelaration.x += PLAYER_DECELERATION;
        }
    }
    if player.accelaration.y > 0f32 {
        if player.accelaration.y.abs() - MIN_ALLOWED_ACCEL <= 0f32 {
            player.accelaration.y = 0f32;
        } else {
            player.accelaration.y -= PLAYER_DECELERATION;
        }
    } else {
        if player.accelaration.y.abs() - MIN_ALLOWED_ACCEL <= 0f32 {
            player.accelaration.y = 0f32;
        } else {
            player.accelaration.y += PLAYER_DECELERATION;
        }
    }
}
fn parse_level(ctx: &mut Context, level_path: &str) -> Vec<Tile>{
        let mut lvl_str: String = String::new();

        let mut lvl_file = ctx.fs.open(level_path).unwrap();

        lvl_file.read_to_string(&mut lvl_str).unwrap();


        let tiles_info = html_parser::parse_html(&lvl_str, "level");

        let (tiles_coords, mut tiles_props) = (tiles_info.0, tiles_info.1);

        // println!("{}", &tiles_props.len());

        let mut i = 0usize;
        if tiles_coords[0] == "" {
            vec![]
        } else {
            while i <= tiles_props.len() - 1 {
                if tiles_props[i].is_empty() {
                    tiles_props.remove(i);
                }
                i+=1;
            }
            // println!("len of coords: {}, len of props: {}", &tiles_coords.len(), &tiles_props.len());
            /*HOW THE FUCK DOES THIS EXIST*/

            // println!("{:#?}", &tiles_coords);



            let mut tiles: Vec<Tile> = vec![];
        //      basically a parsing sequence


//             but we do need a number for each tile
            let mut tile_num: i32 = 1;
            for (i, tile_coords) in tiles_coords.iter().enumerate() {
                let tile_coords_mod = tile_coords.replace(',', " ");
                let tile_coords_vec = word_detection::count_and_disect_words(&tile_coords_mod).2;
                let tile_coords_tupple: (f32, f32) = (tile_coords_vec[0].parse().expect("ERROR WHEN PARSING X COORD OF TILE IN parse_level() function"), tile_coords_vec[1].parse().expect("ERROR WHEN PARSING Y COORD OF TILE IN parse_level() function"));
                let tile_type_str = if tiles_props[i].is_empty() {
                    "Regular"
                } else {
                    tiles_props[i][0].as_str()
                };
                let tile_prop_enum: TileType = match tile_type_str {
                    TILE_STR_REGULAR => TileType::Regular,
                    TILE_STR_PHYSICS => TileType::Physics,
                    TILE_STR_HOSTILE => TileType::Hostile,
                    _ => TileType::Regular,
                };
                // println!("{:#?}", tile_coords_tupple);
                let tile = Tile::new(
                    ctx,
                    new_point2(tile_coords_tupple),
                    None,
                    tile_prop_enum,
                    false,
                    tile_num,
                );
                tiles.push(tile);
                tile_num+=1;
                // let tile_coords_2 = word_detection::count_and_disect_words(&tile_coords).2;
                // println!("{}: {:#?}", amount_of_runs, tile_coords);
            }
            tiles
        }
}
fn blocking_collision_check(player: &mut PlayerStruct, tile: &mut Tile) -> ObstacleOrientation {
    let player_center: (f32, f32) = (player.sprite.0.center().x, player.sprite.0.center().y);
    let tile_center: (f32, f32) = (tile.sprite.0.center().x, tile.sprite.0.center().y);
//     // let distance_between_centers: (f32, f32) = (player_center.0 - tile_center.0, player_center.1 - tile_center.1);
//
//     let distance_b_to_t: f32 = player.sprite.0.bottom() - tile.sprite.0.top();
//
//     let distance_l_to_r: f32 = player.sprite.0.left() - tile.sprite.0.right();
//
//     let distance_t_to_b: f32 = player.sprite.0.top() - tile.sprite.0.bottom();
//
//     let distance_r_to_l: f32 = player.sprite.0.right() - tile.sprite.0.left();
//
//
//     // println!("distance_between_centers: {:?}", distance_between_centers);
//     // println!("btt: {},ltr: {}, ttb: {},rtl: {}", distance_b_to_t, distance_l_to_r, distance_t_to_b, distance_r_to_l);
//     // if distance_b_to_t < 1f32 {
//     //     // move_player(player, new_vec2((0f32, -distance_b_to_t)));
//     //     return ObstacleOrientation::Up
//     // } else if distance_t_to_b > -(TILE_SIZE * 0.2){
//     //     // move_player(player, new_vec2((0f32, -distance_t_to_b)));
//     //     return ObstacleOrientation::Down
//     // } else if distance_l_to_r > -(TILE_SIZE * 0.2){
//     //     // move_player(player, new_vec2((-distance_l_to_r, 0f32)));
//     //     return ObstacleOrientation::Right
//     // } else if distance_r_to_l < (TILE_SIZE * 0.2) {
//     //     // move_player(player, new_vec2((-distance_r_to_l, 0f32)));
//     //     return ObstacleOrientation::Left
//     // } else {
//     //     // move_player(player, new_vec2((0f32, -distance_b_to_t)));
//     //     return ObstacleOrientation::Clip
//     // }
// //     basically a squence to determine which part of player is closest to what part of tile
//     let distance_array: [f32; 4] = [distance_b_to_t, distance_l_to_r, distance_t_to_b, distance_r_to_l];
//     let mut lowest_value_var: f32 = f32::MAX;
//     let distance_array = distance_array.map(| elem | elem.abs());
//     for distance in distance_array {
//         if distance < lowest_value_var {
//             lowest_value_var = distance;
//         }
//     }
//     // println!("{}", lowest_value_var);
//     if lowest_value_var == distance_b_to_t.abs() {
//         move_player(player, new_vec2((0f32, -distance_b_to_t)));
// //         how many times player can jump = player.jump_count + 1;
//         ObstacleOrientation::Up
//     } else if lowest_value_var == distance_l_to_r.abs(){
//         if distance_b_to_t <= MAX_PLAYER_CLIPPING_DISTANCE {
//             move_player(player, new_vec2((0f32, -distance_b_to_t)));
//         } else {
//         move_player(player, new_vec2((-distance_l_to_r, 0f32)));
//         }
//         ObstacleOrientation::Right
//     } else if lowest_value_var == distance_t_to_b.abs(){
//         move_player(player, new_vec2((0f32, -distance_t_to_b + 1f32)));
//         ObstacleOrientation::Down
//     } else if lowest_value_var == distance_r_to_l.abs(){
//         if distance_b_to_t <= MAX_PLAYER_CLIPPING_DISTANCE {
//             move_player(player, new_vec2((0f32, -distance_b_to_t)));
//         } else {
//         move_player(player, new_vec2((-distance_r_to_l, 0f32)));
//         }
//         ObstacleOrientation::Left
//     } else {
//         ObstacleOrientation::Clip
//     }

// FIXME THIS IS A BETTER SOLUTION BUT I ARCHIVE IT ANYWAY CUZ FUCK IT
    let relevant_point_x_1: f32;
    let relevant_point_x_2: f32;
    let relevant_point_y_1: f32;
    let relevant_point_y_2: f32;
    let object_collision_l_or_r: ObstacleOrientation;
    let object_collision_u_or_d: ObstacleOrientation;
    let distance_to_push_l_or_r: f32;
    let distance_to_push_u_or_d: f32;
    let player_top = player.sprite.0.top();
    let player_bottom = player.sprite.0.bottom();
    let player_left = player.sprite.0.left();
    let player_right = player.sprite.0.right();
    let tile_top = tile.sprite.0.top();
    let tile_bottom = tile.sprite.0.bottom();
    let tile_left = tile.sprite.0.left();
    let tile_right = tile.sprite.0.right();



    //     first we determine the relative position of collision actors: by x and y. that will determine the value of relevant_point's
    if player_center.0 <= tile_center.0 {
        relevant_point_x_1 = tile_left;
        relevant_point_x_2 = player_right;
        object_collision_l_or_r = ObstacleOrientation::Left;
    } else{
        relevant_point_x_1 = tile_right;
        relevant_point_x_2 = player_left;
        object_collision_l_or_r = ObstacleOrientation::Right;
    }
    distance_to_push_l_or_r = relevant_point_x_1 - relevant_point_x_2;
    if player_center.1 <= tile_center.1 {
        relevant_point_y_1 = tile_top;
        relevant_point_y_2 = player_bottom;
        object_collision_u_or_d = ObstacleOrientation::Up;
    } else {
        relevant_point_y_1 = tile_bottom;
        relevant_point_y_2 = player_top;
        object_collision_u_or_d = ObstacleOrientation::Down;
    }
    distance_to_push_u_or_d = relevant_point_y_1 - relevant_point_y_2;

    if distance_to_push_l_or_r.abs() <= distance_to_push_u_or_d.abs() {

        player.move_object(
            new_vec2(
                (distance_to_push_l_or_r, 0f32)
            )
        );


        return object_collision_l_or_r;
    } else {
        player.move_object(
            new_vec2(
                (0f32, distance_to_push_u_or_d)
            )
        );

        return object_collision_u_or_d;
    }

    ObstacleOrientation::Clip
}
// structs
struct PlayerStruct {
    coords: Point2<f32>,
    sprite: (Rect, Mesh),
    can_jump: bool,
    jump_count: i32,
    is_alive: bool,
    accelaration: Vector2<f32>,
    max_player_acceleration: (f32, f32),
}
#[derive(Clone)]
struct Tile {
    tile_num: i32,
    sprite: (Rect, Mesh),
    tile_info: (Point2<f32>, TileType),
    editor_is_selected: bool,
}
struct EditorInfo {
    selected_tile_type: TileType,
    editor_tile: Tile,
    editor_cursor: (Rect, Mesh),
    editor_prompt: UserInterfaceElement,
}
pub struct UserInterfaceElement {
    element_dims: Point2<f32>,
    element_coords: Point2<f32>,
    text_colour: Color,
    element_colour: Color,
    text_scale: f32,
    is_rounded: bool,
    alignment: TextLayout,
    margin_l_and_r: f32,
    margin_t_and_b: f32,
    rect_and_mesh: (Rect, Mesh),
    text_object: Text,
}

// enums
#[derive(Copy, Clone)]
enum TileType {
    Regular,
    Hostile,
    Physics,
}
enum ObstacleOrientation {
    Left,
    Right,
    Up,
    Down,
    Clip,
}
// traits
pub trait Moveable {
    fn move_object(&mut self, movement_vec: Vector2<f32>);
    fn teleport_object(&mut self, movement_vec: Vector2<f32>);
}
impl Moveable for PlayerStruct {
    fn move_object(&mut self, movement_vec: Vector2<f32>) {
        let (x_mov, y_mov): (f32, f32) = (movement_vec.x, movement_vec.y);
        self.coords.x += x_mov;
        self.coords.y += y_mov;
        self.sprite.0.x += x_mov;
        self.sprite.0.y += y_mov;
    }
    fn teleport_object(&mut self, movement_vec: Vector2<f32>) {
        let (x_mov, y_mov): (f32, f32) = (movement_vec.x, movement_vec.y);
        self.coords.x = x_mov + PLAYER_HB_W_HALF;
        self.coords.y = y_mov + PLAYER_HB_H_HALF;
        self.sprite.0.x = x_mov;
        self.sprite.0.y = y_mov;
    }
}
impl Moveable for Tile {
    fn move_object(&mut self, movement_vec: Vector2<f32>) {
        let (x_mov, y_mov): (f32, f32) = (movement_vec.x, movement_vec.y);
        self.tile_info.0.x += x_mov;
        self.tile_info.0.y += y_mov;
        self.sprite.0.x += x_mov;
        self.sprite.0.y += y_mov;
    }
    fn teleport_object(&mut self, movement_vec: Vector2<f32>) {
        let (x_mov, y_mov): (f32, f32) = (movement_vec.x, movement_vec.y);
        self.tile_info.0.x = x_mov + PLAYER_HB_W_HALF;
        self.tile_info.0.y = y_mov + PLAYER_HB_H_HALF;
        self.sprite.0.x = x_mov;
        self.sprite.0.y = y_mov;
    }
}
impl Moveable for UserInterfaceElement {
    fn move_object(&mut self, movement_vec: Vector2<f32>) {
        let (x_mov, y_mov): (f32, f32) = (movement_vec.x, movement_vec.y);
        self.rect_and_mesh.0.x += x_mov;
        self.rect_and_mesh.0.y += y_mov;
    }
    fn teleport_object(&mut self, movement_vec: Vector2<f32>) {
        let (x_mov, y_mov): (f32, f32) = (movement_vec.x, movement_vec.y);
        self.rect_and_mesh.0.x = x_mov;
        self.rect_and_mesh.0.y = y_mov;
    }
}
// struct implementations
impl PlayerStruct {
    pub fn new(ctx: &mut Context, coords: Point2<f32>) -> Self{
        //  just inniting a player struct
//         and setting the base parameters, jumping and such
        let player_rect = graphics::Rect::new(-PLAYER_HB_W_HALF + ADDITIONAL_COORD_VALUE, -PLAYER_HB_H_HALF + ADDITIONAL_COORD_VALUE, PLAYER_HB_W, PLAYER_HB_H);
        let player_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), player_rect, Color::WHITE).expect("something went wrong when initialising a player");
        let sprite = (player_rect, player_mesh);
        let is_alive = true;
        let jump_count = 2;
        let max_player_acceleration: (f32, f32) = (15f32, 15f32);
        Self {
            coords,
            sprite,
            jump_count,
            is_alive,
            accelaration: new_vec2((0f32,0f32)),
            can_jump: false,
            max_player_acceleration,
        }
    }
}
impl Tile {
    pub fn new(ctx: &mut Context, coords: Point2<f32>, draw_mode: Option<DrawMode>, tile_type: TileType, editor_is_selected: bool, tile_num: i32) -> Self{
        //  just inniting a player struct
        let draw_mode_tile = match draw_mode {
            Some(i) => i,
            None => DrawMode::Fill(FillOptions::default()),
        };
        let colour = if !editor_is_selected {
            match tile_type {
                TileType::Regular => Color::WHITE,
                TileType::Physics => Color::BLUE,
                TileType::Hostile => Color::RED,
            }
        } else {
            Color::GREEN
        };
        let tile_rect = graphics::Rect::new(coords.x, coords.y, TILE_SIZE, TILE_SIZE);
        let tile_mesh = graphics::Mesh::new_rectangle(ctx, draw_mode_tile, tile_rect, colour).expect("bruh at impl Tile\n \tpub fn new(){\n}");
        let sprite = (tile_rect, tile_mesh);
        Self {
            tile_num,
            tile_info:(coords, tile_type),
            sprite,
            editor_is_selected
        }
    }
    pub fn change_draw_mode(&mut self, ctx: &mut Context, new_draw_mode: Option<DrawMode>, tile_type: TileType, editor_is_selected: bool) -> Self {
        Tile::new(ctx, self.tile_info.0, new_draw_mode, tile_type, editor_is_selected, self.tile_num)
    }
    pub fn default(ctx: &mut Context) -> Self {
        let tile_rect = graphics::Rect::new(0f32, 0f32, TILE_SIZE, TILE_SIZE);
        let tile_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), tile_rect, Color::WHITE).expect("bruh at impl Tile\n \tpub fn default(){\n}");
        let sprite = (tile_rect, tile_mesh);
        let default_coords = ORIGIN_POINT;
        Self {
            tile_num: 0,
            sprite,
            tile_info: (default_coords, TileType::Regular),
            editor_is_selected: false,
        }
    }
}
impl UserInterfaceElement {
    pub fn new(ctx: &mut Context, text: Option<String>, element_dims: Point2<f32>, element_coords: Point2<f32>, text_colour: Color, element_colour: Color, text_font: Option<String>, text_scale: f32, is_rounded: bool, alignment: TextLayout, margin_l_and_r: f32, margin_t_and_b: f32) -> UserInterfaceElement {
        let reale_text = match text {
            Some(n) => n,
            None => "Default Text".to_string(),
        };
        let reale_text_font = match text_font {
            Some(n) => n,
            None => "NightmareCodehack".to_string(),
        };
        let text_fragment = TextFragment::new(reale_text)
            .font(reale_text_font)
            .scale(text_scale)
            .color(text_colour);
        let mut text_object = Text::new(text_fragment);
        text_object
            .set_bounds(element_dims)
            .set_wrap(true)
            .set_layout(alignment);
        // let text_dims = match text_object.measure(ctx) {
        //     Ok(n) => n,
        //     Err(e) => panic!("panick!'d at UserInterfaceElement: new()"),
        // };
        let textbox_rect = Rect::new(
            // -((element_dims.x + margin_l_and_r) * 0.5),
            // -((element_dims.y + margin_t_and_b) * 0.5),
            element_coords.x,
            element_coords.y,
            element_dims.x + margin_l_and_r,
            element_dims.y + margin_t_and_b,
        );
        let textbox_mesh = match is_rounded {
            true => Mesh::new_rounded_rectangle(
                ctx,
                DrawMode::Fill (
                    FillOptions::default()
                ),
                textbox_rect,
                10f32,
                element_colour,
            ),
            false => Mesh::new_rectangle(
                ctx,
                DrawMode::Fill(
                    FillOptions::default()
                ),
                textbox_rect,
                element_colour,
            ),
        }.expect("error at rendering mesh at UserInterfaceElement: new()");
        UserInterfaceElement {
            element_dims,
            element_coords,
            text_colour,
            element_colour,
            text_scale,
            is_rounded,
            alignment,
            margin_l_and_r,
            margin_t_and_b,
            rect_and_mesh: (textbox_rect, textbox_mesh),
            text_object,
        }
    }
    pub fn render_to_canvas(&self, canvas: &mut Canvas) {
        canvas.draw(
            &self.rect_and_mesh.1,
            // new_point2(
            //     (
            //         self.element_coords.x  + self.margin_l_and_r * 0.5,
            //         self.element_coords.y  + self.margin_t_and_b * 0.5
            //     )
            //
            // ),
            new_point2(
                (self.rect_and_mesh.0.center().x - self.element_coords.x - self.element_dims.x * 0.5, self.rect_and_mesh.0.center().y - self.element_coords.y - self.element_dims.y * 0.5),
            ),
        );
        canvas.draw(
            &self.text_object,
            // new_point2(
            //     (
            //         self.element_coords.x + self.element_dims.x * 0.5 - text_dims.x * 0.5,
            //         self.element_coords.y + self.element_dims.y * 0.5 - text_dims.y * 0.5
            //     )
            // )
            self.rect_and_mesh.0.center(),
        );
    }
}
// MainState init struct
struct MainState {
    player: PlayerStruct,
    tiles: Vec<Tile>,
    screen_dimensions: (f32, f32),
    mode: i32,
    editor_info: EditorInfo,
    // tiles: Vec<CollisonTile>,
}
// implement MainState
impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        let screen_dimensions = ctx.gfx.drawable_size();
        let tiles = parse_level(ctx, "/level.txt");
        let editor_cursor_rect = graphics::Rect::new(ORIGIN_POINT.x, ORIGIN_POINT.y, EDITOR_CURSOR_DIMS, EDITOR_CURSOR_DIMS);
        let editor_cursor_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::Fill(FillOptions::default()), editor_cursor_rect, Color::MAGENTA).expect("COULD NOT CREATE MESH AT MainState::new()");
        let editor_cursor = (
            editor_cursor_rect,
            editor_cursor_mesh
        );
        let editor_text = UserInterfaceElement::new(
            ctx,
            Some(
                String::from("EDITOR MODE. CLICK THE RIGHT MOUSE BUTTON TO SAVE LAYOUT TO CUSTOM FILE")
            ),
            new_point2(
                (320f32, 140f32)
            ),
            new_point2(
                (screen_dimensions.0 - 330f32, screen_dimensions.1 - 890f32)
            ),
            Color::new(
                0.7764705882352941,
                0.7764705882352941,
                0.7215686274509804,
                1f32
            ),
            Color::new(
                0.1490196078431373,
                0.1529411764705882,
                0.3333333333333333,
                1f32
            ),
            None,
            20f32,
            true,
            TextLayout {
                h_align: TextAlign::Middle,
                v_align: TextAlign::Middle
            },
            10f32,
            10f32,
        );
        MainState {
            player: PlayerStruct::new(ctx, new_point2((STARTER_COORDS.x + ADDITIONAL_COORD_VALUE, STARTER_COORDS.y + ADDITIONAL_COORD_VALUE))),
            tiles,
            screen_dimensions,
            mode: 0,
            editor_info: EditorInfo {
                selected_tile_type: TileType::Regular,
                editor_tile: Tile::default(ctx),
                editor_cursor,
                editor_prompt: editor_text,
            },
        }
    }
}
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
//         enter play mode in custom lvl
        if frame_key_check(ctx, KeyCode::O) {
//         resets tile array and player pos, TURNS GAMEMODE BACK TO 0 (PLAY)
            teleport_player(&mut self.player, new_vec2((0f32,0f32)));
            self.mode = 0;
            self.tiles = parse_level(ctx, "/levels/test_level.txt");
        }
//         empty tile() vector
        if frame_key_check(ctx, KeyCode::P) {
            self.tiles = vec![];
        }
//         set gamemode to editor
        if frame_key_check(ctx, KeyCode::I) {
            self.mode = 1;
        }
//         load standard level
        if frame_key_check(ctx, KeyCode::L) {
        self.player.teleport_object( new_vec2((0f32,0f32)));
        self.mode = 0;
        self.tiles = parse_level(ctx, "/level.txt");
        }
//         start logic
        if self.mode == 0 {
            self.player.max_player_acceleration = (15f32, 15f32);

            let dt = ctx.time.delta().as_secs_f32();

            // println!("jump_count: {}", self.player.jump_count);

            let mut player_movement: Vector2<f32> = new_vec2((0f32,0f32));

            if key_check(ctx, KeyCode::Right) {
                jump(ctx, &mut self.player, dt);
                self.player.move_object(new_vec2((1f32, 0f32)));
                self.player.accelaration.x += 50f32 * dt;
            }
            else if key_check(ctx, KeyCode::Left) {
                jump(ctx, &mut self.player, dt);
                self.player.move_object(new_vec2((-1f32, 0f32)));
                self.player.accelaration.x += -50f32 * dt;
            }
            else if key_check(ctx, KeyCode::Down) {
                // self.player.move_object(new_vec2((0f32, 20f32)));
            } else {
                jump(ctx, &mut self.player, dt);
            }
    //         skidding parte

    //         GRAVITY NOOOOO
            self.player.accelaration.y += GRAVITY * dt;
    //         max speed setting
            if self.player.accelaration.x.abs() > self.player.max_player_acceleration.0 {
            let more_than_zero = self.player.accelaration.x > 0f32;
                match more_than_zero {
                true => {self.player.accelaration.x = self.player.max_player_acceleration.0},
                false => {self.player.accelaration.x = -self.player.max_player_acceleration.0},
                }
            };
            if self.player.accelaration.y.abs() > self.player.max_player_acceleration.1 {
            let more_than_zero = self.player.accelaration.y > 0f32;
                match more_than_zero {
                true => {self.player.accelaration.y = self.player.max_player_acceleration.1},
                false => {self.player.accelaration.y = -self.player.max_player_acceleration.1},
                }
            };
            slow_player_down(&mut self.player);
//             for tile in &self.tiles {
//
//             }
            // println!("the whole length of self.tiles{}", self.tiles.len());
            let mut vec_of_collision_tiles: Vec<&mut Tile> = vec![];
            for tile in &mut self.tiles{
                if (
                    (self.player.coords.x - tile.tile_info.0.x).abs() < 150f32
                ) && (
                    (self.player.coords.y - tile.tile_info.0.y).abs() < 150f32
                ) {

                    vec_of_collision_tiles.push(tile);
                }
            }
            // println!("the length of vec_of_collision_tiles: {}", &vec_of_collision_tiles.len());
    //         DONT CHANGE I THIIIINK
            for tile in vec_of_collision_tiles {
                if self.player.sprite.0.overlaps(&tile.sprite.0) {
                    // let reverse_accel: f32 = self.player.accelaration.y * -0.1;
                    let obst_dir: ObstacleOrientation = blocking_collision_check(&mut self.player, tile);
                    match obst_dir {
                        ObstacleOrientation::Left => {
                            self.player.accelaration.x = 0f32;
                            // println!("Left");
                        }
                        ObstacleOrientation::Right => {
                            self.player.accelaration.x = 0f32;
                            // println!("Right");
                        }
                        ObstacleOrientation::Up => {
                            self.player.can_jump = true;
                            self.player.accelaration.y = 0f32;
                            self.player.jump_count = 3;

                            // println!("Up");
                        }
                        ObstacleOrientation::Down => {
                            self.player.accelaration.y = 0f32;
                            self.player.jump_count = 3;

                            // println!("Down");
                        }
                        ObstacleOrientation::Clip => {

                        }
                    }
                }
            }
            player_movement.x += self.player.accelaration.x;
            player_movement.y += self.player.accelaration.y;
            self.player.move_object(player_movement);
            vec_of_collision_tiles = vec![];
            Ok(())
        } else if self.mode == 1{
            let mut tiles_len = self.tiles.len();
            if frame_key_check(ctx, KeyCode::Z) {
                if !self.tiles.is_empty(){
                        self.tiles.pop();
                    }
            }
            if ctx.mouse.button_just_pressed(MouseButton::Middle) {
                for tile in &mut self.tiles {
                    if tile.sprite.0.overlaps(&self.editor_info.editor_cursor.0) {
                        tile.editor_is_selected = !tile.editor_is_selected;
                    }
                }
            }
            if frame_key_check(ctx, KeyCode::Delete) {
                let mut i = 0usize;
                while i < tiles_len {
                    if self.tiles[i].editor_is_selected {
                        self.tiles.remove(i);
                    } else {
                        i+=1;
                    }
                    tiles_len = self.tiles.len();
                }
            }
            if key_check(ctx, KeyCode::Key1) {
                self.editor_info.selected_tile_type = TileType::Regular;
            } else if key_check(ctx, KeyCode::Key2) {
                self.editor_info.selected_tile_type = TileType::Hostile;
            } else if key_check(ctx, KeyCode::Key3) {
                self.editor_info.selected_tile_type = TileType::Physics;
            }
            if ctx.mouse.button_just_pressed(MouseButton::Right) {
                let mut file_string = String::from("<level>\n");
                if ctx.fs.exists("/levels/test_level.txt") {
                    ctx.fs.delete("/levels/test_level.txt")?;
                }
                for i in &self.tiles {
                    let prop_str = match i.tile_info.1 {
                        TileType::Regular => TILE_STR_REGULAR,
                        TileType::Physics => TILE_STR_PHYSICS,
                        TileType::Hostile => TILE_STR_HOSTILE,
                    };
                    file_string.push_str(format!("\t<tile tile_type=*{}*>{},{}</tile>\n", prop_str,  i.tile_info.0.x, i.tile_info.0.y).as_str())
                }
                file_string.push_str("</level>");
                ctx.fs.create("/levels/test_level.txt")?
                    .write_all(file_string.as_bytes()).expect("FAILED TO WRITE TO FILE ON LINE 440");
            }

            Ok(())
        } else {
            Ok(())
        }
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: Canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        mouse::set_cursor_hidden(ctx, true);
        // self.mode == 0 IS REGULAR GAMEPLAY
        if self.mode == 0  {
//             FIXME FIXED VALUES FOR DEBUGGING PURPOSES ONLY
            let mut mb = MeshBuilder::new();
                //  get mesh from the PlayerStruct

            let player_mesh = &self.player.sprite.1;


            canvas.draw(player_mesh, self.player.coords);

            for tile in &mut self.tiles {
                let tile_mesh: &Mesh = &tile.sprite.1;
                tile.sprite.0.x = tile.tile_info.0.x;
                tile.sprite.0.y = tile.tile_info.0.y;
                canvas.draw(tile_mesh, ORIGIN_POINT);
            }
            canvas.finish(ctx)?;

            Ok(())
        }
        // self.mode == 1 IS EDITOR MODE
        else if self.mode == 1 {
            let stroke_options: StrokeOptions = StrokeOptions::default().with_line_join(LineJoin::Round);
            let draw_mode: Option<DrawMode> = Some(DrawMode::Stroke(stroke_options.with_line_width(5f32)));
//         grab mouse, represent coords as a 5*5 grid for consistancy and do... stuffe
            ctx.mouse.cursor_hidden();
            let mouse_pos: Point2<f32> = ctx.mouse.position();
            let prototype_pos: (f32, f32) = (
            (
                (
                    mouse_pos.x / 5f32)
                    .floor()
                    * 5f32
                )
                - TILE_SIZE_HALF,
            (
                (
                    mouse_pos.y / 5f32)
                    .floor() * 5f32
                )
                - TILE_SIZE_HALF,
            );

//             TODO REMOVE HARDCODED TILE TYPE, THIS IS FOR DEBUG PURPOSES FIXME
            self.editor_info.editor_tile = Tile::new(
                ctx, new_point2(prototype_pos),
                draw_mode,
                self.editor_info.selected_tile_type,
                false,
                0,
            );
//             DON'T CHANGE!!!
            self.editor_info.editor_cursor.0.x = prototype_pos.0 + TILE_SIZE_HALF - (EDITOR_CURSOR_DIMS * 0.5);
            self.editor_info.editor_cursor.0.y = prototype_pos.1 + TILE_SIZE_HALF - (EDITOR_CURSOR_DIMS * 0.5);

            let block: Tile = self.editor_info.editor_tile.clone();
            // adds block to screen perpanently unless removed lol
            canvas.draw(&block.sprite.1, ORIGIN_POINT);
            if ctx.mouse.button_just_pressed(MouseButton::Left) {
                self.tiles.push(block);
            }
//             draws all those pushed blocks
            let mut index_of_tile: usize = 0;
            while index_of_tile < self.tiles.len() {
                let tile_data_type = self.tiles[index_of_tile].tile_info.1;
                let tile_is_selected = self.tiles[index_of_tile].editor_is_selected;
                let changed_tile_sprite = Tile::change_draw_mode(
                        &mut self.tiles[index_of_tile],
                        ctx,
                        draw_mode,
                        tile_data_type,
                        tile_is_selected,
                    ).sprite.1;
                canvas.draw(
                    &changed_tile_sprite,
                    ORIGIN_POINT,
                );
                index_of_tile+=1;
            }

            let mouse_mov_delta = ctx.mouse.delta();
            // let editor_text = UserInterfaceElement {
                // text: String::from("EDITOR MODE. CLICK THE RIGHT MOUSE BUTTON TO SAVE LAYOUT TO CUSTOM FILE"),
                // element_dims: new_point2(
                    // (320f32, 140f32)
                // ),
                // element_coords: new_point2(
                    // (self.screen_dimensions.0 - 170f32, self.screen_dimensions.1 - 820f32)
                // ),
                // element_colour: Color::new(0.1490196078431373, 0.1529411764705882, 0.3333333333333333, 1f32),
                // text_colour: Color::new(0.7764705882352941,  0.7764705882352941, 0.7215686274509804, 1f32),
                // element_colour: Color::WHITE,
                // text_colour: Color::BLACK,
                // text_font: NightmareCodehack,
                // text_scale: 20f32,
                // is_rounded: true,
                // alignment: TextLayout {
                    // h_align: TextAlign::Middle,
                    // v_align: TextAlign::Middle
                // },
                // margin_l_and_r: 10f32,
                // margin_t_and_b: 10f32,
            // };
            // let text_editor_fragment = TextFragment::new("EDITOR MODE\n CLICK THE RIGHT MOUSE BUTTON\n TO SAVE LAYOUT TO CUSTOM FILE")
            //     .font("NightmareCodehack")
            //     .color(Color::RED);
            // let text_editor = Text::new(text_editor_fragment);
            // let (text_editor_width, text_editor_height) = (text_editor.measure(ctx)?.x, text_editor.measure(ctx)?.y);
            // let text_editor_pos = (self.screen_dimensions.0 * 0.9 - (text_editor_width * 0.5), self.screen_dimensions.1 * 0.05 - (text_editor_height * 0.5));
            // let text_editor_draw_params = DrawParam::default()
            //     .dest(new_point2(text_editor_pos));
            // canvas.draw(&text_editor, text_editor_draw_params);
            if self.editor_info.editor_tile.sprite.0.overlaps(&self.editor_info.editor_prompt.rect_and_mesh.0) {
                self.editor_info.editor_prompt.move_object(point2_to_vec(mouse_mov_delta));
            }
            canvas.draw (
                &self.editor_info.editor_cursor.1,
                new_point2(
                    (
                        prototype_pos.0 + TILE_SIZE_HALF - (EDITOR_CURSOR_DIMS * 0.5),
                        prototype_pos.1 + TILE_SIZE_HALF - (EDITOR_CURSOR_DIMS * 0.5)
                    )
                )
            );
            self.editor_info.editor_prompt.render_to_canvas(&mut canvas);
            canvas.finish(ctx)?;
            Ok(())
        } else {
            Ok(())
        }
    }
}
// main

fn main() -> GameResult {
    let title: &str = "BEST GAME EVA (ping ponge)";

    let (mut ctx, event_loop) = ContextBuilder::new("best platforma game gamee", "ruslan")
        .window_setup(WindowSetup::default().title(title).vsync(true).icon("/resources/icon.png"))
        .window_mode(WindowMode::default().dimensions(1200f32,900f32))
        .build()?;

    let state = MainState::new(&mut ctx);

    // let home = std::env::var(".env").expect("bro?");
    // println!("{home}");

    let font_data = FontData::from_path(&ctx.fs, "/resources/fonts/NightmareCodehack.ttf").expect("FAILED TO PARSE FONT IN main()");

    ctx.gfx.add_font("NightmareCodehack", font_data);

    event::run(ctx, event_loop, state);
}
