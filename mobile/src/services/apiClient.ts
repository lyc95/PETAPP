import axios from 'axios';
import { API_BASE_URL, LOCAL_MODE } from '../config/env';
import { authService } from './authService';

const apiClient = axios.create({ baseURL: API_BASE_URL });

// Inject auth header on every request.
// Local mode: X-User-Id header (backend bypasses JWT validation).
// Prod mode: Cognito Bearer token.
apiClient.interceptors.request.use(async config => {
  if (LOCAL_MODE) {
    config.headers['X-User-Id'] = 'local-dev-user';
  } else {
    const token = await authService.getAccessToken();
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
  }
  return config;
});

// On 401: refresh the session and retry the original request once.
let isRefreshing = false;

apiClient.interceptors.response.use(
  response => response,
  async error => {
    const originalRequest = error.config;

    if (error.response?.status === 401 && !isRefreshing && !originalRequest._retried) {
      isRefreshing = true;
      originalRequest._retried = true;

      try {
        const newToken = await authService.refreshSession();
        if (newToken) {
          originalRequest.headers.Authorization = `Bearer ${newToken}`;
          return apiClient(originalRequest);
        }
      } finally {
        isRefreshing = false;
      }
    }

    return Promise.reject(error);
  },
);

export default apiClient;
