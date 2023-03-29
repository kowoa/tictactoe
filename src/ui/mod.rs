use bevy::{prelude::*, utils::HashMap, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::{PickableBundle, PickingCameraBundle};

use crate::data::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_materials.in_base_set(StartupSet::PreStartup))
            .add_startup_system(init_textures.in_base_set(StartupSet::PreStartup))
            .add_startup_system(spawn_camera)
            .add_startup_system(spawn_board);
    }
}

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mat_handles: Res<MaterialHandles>,
    tex_atlas_handle: Res<TextureAtlasHandle>,
    tex_atlas_indices: Res<TextureAtlasIndices>,
    params: Res<Params>,
) {
    let board_ent = commands.spawn(SpatialBundle::default())
        .insert(Name::new("Board"))
        .id();
        
    let bg_ent = commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(tex_atlas_indices.bg_index),
            texture_atlas: tex_atlas_handle.0.clone_weak(),
            transform: Transform::from_scale(Vec3::splat(8.))
                .with_translation(Vec3::new(0., 0., -100.)),
            ..default()
        },
        Name::new("Background"),
    )).id();
    commands.entity(board_ent).add_child(bg_ent);
    
    let mut board = Board(HashMap::new());
    
    for row in -1..=1 {
        for col in -1..=1 {
            let gap_multiplier = 1.18;
            let transform = Transform::from_scale(Vec3::splat(params.tile_size * 1.12))
                .with_translation(Vec3::new(
                    col as f32 * params.tile_size * gap_multiplier,
                    -(row as f32 * params.tile_size * gap_multiplier + 52.),
                    0.,
                ));
            let cell_pos = CellPosition { row, col };
            let cell_ent = commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform,
                    material: mat_handles.transparent.clone_weak(),
                    ..default()
                },
                PickableBundle::default(),
                CellState::None,
                cell_pos,
                Name::new("Cell"),
            )).id();
            commands.entity(board_ent).add_child(cell_ent);
            
            board.0.insert(cell_pos, cell_ent);
        }
    }
    
    commands.insert_resource(board);
}

fn spawn_game_over_popup(
    mut commands: Commands,
) {
}

fn init_textures(
    mut commands: Commands,
    mut tex_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let tex_handle = asset_server.load("../assets/atlas.png");
    let mut tex_atlas = TextureAtlas::new_empty(tex_handle, Vec2::new(248., 119.));

    let bg_index = tex_atlas.add_texture(Rect {
        min: Vec2::new(125., 3.),
        max: Vec2::new(189., 116.),
    });
    let x_index = tex_atlas.add_texture(Rect {
        min: Vec2::new(192., 97.),
        max: Vec2::new(208., 113.),
    });
    let o_index = tex_atlas.add_texture(Rect {
        min: Vec2::new(211., 97.),
        max: Vec2::new(227., 113.),
    });
    commands.insert_resource(TextureAtlasIndices {
        bg_index,
        x_index,
        o_index,
    });

    let tex_atlas_handle = tex_atlases.add(tex_atlas);
    commands.insert_resource(TextureAtlasHandle(tex_atlas_handle));
}

fn init_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let transparent = materials.add(ColorMaterial {
        color: Color::rgba(0., 0., 0., 0.),
        ..default()
    });
    
    let hovered = materials.add(ColorMaterial {
        color: Color::hex("#6540537f").unwrap(),
        ..default()
    });
    
    let winner = materials.add(ColorMaterial {
        color: Color::hex("#654053").unwrap(),
        ..default()
    });
    
    commands.insert_resource(MaterialHandles {
        transparent,
        hovered,
        winner,
    });
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle{
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
        ..default()
    })
        .insert(PickingCameraBundle::default())
        .insert(Name::new("Camera"));
}
