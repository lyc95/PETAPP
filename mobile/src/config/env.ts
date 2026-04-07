import { Platform } from 'react-native';

// Android emulator reaches the host via 10.0.2.2; iOS simulator uses localhost.
const DEV_HOST = Platform.OS === 'android' ? '10.0.2.2' : 'localhost';

export const API_BASE_URL = __DEV__
  ? `http://${DEV_HOST}:9000`
  : 'https://your-ec2-hostname-or-ip';
