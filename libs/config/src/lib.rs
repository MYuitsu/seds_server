// libs/config/src/lib.rs

//! Thư viện cấu hình chung cho toàn workspace.
//! Cho phép load `Settings` dễ dàng thông qua hàm `load()`.

pub mod settings;
mod loader; // Giữ loader là private module

/// Rút gọn hàm load từ loader
pub use loader::load as load_settings;

/// Xuất struct Settings để dễ sử dụng
pub use settings::Settings;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // TODO: Add actual tests for config loading
        assert!(true);
    }
}
