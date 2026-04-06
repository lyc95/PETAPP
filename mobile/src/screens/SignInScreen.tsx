import React, { useState } from 'react';
import { StyleSheet, View } from 'react-native';
import { Button, Snackbar, Text, TextInput } from 'react-native-paper';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { useAuth } from '../contexts/AuthContext';
import { LOCAL_MODE } from '../config/env';
import type { AuthStackParamList } from '../navigation/RootNavigator';

type Props = NativeStackScreenProps<AuthStackParamList, 'SignIn'>;

export function SignInScreen({ navigation }: Props) {
  const { signIn } = useAuth();
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleSignIn = async () => {
    if (!email || !password) {
      setError('Please enter your email and password.');
      return;
    }
    setLoading(true);
    try {
      await signIn(email.trim(), password);
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : 'Sign in failed. Please try again.';
      setError(msg);
    } finally {
      setLoading(false);
    }
  };

  return (
    <View style={styles.container}>
      <Text variant="headlineMedium" style={styles.title}>
        Cat Care
      </Text>

      {LOCAL_MODE && (
        <Text variant="bodySmall" style={styles.localHint}>
          Local mode — any email and password will work
        </Text>
      )}

      <TextInput
        label="Email"
        value={email}
        onChangeText={setEmail}
        keyboardType="email-address"
        autoCapitalize="none"
        mode="outlined"
        style={styles.input}
      />

      <TextInput
        label="Password"
        value={password}
        onChangeText={setPassword}
        secureTextEntry
        mode="outlined"
        style={styles.input}
      />

      <Button
        mode="contained"
        onPress={handleSignIn}
        loading={loading}
        disabled={loading}
        style={styles.button}
      >
        Sign In
      </Button>

      <Button
        mode="text"
        onPress={() => navigation.navigate('SignUp')}
        style={styles.link}
      >
        Don't have an account? Sign Up
      </Button>

      <Snackbar
        visible={!!error}
        onDismiss={() => setError('')}
        duration={4000}
      >
        {error}
      </Snackbar>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    padding: 24,
  },
  title: {
    textAlign: 'center',
    marginBottom: 32,
  },
  input: {
    marginBottom: 12,
  },
  button: {
    marginTop: 8,
  },
  link: {
    marginTop: 8,
  },
  localHint: {
    textAlign: 'center',
    marginBottom: 16,
    opacity: 0.6,
  },
});
