import React from 'react';
import { StyleSheet, View } from 'react-native';
import { Button, Text } from 'react-native-paper';
import { useAuth } from '../contexts/AuthContext';

export function HomeScreen() {
  const { signOut } = useAuth();

  return (
    <View style={styles.container}>
      <Text variant="headlineMedium">My Cats</Text>
      <Text variant="bodyMedium" style={styles.subtitle}>
        Your cats will appear here.
      </Text>
      <Button mode="outlined" onPress={signOut} style={styles.signOut}>
        Sign Out
      </Button>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    padding: 24,
  },
  subtitle: {
    marginTop: 8,
    opacity: 0.6,
  },
  signOut: {
    marginTop: 32,
  },
});
