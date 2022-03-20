use std::f32::consts::PI;

use nalgebra::Point3;
use winit::{
    dpi::PhysicalSize,
    event,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use eng::render::{ArcballCameraController, Camera, Renderer, Scene};

#[tokio::main]
async fn main() {
    let _ = pretty_env_logger::init_timed();

    let event_loop = EventLoop::new();

    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("test").with_inner_size(PhysicalSize::new(1920i32, 1080));
    let window = builder.build(&event_loop).unwrap();

    let mut app = App::new(&window).await;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            event::Event::MainEventsCleared => window.request_redraw(),
            event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            virtual_keycode: Some(event::VirtualKeyCode::Escape),
                            state: event::ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            event::Event::RedrawRequested(_) => {
                app.render();
            }
            _ => {}
        }
    });
}

struct App {
    renderer: Renderer,
    scene: Scene,
    camera: Camera<ArcballCameraController>,
}

impl App {
    pub async fn new(window: &Window) -> App {
        let size = window.inner_size();
        let renderer = Renderer::new(window, size.width, size.height).await;

        let controller = ArcballCameraController::new(Point3::new(0.0, 0.8, 0.0), 2.5);
        let camera = Camera::new(45.0 * PI / 180.0, size.width as f32 / size.height as f32, 0.1, 100.0, controller);
        let scene = Scene::new();

        Self { renderer, scene, camera }
    }

    pub fn render(&mut self) {
        self.renderer.render(&self.camera, &self.scene);
    }
}
