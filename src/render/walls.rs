use bevy::prelude::*;

use crate::warehouse::structs::Warehouse;

pub fn add_walls(
    mut commands: Commands,
    warehouse: Res<Warehouse>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::new(1.0, 1.0, 2.0));
    let material = materials.add(Color::srgb(1.0, 0.6, 0.3));

    for pos in warehouse.walls.iter() {
        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            warehouse.get_bevy_transform(pos, 1.0),
        ));
    };
}
