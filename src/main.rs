use bevy::prelude::*;
use bevy::transform::TransformSystem;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<AmbientLight>()
        .add_systems(Startup, startup)
        .add_systems(Update, spawn_entity)
        .add_systems(
            PostUpdate,
            add_children.before(TransformSystem::TransformPropagate),
        )
        .add_systems(
            PostUpdate,
            snap_transform.before(TransformSystem::TransformPropagate),
        )
        .add_systems(Last, log)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });
}

fn spawn_entity(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut counter: Local<usize>,
) {
    if *counter < 50 {
        let mut entity_commands = commands.spawn_empty();

        entity_commands.insert((
            Position { x: 1, y: 1, z: 1 },
            NeedsChildren,
            PbrBundle {
                mesh: meshes.add(shape::Cube::new(1.0).into()),
                material: materials.add(Color::CYAN.into()),
                ..default()
            },
        ));

        *counter += 1;
    }
}

fn snap_transform(
    mut query: Query<(&mut Transform, &Position), Or<(Changed<Transform>, Changed<Position>)>>,
) {
    for (mut transform, position) in query.iter_mut() {
        *transform = Transform::from_xyz(position.x as f32, position.y as f32, position.z as f32);
    }
}

fn add_children(
    q_needs_children: Query<(Entity, Option<&Children>), (With<NeedsChildren>, Changed<Position>)>,
    mut commands: Commands,
) {
    for (entity, children) in q_needs_children.iter() {
        if let Some(children) = children {
            for child in children {
                commands.entity(*child).despawn_recursive();
            }
        }

        for _ in 0..5 {
            commands.entity(entity).with_children(|parent| {
                parent.spawn(SpatialBundle::default());
            });
        }
    }
}

fn log(query: Query<(Entity, &GlobalTransform), With<Position>>) {
    let entities = query
        .iter()
        .filter(|(entity, global_transform)| **global_transform == GlobalTransform::default())
        .map(|(entity, _)| entity)
        .collect::<Vec<_>>();

    println!("Entities with unchanged GlobalTransform: {:?}", entities);
}

#[derive(Component)]
struct NeedsChildren;

#[derive(Component)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}
