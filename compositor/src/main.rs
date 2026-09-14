mod renderer;
mod state;

use std::sync::Arc;

use smithay::reexports::winit::dpi::LogicalSize;
use smithay::reexports::winit::window::WindowAttributes;

use smithay::{
    backend::{
        renderer::{
            element::{
                surface::render_elements_from_surface_tree,
                Kind,
            },
            utils::draw_render_elements,
            Color32F,
            Renderer,
        },
        winit::{self, WinitEvent},
    },
    reexports::{
        calloop::EventLoop,
        wayland_server::{Display, ListeningSocket},
    },
    utils::{Rectangle, Transform},
};

use state::{ClientState, MyCompositor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting MyDE compositor...");
    println!("Backend: Winit / Termux:X11");

    let mut display = Display::<MyCompositor>::new()?;
    let display_handle = display.handle();

    let mut event_loop = EventLoop::<MyCompositor>::try_new()?;
    let mut state = MyCompositor::new(&display_handle)?;

    println!("MyDE compositor state initialized.");

    let socket_name = "myde-0";
    let socket = ListeningSocket::bind(socket_name)?;

    println!("Wayland socket: {socket_name}");
    println!("Initializing graphical backend...");

    let attributes = WindowAttributes::default()
        .with_title("MyDE")
        .with_visible(true)
        .with_inner_size(LogicalSize::new(1080.0, 2194.0))
        .with_decorations(false)
        .with_resizable(false);

    let (mut backend, mut winit) =
        winit::init_from_attributes::<
            smithay::backend::renderer::gles::GlesRenderer
        >(attributes)?;

    println!(
        "MyDE X11 canvas: {}x{}",
        backend.window_size().w,
        backend.window_size().h
    );

    println!("MyDE graphical backend initialized.");
    println!("MyDE output follows the Termux:X11 display.");

    loop {
        winit.dispatch_new_events(|event| {
            match event {
                WinitEvent::Resized { size, .. } => {
                    println!(
                        "MyDE: display resized to {}x{}.",
                        size.w,
                        size.h
                    );
                }

                WinitEvent::Input(_) => {}

                WinitEvent::CloseRequested => {
                    println!("MyDE: display closed.");
                }

                _ => {}
            }
        });

        // Accept Wayland clients.
        loop {
            match socket.accept()? {
                Some(stream) => {
                    println!("Wayland client connecting...");

                    let client_state = Arc::new(ClientState {
                        compositor_state:
                            smithay::wayland::compositor::CompositorClientState::default(),
                    });

                    display
                        .handle()
                        .insert_client(stream, client_state)?;

                    println!("Wayland client connected.");
                }

                None => break,
            }
        }

        // Process client requests and commits.
        display.dispatch_clients(&mut state)?;
        display.flush_clients()?;

        let size = backend.window_size();
        let damage = Rectangle::from_size(size);

        {
            let (renderer, mut framebuffer) = backend.bind()?;

            let mut frame = renderer.render(
                &mut framebuffer,
                size,
                Transform::Flipped180,
            )?;

            // MyDE desktop background.
            frame.clear(
                Color32F::new(0.055, 0.055, 0.065, 1.0),
                &[damage],
            )?;

            // Render all XDG toplevel surfaces.
            for surface in state.xdg_shell_state.toplevel_surfaces() {
                let elements = render_elements_from_surface_tree(
                    renderer,
                    surface.wl_surface(),
                    (0, 0),
                    1.0,
                    1.0,
                    Kind::Unspecified,
                );

                draw_render_elements(
                    &mut frame,
                    1.0,
                    &elements,
                    &[damage],
                )?;
            }

            frame.finish()?.wait()?;
        }

        backend.submit(Some(&[damage]))?;

        event_loop.dispatch(
            Some(std::time::Duration::from_millis(16)),
            &mut state,
        )?;
    }
}
