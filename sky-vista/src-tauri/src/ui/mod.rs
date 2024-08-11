pub fn launch() {
    match tauri::Builder::default().build(tauri::generate_context!()) {
        Ok(app) => {
            let mut window_builder =
                tauri::WindowBuilder::new(&app, "main", tauri::WindowUrl::App("index.html".into()))
                    .focused(true)
                    .decorations(false)
                    .title("Sky Vista")
                    .inner_size(400.0, 300.0);

            match window_builder.build() {
                Ok(window) => {
                    #[cfg(target_os = "windows")]
                    {
                        if let Ok(hwnd) = window.hwnd() {
                            let hwnd = windows::Win32::Foundation::HWND(hwnd.0 as *mut _);
                            let margins = windows::Win32::UI::Controls::MARGINS {
                                cxLeftWidth: 1,
                                cxRightWidth: 1,
                                cyTopHeight: 1,
                                cyBottomHeight: 1,
                            };
                            unsafe {
                                let _ = windows::Win32::Graphics::Dwm::DwmExtendFrameIntoClientArea(
                                    hwnd, &margins,
                                );
                            }
                        }
                    }

                    #[cfg(target_os = "macos")]
                    {
                        match window.ns_window() {
                            Ok(handle) => unsafe {
                                use cocoa::base::id;
                                use objc::{
                                    class, msg_send,
                                    runtime::{Object, NO, YES},
                                    sel, sel_impl,
                                };

                                let handle = handle as id;

                                let _: () = msg_send![handle, setHasShadow: YES];

                                let content_view: *mut Object = msg_send![handle, contentView];
                                let _: () = msg_send![content_view, setWantsLayer: YES];

                                let layer: *mut Object = msg_send![content_view, layer];
                                let _: () = msg_send![layer, setCornerRadius: 8.0];

                                let clear_color: *mut Object =
                                    msg_send![class!(NSColor), clearColor];
                                let _: () = msg_send![content_view, setBackgroundColor:clear_color];
                                let _: () = msg_send![handle, setOpaque: NO];
                                let _: () = msg_send![handle, setBackgroundColor:clear_color];
                            },
                            Err(err) => {
                                log::error!("Fail to get main window ns handle, Error: {:?}", err)
                            }
                        }
                    }
                }
                Err(err) => log::error!("Fail to build main window, Error: {:?}", err),
            }

            app.run(|_, _| {});
        }
        Err(err) => log::error!("Fail to build tauri app, Error: {:?}", err),
    }
}
