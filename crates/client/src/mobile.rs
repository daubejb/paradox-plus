#[cfg(target_os = "android")]
use android_activity::AndroidApp;

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: AndroidApp) {
    // Initialize logging back-end for Android Logcat
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("ParadoxPlus")
    );

    std::panic::set_hook(Box::new(|info| {
        log::error!("Android Panic: {:?}", info);
    }));

    let _ = bevy::winit::ANDROID_APP.set(app.clone());

    crate::init_client_environment();
    crate::run_client(app);
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn start_ios_client(width: f32, height: f32) {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("iOS Panic: {:?}", info);
    }));

    if let Ok(mut lock) = crate::IOS_SCREEN_SIZE.lock() {
        *lock = Some((width, height));
    }

    crate::init_client_environment();
    crate::run_client();
}
