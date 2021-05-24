#[cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]


use neon::prelude::*;
use std::{thread, time};
use winit::{
    event::{Event, StartCause},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};


fn open_window(mut cx: FunctionContext) -> JsResult<JsString> {

    let queue = cx.queue();

    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;


        match event {
            Event::MainEventsCleared => {
                *control_flow = ControlFlow::Exit;
                // window.request_redraw();
            },
            _ => (),
        }
    });

    /* std::thread::spawn(move || {

        loop{
            thread::sleep(time::Duration::from_secs(1));

            queue.send(move |_| {
                window.request_redraw();
                Ok(())
            });
        }
    }); */

    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("open_window", open_window)?;
    Ok(())
}
