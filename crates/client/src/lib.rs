pub mod network;
pub mod replication;
pub mod presenter;

pub fn init_client_environment() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
    }
}
