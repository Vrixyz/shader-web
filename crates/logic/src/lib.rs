use bevy::{prelude::*, render::pipeline::RenderPipeline};
use shapes::{CircleGaugeMaterial, ShapeMeshes};
use wasm_bindgen::prelude::*;

pub mod shapes;

pub struct GamePlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    Ok,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::Loading);
        app.add_plugins(DefaultPlugins);
        app.add_plugin(shapes::ShapesPlugin);

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        app.add_system(check_load.system());
        let init_visible_graphics = SystemSet::on_enter(AppState::Ok).with_system(init.system());
        app.add_system_set(init_visible_graphics);
        let update_visible_graphics =
            SystemSet::on_update(AppState::Ok).with_system(update_material.system());
        app.add_system_set(update_visible_graphics);
    }
}

fn check_load(time: Res<Time>, mut state: ResMut<State<AppState>>) {
    // Without that code, we're using render pipeline too early for the shaders, and it results in a crash.
    if time.seconds_since_startup() > 0.5f64 {
        state.set(AppState::Ok);
    }
}

fn init(mut commands: Commands, shapes: Res<ShapeMeshes>) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera_bundle);
    let mut transform = Transform::identity();
    transform.scale = [100f32, 100f32, 100f32].into();
    dbg!("init");
    let character = MeshBundle {
        mesh: shapes.quad2x2.clone(),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            shapes.pipeline_circle_gauge.clone(),
        )]),
        transform,
        ..Default::default()
    };
    commands
        .spawn_bundle(character)
        .insert(shapes.mat_circle_gauge.clone());
}

fn update_material(
    time: Res<Time>,
    shapes: ResMut<ShapeMeshes>,
    mut materials_circle_gauge: ResMut<Assets<CircleGaugeMaterial>>,
) {
    if let Some(mat) = materials_circle_gauge.get_mut(shapes.mat_circle_gauge.clone()) {
        mat.ratio = time.seconds_since_startup() as f32 % 2f32 / 2f32;
        if mat.ratio >= 0.5f32 {
            mat.color = Color::WHITE
        } else {
            mat.color = Color::GRAY;
        }
    }
}

#[wasm_bindgen]
pub fn run() {
    App::build().add_plugin(GamePlugin).run();
}
