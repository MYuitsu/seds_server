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

## 2. Setup và Chạy Services

Bên trong Docker container:

- Đăng nhập Hugging face bằng acess token:

```sh
huggingface-cli login --token <your-hugging-face-access-token>
```

- Chạy các services:

```sh
just gateway & just patient-summary-dev
```

**Note:**
- `just gateway` sẽ khởi động gateway service
- `just patient-summary-dev` sẽ khởi động 3 services (backend, frontend, và AI agent) của chức năng patient summary.
- Sau khi các services đã khởi động xong, chuyển sang terminal của máy host.

## 3. Kết nối gateway đến domain

Tuỳ vào nhà cung cấp domain, trong trường hợp này là Ngrok

- Kết nối Ngrok Domain đến gateway trong Docker container:

```sh
ngrok http 3000 --domain=<your-ngrok-domain>
```

## 4. Vào ứng dụng

Đi đến đường dẫn: `https://<your-ngrok-domain>`

Đăng nhập [FHIR](https://fhir.epic.com/Home) bằng tài khoảng mẫu có trong sand box
