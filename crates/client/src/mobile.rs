#[cfg(target_os = "android")]
use bevy::winit::AndroidApp;

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

    crate::init_client_environment();
    crate::run_client(app);
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn start_ios_client() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("iOS Panic: {:?}", info);
    }));

    crate::init_client_environment();
    crate::run_client();
}
