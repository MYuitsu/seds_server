// libs/config/src/lib.rs

//! Thư viện cấu hình chung cho toàn workspace.
//! Cho phép load `Settings` dễ dàng thông qua hàm `load()`.

pub mod loader;
pub mod settings;

/// Rút gọn hàm load từ loader
pub use loader::load as load_settings;

/// Xuất struct Settings để dễ sử dụng
pub use settings::Settings;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
