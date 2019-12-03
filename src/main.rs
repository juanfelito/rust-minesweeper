// use winit::{
//     event::{Event, WindowEvent},
//     event_loop::{ControlFlow, EventLoop},
//     window::WindowBuilder,
// };

// fn main() {
//     let event_loop = EventLoop::new();

//     let window = WindowBuilder::new()
//         .with_title("JuanFelito's minesweeper")
//         .build(&event_loop)
//         .unwrap();

//     event_loop.run(move |event, _, control_flow| {
//         println!("{:?}", event);

//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 window_id,
//             } if window_id == window.id() => *control_flow = ControlFlow::Exit,
//             _ => *control_flow = ControlFlow::Wait,
//         }
//     });
// }
mod board;

fn main() {
    let width = 7;
    let height = 6;

    let array = board::new(width, height);

    array.print_board();
}