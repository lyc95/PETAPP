import AsyncStorage from '@react-native-async-storage/async-storage';
import apiClient from './apiClient';

const TOKEN_KEY = '@catcare/token';

export const authService = {
  async signIn(email: string, password: string): Promise<void> {
    const res = await apiClient.post<{ data: { token: string } }>('/auth/login', {
      email,
      password,
    });
    await AsyncStorage.setItem(TOKEN_KEY, res.data.data.token);
  },

  async signUp(email: string, password: string): Promise<void> {
    const res = await apiClient.post<{ data: { token: string } }>('/auth/register', {
      email,
      password,
    });
    await AsyncStorage.setItem(TOKEN_KEY, res.data.data.token);
  },

  async signOut(): Promise<void> {
    await AsyncStorage.removeItem(TOKEN_KEY);
  },

  async getToken(): Promise<string | null> {
    return AsyncStorage.getItem(TOKEN_KEY);
  },

  async isSignedIn(): Promise<boolean> {
    const token = await AsyncStorage.getItem(TOKEN_KEY);
    return token !== null;
  },
};
