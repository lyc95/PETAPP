import { Platform } from 'react-native';

// Android emulator reaches the host (and Docker port-forwards) via 10.0.2.2.
// iOS simulator uses localhost directly.
const DEV_HOST = Platform.OS === 'android' ? '10.0.2.2' : 'localhost';

export const API_BASE_URL = __DEV__
  ? `http://${DEV_HOST}:9000`
  : 'example.com';

export const COGNITO_USER_POOL_ID = 'xxxx';
export const COGNITO_CLIENT_ID = 'xxxx';
export const S3_REGION = 'ap-southeast-1';

// In dev builds: skip Cognito, use local backend with a fixed user ID.
// In prod builds: use real Cognito and the deployed API.
export const LOCAL_MODE = __DEV__;