pub fn launch() {
    match tauri::Builder::default().build(tauri::generate_context!()) {
        Ok(app) => {
            match tauri::WindowBuilder::new(
                &app,
                "main",
                tauri::WindowUrl::App("index.html".into()),
            )
            .focused(true)
            .decorations(false)
            .transparent(true)
            .title("Sky Vista")
            .inner_size(400.0, 300.0)
            .build()
            {
                Ok(window) => {}
                Err(err) => log::error!("Fail to build main window, Error: {:?}", err),
            }

            app.run(|_, _| {});
        }
        Err(err) => log::error!("Fail to build tauri app, Error: {:?}", err),
    }
}
