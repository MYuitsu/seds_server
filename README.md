### Thiết lập OAuth2 với Epic
Export các biến môi trường:
```bash
export EPIC_CLIENT_ID=…
export EPIC_CLIENT_SECRET=…
export EPIC_AUTHORIZE_URL=https://fhir.epic.com/…/authorize
export EPIC_TOKEN_URL=https://fhir.epic.com/…/token
export OAUTH2_REDIRECT_URI=https://my-app.com/auth/callback
export SESSION_KEY=$(openssl rand -hex 32)