pub fn launch() {
    match tauri::Builder::default().build(tauri::generate_context!()) {
        Ok(app) => {
            let mut window_builder =
                tauri::WindowBuilder::new(&app, "main", tauri::WindowUrl::App("index.html".into()))
                    .focused(true)
                    .decorations(false)
                    // .transparent(true)
                    .title("Sky Vista")
                    .inner_size(400.0, 300.0);

            #[cfg(target_os = "windows")]
            {
                window_builder = window_builder.transparent(true);
            }

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
                    {}
                }
                Err(err) => log::error!("Fail to build main window, Error: {:?}", err),
            }

            app.run(|_, _| {});
        }
        Err(err) => log::error!("Fail to build tauri app, Error: {:?}", err),
    }
}
