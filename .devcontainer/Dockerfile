# .devcontainer/Dockerfile
FROM mcr.microsoft.com/devcontainers/rust:1

# Cài thêm các tool nếu cần, ví dụ just, cargo-watch
RUN sudo apt-get update \
 && sudo apt-get install -y just curl

# Tạo thư mục workspace
WORKDIR /workspace

# Copy toàn bộ Cargo.toml/Cargo.lock trước để cache dependencies
COPY ./Cargo.toml ./Cargo.lock ./
# Nếu bạn muốn cache từng crate member, copy các file member specs tương ứng
# rồi chạy cargo fetch. Đơn giản ta sẽ fetch toàn workspace:
RUN mkdir src && echo "fn main(){}" > src/main.rs \
 && cargo fetch

# Cuối cùng copy toàn bộ source
COPY . .

# Mặc định build để dev nhanh (tuỳ bạn)
RUN just build
