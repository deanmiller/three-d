
use three_d::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let screenshot_path = if args.len() > 1 { Some(args[1].clone()) } else {None};

    let mut window = Window::new_default("Effect").unwrap();
    let (width, height) = window.framebuffer_size();
    let gl = window.gl();

    // Renderer
    let mut renderer = DeferredPipeline::new(&gl).unwrap();
    let mut camera = Camera::new_perspective(&gl, vec3(4.0, 4.0, 5.0), vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0),
                                                degrees(45.0), width as f32 / height as f32, 0.1, 1000.0);

    let mut monkey = CPUMesh::from_bytes(include_bytes!("../assets/models/suzanne.3d")).unwrap().to_mesh(&gl).unwrap();
    monkey.color = vec3(0.5, 1.0, 0.5);

    let ambient_light = AmbientLight::new(&gl, 0.2, &vec3(1.0, 1.0, 1.0)).unwrap();
    let directional_light = DirectionalLight::new(&gl, 0.5, &vec3(1.0, 1.0, 1.0), &vec3(-1.0, -1.0, -1.0)).unwrap();

    let mut fog_effect = effects::FogEffect::new(&gl).unwrap();
    fog_effect.color = vec3(0.8, 0.8, 0.8);
    let mut debug_effect = effects::DebugEffect::new(&gl).unwrap();

    // main loop
    let mut time = 0.0;
    let mut rotating = false;
    window.render_loop(move |frame_input|
    {
        camera.set_size(frame_input.screen_width as f32, frame_input.screen_height as f32);

        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick {state, button, ..} => {
                    rotating = *button == MouseButton::Left && *state == State::Pressed;
                },
                Event::MouseMotion {delta} => {
                    if rotating {
                        camera.rotate(delta.0 as f32, delta.1 as f32);
                    }
                },
                Event::MouseWheel {delta} => {
                    camera.zoom(*delta as f32);
                },
                Event::Key { state, kind } => {
                    if kind == "R" && *state == State::Pressed
                    {
                        debug_effect.change_type();
                    }
                }
            }
        }
        time += frame_input.elapsed_time;

        // draw
        // Geometry pass
        renderer.geometry_pass(width, height, &|| {
            let transformation = Mat4::identity();
            monkey.render(&transformation, &camera);
        }).unwrap();

        // Light pass
        Screen::write(&gl, 0, 0, width, height, Some(&vec4(0.0, 0.0, 0.0, 1.0)), None, &|| {
            renderer.light_pass(&camera, Some(&ambient_light), &[&directional_light], &[], &[]).unwrap();
        }).unwrap();

        // Effect
        fog_effect.apply(time as f32, &camera, renderer.geometry_pass_depth_texture()).unwrap();
        debug_effect.apply(&camera, renderer.geometry_pass_texture(), renderer.geometry_pass_depth_texture()).unwrap();

        if let Some(ref path) = screenshot_path {
            #[cfg(target_arch = "x86_64")]
            Screen::save_color(path, &gl, 0, 0, width, height).unwrap();
            std::process::exit(1);
        }
    }).unwrap();
}