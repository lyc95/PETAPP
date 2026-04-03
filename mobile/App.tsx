import React from 'react';
import { PaperProvider } from 'react-native-paper';
import { theme } from './src/theme';
import { RootNavigator } from './src/navigation/RootNavigator';

export default function App() {
  return (
    <PaperProvider theme={theme}>
      <RootNavigator />
    </PaperProvider>
  );
}
