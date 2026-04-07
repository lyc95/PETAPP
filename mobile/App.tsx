import React, { useEffect } from 'react';
import { PaperProvider } from 'react-native-paper';
import { AuthProvider } from './src/contexts/AuthContext';
import { RootNavigator } from './src/navigation/RootNavigator';
import { theme } from './src/theme';
import { createAndroidChannel, requestPermission } from './src/services/notificationService';

export default function App() {
  useEffect(() => {
    createAndroidChannel();
    requestPermission();
  }, []);

  return (
    <PaperProvider theme={theme}>
      <AuthProvider>
        <RootNavigator />
      </AuthProvider>
    </PaperProvider>
  );
}
