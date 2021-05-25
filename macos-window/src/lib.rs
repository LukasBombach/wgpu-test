use neon::prelude::*;

#[macro_use] extern crate cocoa;
#[macro_use] extern crate objc;

use cocoa::appkit::NSWindow;
use cocoa::base::{id, nil};

use objc::runtime::{Object, Sel};


fn hello(mut cx: FunctionContext) -> JsResult<JsString> {


    let my_window: id = NSWindow::alloc(nil);

    extern fn on_enter_fullscreen(this: &Object, _cmd: Sel, _notification: id) {
        unsafe {
            let window: id = *this.get_ivar("window");
            window.setToolbar_(nil);
        }
    }

    my_window.setDelegate_(delegate!("MyWindowDelegate", {
        window: id = my_window, // Declare instance variable(s)
        (onWindowWillEnterFullscreen:) => on_enter_fullscreen as extern fn(&Object, Sel, id) // Declare function(s)
    }));


    Ok(cx.string("hello node from rust"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}
