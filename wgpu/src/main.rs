use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("WGPU + Winit 0.29 FIXED FINAL")
        .build(&event_loop)
        .unwrap();

    

    
    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                elwt.exit(); // ✅ this is the proper way to exit in 0.29+
            }

            Event::AboutToWait => {
                // drawing/rendering goes here
            }

            _ => {}
        }
    });
}
