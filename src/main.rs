use bevy::{core_pipeline::clear_color::ClearColorConfig, gizmos, prelude::*};
use constrained_denaulay_triangulation::{triangulate, Triangle, Vector};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins))
        .insert_resource(Triangles(vec![]))
        .add_systems(Startup, (setup, gizmo_setup))
        .add_systems(Update, system)
        .run();
}
#[derive(Resource)]
struct Triangles(Vec<Triangle>);
fn setup(mut res: ResMut<Triangles>) {
    let mut input_points = Vec::new();
    input_points.push(Vector::new(-0., 7.0)*10.); //
    input_points.push(Vector::new(-5., 5.)*10.); //
    input_points.push(Vector::new(5., 5.)*10.); //
    input_points.push(Vector::new(-1., 3.)*10.); //
  input_points.push(Vector::new(3., 1.)*10.); //
  input_points.push(Vector::new(-4., -1.)*10.); //
  input_points.push(Vector::new(1., -2.)*10.); //
  input_points.push(Vector::new(-6., -4.)*10.); //
  input_points.push(Vector::new(5., -4.)*10.); //
    let mut holes: Vec<Vec<Vector>> = vec![];
    let mut hole = Vec::<Vector>::new();
    hole.push(Vector::new(-6., 6.)*10.);
    hole.push(Vector::new(6., 6.)*10.);
    hole.push(Vector::new(0., -2.)*10.);
    holes.push(hole);
    let input_hole = Some(&mut holes);
    //let input_hole = None;

    res.0 = match triangulate(&mut input_points, input_hole, None) {
        Ok(result) => result,
        Err(err) => panic!("triangulation failed!{:?}", err),
    };
    println!("{}",res.0.len());

}
fn system(mut res: Res<Triangles>, mut gizmos: Gizmos) {
    // Triangle
    for triangle in &res.0 {
        gizmos.linestrip_gradient_2d([
            (Vec2::new(triangle.p(0).x, triangle.p(0).y), Color::BLUE),
            (Vec2::new(triangle.p(1).x, triangle.p(1).y), Color::BLUE),
            (Vec2::new(triangle.p(2).x, triangle.p(2).y), Color::BLUE),
            (Vec2::new(triangle.p(0).x, triangle.p(0).y), Color::BLUE),
        ]);
    }
}

fn gizmo_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        ..default()
    });
}

struct VecWrapper(Vec2);

impl From<Vector> for VecWrapper {
    fn from(value: Vector) -> Self {
        VecWrapper(Vec2::new(value.x, value.y))
    }
}

impl From<VecWrapper> for Vec2 {
    fn from(value: VecWrapper) -> Self {
        value.0
    }
}
