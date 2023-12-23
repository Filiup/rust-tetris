use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

const ROWS: usize = 20;
const COLS: usize = 10;
const GRID_SPACING: f32 = 2.0;

#[derive(Resource, Default)]
pub struct Grid {
    width: f32,
    height: f32,
    cell_size: f32,
}

#[derive(Component)]
pub struct Tetrimino;

impl Grid {
    pub fn get_cell_size(&self) -> f32 {
        self.cell_size
    }
    pub fn get_width(&self) -> f32 {
        self.width
    }
    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn init(&mut self, width: f32, height: f32, cell_size: f32) {
        self.set_width(width);
        self.set_height(height);
        self.set_cell_size(cell_size);
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    pub fn set_cell_size(&mut self, cell_size: f32) {
        self.cell_size = cell_size;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(600.0, 800.0),
                resizable: false,
                title: "Tetris".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<Grid>()
        .add_systems(Startup, (spawn_camera, init_grid, spawn_tetrimino).chain())
        .run();
}

fn init_grid(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut grid: ResMut<Grid>,
) {
    let window = window_query.get_single().unwrap();
    let grid_height = window.height();
    let cell_size = grid_height / ROWS as f32 - GRID_SPACING;
    let half_cell_size = cell_size / 2.0;
    let grid_width = cell_size * COLS as f32 + (COLS as f32 * GRID_SPACING);

    grid.init(grid_width, grid_height, cell_size);

    for row in 0..ROWS {
        for col in 0..COLS {
            let x = half_cell_size + (col as f32 * (cell_size + GRID_SPACING));
            let y = grid_height - half_cell_size - (row as f32 * (cell_size + GRID_SPACING));

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(cell_size, cell_size)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            });
        }
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_tetrimino(mut commands: Commands, grid: Res<Grid>) {
    commands
        .spawn((
            Tetrimino {},
            SpriteBundle {
                transform: Transform::from_xyz(
                    grid.cell_size / 2.0,
                    grid.height - grid.cell_size / 2.0,
                    0.0,
                ),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(grid.cell_size, grid.cell_size)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });

            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(grid.cell_size, grid.cell_size)),
                    ..default()
                },
                transform: Transform::from_xyz(grid.cell_size + GRID_SPACING, 0.0, 0.0),
                ..default()
            });

            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(grid.cell_size, grid.cell_size)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, - grid.cell_size - GRID_SPACING, 0.0),
                ..default()
            });

            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(grid.cell_size, grid.cell_size)),
                    ..default()
                },
                transform: Transform::from_xyz(grid.cell_size + GRID_SPACING, - grid.cell_size - GRID_SPACING, 0.0),
                ..default()
            });
        });
}
