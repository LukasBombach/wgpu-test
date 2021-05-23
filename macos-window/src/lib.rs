use neon::prelude::*;

extern crate cocoa;
extern crate core_graphics;

#[macro_use]
extern crate objc;

use cocoa::base::{selector, nil, NO, id};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo,
                        NSString, NSUInteger};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow,
                    NSBackingStoreBuffered, NSMenu, NSMenuItem, NSWindowStyleMask,
                    NSRunningApplication, NSApplicationActivateIgnoringOtherApps,
                    NSWindowCollectionBehavior, NSApplicationPresentationOptions};

use core_graphics::display::CGDisplay;

use objc::runtime::{Object, Sel};
use objc::declare::ClassDecl;



fn hello(mut cx: FunctionContext) -> JsResult<JsString> {

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

        // create Menu Bar
        let menubar = NSMenu::new(nil).autorelease();
        let app_menu_item = NSMenuItem::new(nil).autorelease();
        menubar.addItem_(app_menu_item);
        app.setMainMenu_(menubar);

        // create Application menu
        let app_menu = NSMenu::new(nil).autorelease();
        let quit_prefix = NSString::alloc(nil).init_str("Quit ");
        let quit_title =
            quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
        let quit_action = selector("terminate:");
        let quit_key = NSString::alloc(nil).init_str("q");
        let quit_item = NSMenuItem::alloc(nil)
            .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
            .autorelease();
        app_menu.addItem_(quit_item);
        app_menu_item.setSubmenu_(app_menu);


        // Create NSWindowDelegate
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("MyWindowDelegate", superclass).unwrap();


        extern fn application_did_finish_launching(_: &Object, _: Sel, _: id, _: NSUInteger) -> () {
        }

        decl.add_method(sel!(window:applicationDidFinishLaunching:), application_did_finish_launching as extern fn(&Object, Sel, id, NSUInteger) -> ());

        let delegate_class = decl.register();
        let delegate_object = msg_send![delegate_class, new];


        // create Window
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(NSRect::new(NSPoint::new(0., 0.),
                                                                      NSSize::new(200., 200.)),
                                                          NSWindowStyleMask::NSTitledWindowMask,
                                                          NSBackingStoreBuffered,
                                                          NO)
            .autorelease();

        window.setDelegate_(delegate_object);

        window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
        window.center();
        let title = NSString::alloc(nil).init_str("Hello World!");
        window.setTitle_(title);
        window.makeKeyAndOrderFront_(nil);
        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);
        app.run();
    }

    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}
