use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent, VirtualKeyCode, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use log::info;

pub struct WindowService {
    window: Window,
    pixels: Pixels,
}

impl WindowService {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = winit::window::WindowBuilder::new()
            .with_title("Learning Graphics Rendering")
            .with_inner_size(PhysicalSize::new(800, 600))
            .build(event_loop)
            .unwrap();

        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        let pixels = Pixels::new(size.width, size.height, surface_texture).unwrap();

        Self { window, pixels }
    }

    pub fn run<F>(mut self, event_loop: EventLoop<()>, mut draw_callback: F) 
    where F: 'static + FnMut(&mut [u8], usize, usize), {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => {
                    self.handle_window_event(event, control_flow);
                }
                Event::RedrawRequested(_) => {
                    self.redraw(&mut draw_callback);
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => {}
            }
        });
    }

    fn handle_window_event(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::CloseRequested => {
                info!("Closing window");
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(new_size) => {
                self.resize_pixels(new_size, control_flow);
            }
            WindowEvent::KeyboardInput { input, .. } => {
                self.handle_keyboard_input(input.virtual_keycode, input.state, control_flow);
            }
            _ => {}
        }
    }

    fn resize_pixels(&mut self, new_size: PhysicalSize<u32>, control_flow: &mut ControlFlow) {
        if let Err(e) = self.pixels.resize_surface(new_size.width, new_size.height) {
            eprintln!("Error resizing surface: {:?}", e);
            *control_flow = ControlFlow::Exit;
        }
    
        if self.pixels.resize_buffer(new_size.width, new_size.height).is_err() {
            eprintln!("Error resizing pixels buffer");
            *control_flow = ControlFlow::Exit;
        }
    }

    fn handle_keyboard_input(
        &self,
        key: Option<VirtualKeyCode>,
        state: ElementState,
        control_flow: &mut ControlFlow,
    ) {
        if let (Some(VirtualKeyCode::Escape), ElementState::Pressed) = (key, state) {
            info!("Closing");
            *control_flow = ControlFlow::Exit;
        }
    }

    fn redraw<F>(&mut self, draw_callback: &mut F)
    where F: FnMut(&mut [u8], usize, usize), {
        let size = self.window.inner_size();
        let width = size.width as usize;
        let height = size.height as usize;

        draw_callback(self.pixels.frame_mut(), width, height);
        self.pixels.render().unwrap();
    }
}