use bevy::{camera::ScalingMode, prelude::*};
use bevy::window::{Window, WindowResolution, WindowPlugin};
use bevy_spritesheet_animation::prelude::*; // ANIMATION PLUGIN
use rand::Rng;

//[GAME LOADER (RUST'S MAIN FUNCTION)]
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin { //GAME WINDOW SETTINGS
                    primary_window: Some(Window {
                        title: "Toy RTS Camera And Units System".to_string(),
                        resolution: WindowResolution::new(1600, 900),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()) //PIXEL PERFECT 2D TEXTURES RENDERER
        )
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, game_init)
        .add_systems(Update, (udt_camera_movement_system, udt_zoom_control_system, udt_print_sprite_bounding_boxes, udt_wandering_system))
        .run();
}

//<DEFINE STRUCTS AREA>
#[derive(Component)]
struct Unit{
    movement_timer: f32,
    timer: Timer,
}


#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;
//</DEFINE STRUCTS AREA>

//[GAME INITIALIZER (GAME STARTUP/SETUP)]
fn game_init(mut commands: Commands,asset_server: Res<AssetServer>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("0==============================================================0");
    println!("Toy RTS Camera And Units System succesfully run and initialized!");
    println!("0==============================================================0");

    //add camera to scene
    commands.spawn((
        MainCamera,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 1.0,
            //viewport_origin: Vec2::new(0.85, 0.08),
            ..OrthographicProjection::default_2d()
        }),
    ));


    //world origin circle (debug-tool)
    const CIRCLE_COLOR: bevy::prelude::Color = Color::hsl(0.0, 0.0, 1.0);
    commands.spawn((Mesh2d(meshes.add(Circle::new(5.0))),MeshMaterial2d(materials.add(CIRCLE_COLOR)),Transform::from_xyz(0., 0., 0.)));

    // let spritesheet_uam_riflemen_idle_s = Spritesheet::new(&texture_uam_riflemen_idle_s, 1, 1);
    // let animation_uam_riflemen_idle_s = spritesheet_uam_riflemen_idle_s
    //     .create_animation()
    //     .add_row(1)
    //     .set_duration(AnimationDuration::PerFrame(100))
    //     .build();
    // let animation_handle_uam_riflemen_idle_s = animations.add(animation_uam_riflemen_idle_s);
    // let sprite_uam_riflemen_idle_s = spritesheet_uam_riflemen_idle_s
    //     .with_size_hint(768, 768)
    //     .sprite(&mut atlas_layouts);
    
    //example game sprite (uam_riflemen)
    for x in 0..20 { //spawn 10 uam_riflemens
        for y in 0..20 {
            let mut rng = rand::thread_rng();
            let random_offset: f32 = rng.gen_range(50.0..140.0);
            let texture_uam_riflemen_idle_s = asset_server.load("uam_riflemen_idle_s.png");
            commands.spawn((
                Unit {
                    movement_timer: 0.0,
                    timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                },
                Sprite{
                    image: texture_uam_riflemen_idle_s,
                    custom_size: Some(Vec2::new(200.0, 200.0)),
                    ..default()
                },
                Transform::from_xyz((x as f32 * -200.0) + 2000.0 + random_offset, (y as f32 * -200.0) + 2000.0 + random_offset, 0.0),
            ));
        }
    }
}

//[UDT MEANS UPDATE FUNCTION (RUNS EVERY FRAME)]

//camera movement control system
fn udt_camera_movement_system(
  time: Res<Time>,
  input: Res<ButtonInput<KeyCode>>,
  mut transform: Single<&mut Transform, With<MainCamera>>,
) {
  let dt = time.delta_secs();
  let movement_speed = 200.;

  let mut direction = Vec3::ZERO;

  if input.pressed(KeyCode::KeyW) {
    direction.y += 1.0;
  }
  if input.pressed(KeyCode::KeyS) {
    direction.y -= 1.0;
  }
  if input.pressed(KeyCode::KeyA) {
    direction.x -= 1.0;
  }
  if input.pressed(KeyCode::KeyD) {
    direction.x += 1.0;
  }

  if direction != Vec3::ZERO {
    direction = direction.normalize();
    transform.translation += direction * dt * movement_speed;
  }
}

//camera zoom/scale control system
fn udt_zoom_control_system(
  input: Res<ButtonInput<KeyCode>>,
  mut projection: Single<&mut Projection, With<MainCamera>>,
) {
  let Projection::Orthographic(perspective) = projection.as_mut() else {
    return;
  };

  if input.pressed(KeyCode::Minus) {
    perspective.scale += 0.01;
  }
  if input.pressed(KeyCode::Equal) {
    perspective.scale -= 0.01;
  }

  perspective.scale = perspective.scale.clamp(0.1, std::f32::consts::FRAC_PI_2);
}

fn trst_lerp<T>(start: T, end: T, t: f32) -> T
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<f32, Output = T> + Copy,
{
    start + (end - start) * t
}

fn udt_wandering_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Unit)>,
) {
    for (mut transform, mut wanderer) in query.iter_mut() {
        let mut rng = rand::thread_rng();
        let direction = Vec2::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        ).normalize_or_zero();
        transform.translation += direction.extend(0.0) * 50.0 * time.delta_secs();
    }
}


//sprite infomations (debug-tool)
fn udt_print_sprite_bounding_boxes(
  mut sprites: Query<(&Transform, &Sprite)>,
  input: Res<ButtonInput<KeyCode>>,
  assets: Res<Assets<Image>>,
) {
    if input.just_pressed(KeyCode::KeyI) {
        for (transform, sprite) in &mut sprites {
            let image_size = assets.get(&sprite.image).unwrap().size_f32();

            info!("image_dimensions: {:?}", image_size);
            info!("position: {:?}", transform.translation);
            info!("scale: {:?}", transform.scale);

            let scaled = image_size * transform.scale.truncate();
            let bounding_box =
            Rect::from_center_size(transform.translation.truncate(), scaled);

            info!("bounding_box: {:?}", bounding_box);
        }
    }
}
