fn main() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        client::init_client_environment();
        client::run_client();
    }
}
