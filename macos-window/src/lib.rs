use neon::prelude::*;

  
extern crate cocoa;

#[macro_use]
extern crate objc;

use cocoa::base::{selector, nil, NO, id};
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo,
                        NSString};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow,
                    NSBackingStoreBuffered, NSMenu, NSMenuItem, NSWindowStyleMask,
                    NSRunningApplication, NSApplicationActivateIgnoringOtherApps
                    };

// use core_graphics::display::CGDisplay;

use objc::runtime::{Object, Sel};
use objc::declare::ClassDecl;


fn hello(mut cx: FunctionContext) -> JsResult<JsString> {

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        extern "C" fn did_finish_launching(_: &Object, _: Sel, _: id) {
            println!("application_did_finish_launching");
        }


        // This must be done before `NSApp()` (equivalent to sending
        // `sharedApplication`) is called anywhere else, or we'll end up
        // with the wrong `NSApplication` class and the wrong thread could
        // be marked as main.
        // let app: id = msg_send![APP_CLASS.0, sharedApplication];

        let app = NSApp();

        // Create NSWindowDelegate
        let superclass = class!(NSResponder);
        let mut decl = ClassDecl::new("MyAppDelegate", superclass).unwrap();

        // decl.add_class_method(sel!(new), new as extern "C" fn(&Class, Sel) -> id);
        // decl.add_method(sel!(dealloc), dealloc as extern "C" fn(&Object, Sel));

        decl.add_method(
            sel!(applicationDidFinishLaunching:),
            did_finish_launching as extern "C" fn(&Object, Sel, id),
        );
        // decl.add_ivar::<*mut c_void>(AUX_DELEGATE_STATE_NAME);


        let app_delegate_class = decl.register();
        let app_delegate: id = msg_send![app_delegate_class, new];

        // app.setDelegate(app_delegate);

        let _: () = msg_send![app, setDelegate: app_delegate];
        // let _: () = msg_send![pool, drain];

        // pub struct AppDelegateClass(pub *const Class);
        // unsafe impl Send for AppDelegateClass {}
        // unsafe impl Sync for AppDelegateClass {}
        // AppDelegateClass(decl.register())


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


        // create Window
        // let display = CGDisplay::main();
        // let size = NSSize::new(display.pixels_wide() as _, display.pixels_high() as _);
        let window = NSWindow::alloc(nil)
            .initWithContentRect_styleMask_backing_defer_(NSRect::new(NSPoint::new(0., 0.),  NSSize::new(500., 500.)),
                                                          NSWindowStyleMask::NSTitledWindowMask,
                                                          NSBackingStoreBuffered,
                                                          NO)
            .autorelease();

        let title = NSString::alloc(nil).init_str("lsmf!");
        window.setTitle_(title);
        window.makeKeyAndOrderFront_(nil);

        let current_app = NSRunningApplication::currentApplication(nil);
        current_app.activateWithOptions_(NSApplicationActivateIgnoringOtherApps);

        app.run();
    }

    Ok(cx.string("hello node from rust"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}
