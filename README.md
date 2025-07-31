# Hướng dẫn chạy SEDS Server

## Yêu cầu

- Docker
- Domain Name (e.g. Ngrok free domain)
   - Cài đặt cli tool để đăng ký domain (e.g. [Ngrok](https://ngrok.com/docs/getting-started/))
   - Tạo một domain và thay thế `redirect_uri` trong [default.yml](services/api-gateway/config/default.yaml) thành `https://your-ngrok-domain/epic-sandbox/callback`
   - Thêm `redirect_uri` vào `callback uri` của [SEDS](https://fhir.epic.com/Developer/Apps) trên FHIR

- [Đăng ký Hugging face](https://huggingface.co/docs/hub/en/oauth) và tạo access token

## Chạy dự án

### 1. Xây và chạy Docker

- Xây Docker image:

```sh
docker build -t seds_server -f .devcontainer/Dockerfile .
```

- Chạy Docker container:

```sh
docker run -it -p 3000:3000 seds_server
```

**Note:** `-p 3000:3000` cho phép Domain kết nối vào container thông qua `localhost:3000`

### 2. Setup và Chạy Services

Bên trong Docker container:

- Đăng nhập Hugging face bằng acess token:

```sh
huggingface-cli login --token <your-hugging-face-access-token>
```

- Chạy các services:

```sh
just gateway & just patient-summary-dev
```

**Giải thích các câu lệnh trong Justfile:**

1. **just gateway**

 - **Chức năng:** Khởi động `api-gateway`, một cổng chính giao tiếp giữa frontend và backend.
 - **Port:** `0.0.0.0:3000`
 - **Công nghệ:** Rust + Axum
 - **Truy cập:** `http://localhost:3000`
 - **Yêu cầu:** Cần các backend service đã được khởi động trước.

2. **just patient-summary-dev**

  - **Chức năng:** Alias để khởi động nhanh toàn bộ chức năng `patient-summary`.
  - **Bao gồm:**
    - `just patient-summary-backend`
    - `just patient-summary-frontend`
    - `just patient-summary-agent`
 - **Lưu ý:** Nên chạy lệnh này để chuẩn bị môi trường phát triển nhanh chóng.

3. **just patient-summary-backend**

 - **Chức năng:** Khởi động `patient-summary-service`, tổng hợp hồ sơ bệnh nhân từ các API FHIR liên quan.
 - **Port:** `0.0.0.0:3010`
 - **Công nghệ:** Rust + Axum
 - **Truy cập:** `http://localhost:3010`

4. **just patient-summary-frontend**

 - **Chức năng:**  Khởi động giao diện người dùng cho hệ thống `patient-summary`.
 - **Port:** `localhost:8000`
 - **Công nghệ:** Deno + Fresh
 - **Truy cập:** `http://localhost:8000`

5. **just patient-summary-agent**

 - **Chức năng:** khởi động `patient-summary-agent`, một AI agent chạy trên CPU của Container, giúp đưa ra lời khuyên y tế từ hồ sơ bệnh nhân.
 - **Port:** `0.0.0.0:3020`
 - **Công nghệ:** Python + FastAPI + huggingface_hub
 - **Tên agent:** google/gemma-1.1-2b-it
 - **Truy cập:** `http://localhost:3020`
 - **Yêu cầu:** Đăng nhập hugging face bằng access token thông qua huggingface-cli tool được cài trong container.
 

### 3. Kết nối gateway đến domain

Tuỳ vào nhà cung cấp domain, trong trường hợp này là Ngrok

- Kết nối Ngrok Domain đến gateway trong Docker container:

```sh
ngrok http 3000 --domain=<your-ngrok-domain>
```

### 4. Vào ứng dụng

Đi đến đường dẫn: `https://<your-ngrok-domain>`

Đăng nhập [FHIR](https://fhir.epic.com/Home) bằng tài khoảng mẫu có trong sand box

## Phát triển độc lập từng server

Do FHIR OAuth dùng trong api-gateway yêu cầu một domain hợp lệ (ví dụ: `https://<domain-name>`), nên việc kiểm thử end-to-end hoàn chỉnh trên môi trường `localhost` là không khả thi.

Thay vào đó, trong quá trình phát triển, nên áp dụng các phương pháp sau:

1. **Kiểm thử tích hợp (Integration Testing)**

 - Sử dụng các framework kiểm thử phù hợp để kiểm tra từng thành phần của server một cách độc lập.

2. **Kiểm thử API trong container**

 - Mở các cổng cần thiết và chạy các service trong container. Cách này giúp kiểm thử API trong môi trường gần giống thực tế mà không cần cấu hình đầy đủ OAuth domain.

Các phương pháp này giúp đảm bảo từng service hoạt động đúng mà không phụ thuộc vào cấu hình đầy đủ của hệ thống xác thực OAuth.
