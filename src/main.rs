use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {x, y}
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Rect,
    L,
    I,
    T,
    S,
}

#[derive(Debug)]
pub struct Block {
    pos: Position,
    shape: Shape,
    rot_count: i32, // 시계방향으로 90'씩 몇번 회전 했는가
    entities: Vec<Entity>,
}

pub fn create_block(pos: Position, shape: Shape, color: BlockColor, commands: &mut Commands, materials: &Res<Materials>) -> Entity {

    let entities = match shape {
        Shape::Rect => {
            [Position::new(0, 0), Position::new(1, 0), Position::new(0, 1), Position::new(1, 1)]
                .iter()
                .map(|cell_pos| pos + *cell_pos )
                .map(|pos| {
                    create_cell(pos, color, commands, materials)
                })
                .collect::<Vec<_>>()
        },
        _ => {
            todo!();
        }
    };

    commands.spawn( (Block {
            pos: pos,
            shape: shape,
            rot_count: 0,
            entities: entities,
    },))
    .current_entity().unwrap()
}

pub struct Cell;

const CELL_WIDTH: i32 = 100;
const CELL_HEIGHT: i32 = 100;

pub fn create_cell(pos: Position, color: BlockColor, commands: &mut Commands, materials: &Res<Materials>) -> Entity {
    commands.spawn(SpriteComponents {
        material: materials.get_material(color),
        sprite: Sprite::new(Vec2::new(1.0, 1.0)),
        ..Default::default()
    })
    .with(Cell)
    .with(pos)
    .current_entity().unwrap()
}

#[derive(Default)]
pub struct Materials {
    pub red_mat: Handle<ColorMaterial>,
    pub blue_mat: Handle<ColorMaterial>,
    pub green_mat: Handle<ColorMaterial>,
}

impl Materials {
    pub fn get_material(&self, color: BlockColor) -> Handle<ColorMaterial> {
        match color {
            BlockColor::RED => self.red_mat.clone(),
            BlockColor::BLUE => self.blue_mat.clone(),
            BlockColor::GREEN => self.green_mat.clone(),
        }
    }
}

pub fn init_materials(
    mut mat_assets: ResMut<Materials>,
    mut color_mat_res: ResMut<Assets<ColorMaterial>>,
) {
    mat_assets.red_mat = color_mat_res.add(ColorMaterial::color(Color::rgb(1.0, 0.0, 0.0)));
    mat_assets.blue_mat = color_mat_res.add(ColorMaterial::color(Color::rgb(0.0, 0.0, 1.0)));
    mat_assets.green_mat = color_mat_res.add(ColorMaterial::color(Color::rgb(0.0, 1.0, 0.0)));
}

#[derive(Debug, Copy, Clone)]
pub enum BlockColor {
    RED,
    BLUE,
    GREEN,
}

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(WindowDescriptor {
            title: "bevy-tetris".to_string(),
            width: 500,
            height: 800,
            ..Default::default()
        })
        .add_resource(Materials::default())
        .add_startup_system(test_init.system())
        .add_system(transform_cell.system())
        .add_plugins(DefaultPlugins)
        .run();
}

fn test_init(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn(Camera2dComponents::default());
    create_block(Position::new(0, 0), Shape::Rect, BlockColor::RED, &mut commands, &materials);
}

const CELLS_IN_ROW: i32 = 10;
const CELLS_IN_COLUMN: i32 = 16;

fn transform_cell(windows: Res<Windows>, mut query: Query<(&Cell, &Position, &mut Transform, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();

    let cell_width = window.width() as i32 / CELLS_IN_ROW;
    let cell_height = window.height() as i32 / CELLS_IN_COLUMN;

    for (_, pos, mut transform, mut sprite) in query.iter_mut() {
        let x = pos.x * cell_width + cell_width / 2 - (window.width() as i32 / 2);
        let y = pos.y * cell_height + cell_height / 2 - (window.height() as i32 / 2);

        transform.translation.set_x(x as f32);
        transform.translation.set_y(y as f32);

        sprite.size = Vec2::new(cell_width as f32, cell_height as f32);
    }
}