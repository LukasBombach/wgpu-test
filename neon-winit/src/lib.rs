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
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

use cocoa::{
    appkit::{NSApp, NSApplication, NSWindow},
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSSize},
};

#[derive(Debug, Clone, Copy)]
enum CustomEvent {
    Timer,
}


fn open_window(mut cx: FunctionContext) -> JsResult<JsString> {

    let mut event_loop = EventLoop::<CustomEvent>::with_user_event();
    let _window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .build(&event_loop)
        .unwrap();

    // `EventLoopProxy` allows you to dispatch custom events to the main Winit event
    // loop from any thread.
    let event_loop_proxy = event_loop.create_proxy();

    std::thread::spawn(move || {
        // Wake up the `event_loop` once every second and dispatch a custom event
        // from a different thread.
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            event_loop_proxy.send_event(CustomEvent::Timer).ok();
        }
    });

    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;


        match event {
            Event::UserEvent(event) => println!("user event: {:?}", event),
            //Event::MainEventsCleared => {
            //    *control_flow = ControlFlow::Exit;
            //    // window.request_redraw();
            //},
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
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
