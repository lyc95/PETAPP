import axios from 'axios';
import { API_BASE_URL } from '../config/env';
import { authService } from './authService';

const apiClient = axios.create({ baseURL: API_BASE_URL });

// Inject Cognito access token into every request.
apiClient.interceptors.request.use(async config => {
  const token = await authService.getAccessToken();
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
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
