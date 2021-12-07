use crate::{
    render_graph::GizmoPass, PickableGizmo, TransformGizmoBundle, TransformGizmoInteraction,
};
use bevy::{prelude::*, render::render_graph::base::MainPass};
mod cone;
mod truncated_torus;

/// Startup system that builds the procedural mesh and materials of the gizmo.
pub fn build_gizmo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let axis_length = 1.5;
    let arc_radius = 1.1;
    // Define gizmo meshes
    let arrow_tail_mesh = meshes.add(Mesh::from(shape::Capsule {
        radius: 0.015,
        depth: axis_length,
        ..Default::default()
    }));
    let cone_mesh = meshes.add(Mesh::from(cone::Cone {
        height: 0.3,
        radius: 0.12,
        ..Default::default()
    }));
    let sphere_mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.12,
        subdivisions: 3,
    }));
    let rotation_mesh = meshes.add(Mesh::from(truncated_torus::TruncatedTorus {
        radius: arc_radius,
        ring_radius: 0.015,
        ..Default::default()
    }));
    //let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 0.15 }));
    // Define gizmo materials
    let gizmo_material_x = materials.add(StandardMaterial {
        unlit: true,
        base_color: Color::rgb(1.0, 0.4, 0.4),
        ..Default::default()
    });
    let gizmo_material_y = materials.add(StandardMaterial {
        unlit: true,
        base_color: Color::rgb(0.4, 1.0, 0.4),
        ..Default::default()
    });
    let gizmo_material_z = materials.add(StandardMaterial {
        unlit: true,
        base_color: Color::rgb(0.4, 0.5, 1.0),
        ..Default::default()
    });
    let gizmo_material_x_selectable = materials.add(StandardMaterial {
        unlit: true,
        base_color: Color::rgb(1.0, 0.7, 0.7),
        ..Default::default()
    });
    let gizmo_material_y_selectable = materials.add(StandardMaterial {
        unlit: true,
        base_color: Color::rgb(0.7, 1.0, 0.7),
        ..Default::default()
    });
    let gizmo_material_z_selectable = materials.add(StandardMaterial {
        unlit: true,
        base_color: Color::rgb(0.7, 0.7, 1.0),
        ..Default::default()
    });
    /*let gizmo_material_origin = materials.add(StandardMaterial {
        unlit: true,
        base_color: Color::rgb(0.7, 0.7, 0.7),
        ..Default::default()
    });*/
    // Build the gizmo using the variables above.
    commands
        .spawn_bundle(TransformGizmoBundle::default())
        .with_children(|parent| {
            // Translation Axes
            parent
                .spawn_bundle(PbrBundle {
                    mesh: arrow_tail_mesh.clone(),
                    material: gizmo_material_x.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_z(std::f32::consts::PI / 2.0),
                        Vec3::new(axis_length / 2.0, 0.0, 0.0),
                    )),
                    ..Default::default()
                })
                .insert(GizmoPass)
                .remove::<MainPass>();
            parent
                .spawn_bundle(PbrBundle {
                    mesh: arrow_tail_mesh.clone(),
                    material: gizmo_material_y.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_y(std::f32::consts::PI / 2.0),
                        Vec3::new(0.0, axis_length / 2.0, 0.0),
                    )),
                    ..Default::default()
                })
                .insert(GizmoPass)
                .remove::<MainPass>();
            parent
                .spawn_bundle(PbrBundle {
                    mesh: arrow_tail_mesh,
                    material: gizmo_material_z.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_x(std::f32::consts::PI / 2.0),
                        Vec3::new(0.0, 0.0, axis_length / 2.0),
                    )),
                    ..Default::default()
                })
                .insert(GizmoPass)
                .remove::<MainPass>();

            // Translation Handles
            parent
                .spawn_bundle(PbrBundle {
                    mesh: cone_mesh.clone(),
                    material: gizmo_material_x_selectable.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_z(std::f32::consts::PI / -2.0),
                        Vec3::new(axis_length, 0.0, 0.0),
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::TranslateAxis {
                    original: Vec3::X,
                    axis: Vec3::X,
                })
                .insert(GizmoPass)
                .remove::<MainPass>();
            parent
                .spawn_bundle(PbrBundle {
                    mesh: cone_mesh.clone(),
                    material: gizmo_material_y_selectable.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, axis_length, 0.0)),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::TranslateAxis {
                    original: Vec3::Y,
                    axis: Vec3::Y,
                })
                .insert(GizmoPass)
                .remove::<MainPass>();
            parent
                .spawn_bundle(PbrBundle {
                    mesh: cone_mesh.clone(),
                    material: gizmo_material_z_selectable.clone(),
                    transform: Transform::from_matrix(Mat4::from_rotation_translation(
                        Quat::from_rotation_x(std::f32::consts::PI / 2.0),
                        Vec3::new(0.0, 0.0, axis_length),
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::TranslateAxis {
                    original: Vec3::Z,
                    axis: Vec3::Z,
                })
                .insert(GizmoPass)
                .remove::<MainPass>();
            /*
                        // Origin
                        parent
                            .spawn_bundle(PbrBundle {
                                mesh: sphere_mesh.clone(),
                                material: gizmo_material_origin,
                                ..Default::default()
                            })
                            .insert(PickableGizmo::default())
                            .insert(TransformGizmoInteraction::TranslateOrigin)
                            .insert(GizmoPass)
                            .remove::<MainPass>();
            */
            // Rotation Arcs
            parent
                .spawn_bundle(PbrBundle {
                    mesh: rotation_mesh.clone(),
                    material: gizmo_material_x.clone(),
                    transform: Transform::from_rotation(Quat::from_axis_angle(
                        Vec3::Z,
                        f32::to_radians(90.0),
                    )),
                    ..Default::default()
                })
                .insert(GizmoPass)
                .remove::<MainPass>();
            parent
                .spawn_bundle(PbrBundle {
                    mesh: rotation_mesh.clone(),
                    material: gizmo_material_y.clone(),
                    ..Default::default()
                })
                .insert(GizmoPass)
                .remove::<MainPass>();
            parent
                .spawn_bundle(PbrBundle {
                    mesh: rotation_mesh.clone(),
                    material: gizmo_material_z.clone(),
                    transform: Transform::from_rotation(
                        Quat::from_axis_angle(Vec3::Z, f32::to_radians(90.0))
                            * Quat::from_axis_angle(Vec3::X, f32::to_radians(90.0)),
                    ),
                    ..Default::default()
                })
                .insert(GizmoPass)
                .remove::<MainPass>();

            // Rotation Handles
            parent
                .spawn_bundle(PbrBundle {
                    mesh: sphere_mesh.clone(),
                    material: gizmo_material_x_selectable.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        f32::to_radians(45.0).sin() * arc_radius,
                        f32::to_radians(45.0).sin() * arc_radius,
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::RotateAxis {
                    original: Vec3::X,
                    axis: Vec3::X,
                })
                .insert(GizmoPass)
                .remove::<MainPass>();
            parent
                .spawn_bundle(PbrBundle {
                    mesh: sphere_mesh.clone(),
                    material: gizmo_material_y_selectable.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        f32::to_radians(45.0).sin() * arc_radius,
                        0.0,
                        f32::to_radians(45.0).sin() * arc_radius,
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::RotateAxis {
                    original: Vec3::Y,
                    axis: Vec3::Y,
                })
                .insert(GizmoPass)
                .remove::<MainPass>();
            parent
                .spawn_bundle(PbrBundle {
                    mesh: sphere_mesh.clone(),
                    material: gizmo_material_z_selectable.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        f32::to_radians(45.0).sin() * arc_radius,
                        f32::to_radians(45.0).sin() * arc_radius,
                        0.0,
                    )),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::RotateAxis {
                    original: Vec3::Z,
                    axis: Vec3::Z,
                })
                .insert(GizmoPass)
                .remove::<MainPass>();
            /*
            // Scaling Handles
            parent
                .spawn_bundle(PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: gizmo_material_x_selectable.clone(),
                    transform: Transform::from_translation(Vec3::new(arc_radius, 0.0, 0.0)),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::ScaleAxis(Vec3::X))
                .insert(PriorityInteractable)
                .insert(GizmoPass)
                .remove::<MainPass>();
            parent
                .spawn_bundle(PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: gizmo_material_y_selectable.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, arc_radius, 0.0)),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::ScaleAxis(Vec3::Y))
                .insert(PriorityInteractable)
                .insert(GizmoPass)
                .remove::<MainPass>();
            parent
                .spawn_bundle(PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: gizmo_material_z_selectable.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, arc_radius)),
                    ..Default::default()
                })
                .insert(PickableGizmo::default())
                .insert(TransformGizmoInteraction::ScaleAxis(Vec3::Z))
                .insert(PriorityInteractable)
                .insert(GizmoPass)
                .remove::<MainPass>();
            */
        })
        .insert(GizmoPass)
        .remove::<MainPass>();
}