import axios from 'axios';
import { API_BASE_URL } from '../config/env';
import { authService } from './authService';

const apiClient = axios.create({ baseURL: API_BASE_URL });

// Inject Bearer token on every request.
apiClient.interceptors.request.use(async config => {
  const token = await authService.getToken();
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export default apiClient;
