import { Platform } from 'react-native';

// Android emulator reaches the host (and Docker port-forwards) via 10.0.2.2.
// iOS simulator uses localhost directly.
const DEV_HOST = Platform.OS === 'android' ? '10.0.2.2' : 'localhost';

export const API_BASE_URL = __DEV__
  ? `http://${DEV_HOST}:9000`
  : 'REPLACE_WITH_API_GATEWAY_URL';

export const COGNITO_USER_POOL_ID = 'REPLACE_WITH_USER_POOL_ID';
export const COGNITO_CLIENT_ID = 'REPLACE_WITH_CLIENT_ID';
export const S3_REGION = 'us-east-1';
