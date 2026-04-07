import React from 'react';
import { ActivityIndicator, StyleSheet, View } from 'react-native';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { useAuth } from '../contexts/AuthContext';
import { SignInScreen } from '../screens/SignInScreen';
import { SignUpScreen } from '../screens/SignUpScreen';
import { HomeScreen } from '../screens/HomeScreen';
import { AddCatScreen } from '../screens/AddCatScreen';
import { CatProfileScreen } from '../screens/CatProfileScreen';
import { MedicineRemindersScreen } from '../screens/MedicineRemindersScreen';
import type { Cat } from '../types';

// ---------------------------------------------------------------------------
// Navigation param lists
// ---------------------------------------------------------------------------

export type AuthStackParamList = {
  SignIn: undefined;
  SignUp: undefined;
};

export type AppStackParamList = {
  Home: undefined;
  AddCat: undefined;
  CatProfile: { cat: Cat };
  MedicineReminders: { cat: Cat };
};

// ---------------------------------------------------------------------------
// Stacks
// ---------------------------------------------------------------------------

const AuthStack = createNativeStackNavigator<AuthStackParamList>();
const AppStack = createNativeStackNavigator<AppStackParamList>();

function AuthNavigator() {
  return (
    <AuthStack.Navigator screenOptions={{ headerShown: false }}>
      <AuthStack.Screen name="SignIn" component={SignInScreen} />
      <AuthStack.Screen name="SignUp" component={SignUpScreen} />
    </AuthStack.Navigator>
  );
}

function AppNavigator() {
  return (
    <AppStack.Navigator>
      <AppStack.Screen name="Home" component={HomeScreen} options={{ title: 'My Cats' }} />
      <AppStack.Screen name="AddCat" component={AddCatScreen} options={{ title: 'Add Cat' }} />
      <AppStack.Screen
        name="CatProfile"
        component={CatProfileScreen}
        options={({ route }) => ({ title: route.params.cat.name })}
      />
      <AppStack.Screen
        name="MedicineReminders"
        component={MedicineRemindersScreen}
        options={({ route }) => ({ title: `${route.params.cat.name} — Reminders` })}
      />
    </AppStack.Navigator>
  );
}

// ---------------------------------------------------------------------------
// Root
// ---------------------------------------------------------------------------

export function RootNavigator() {
  const { isAuthenticated, isLoading } = useAuth();

  if (isLoading) {
    return (
      <View style={styles.loading}>
        <ActivityIndicator size="large" />
      </View>
    );
  }

  return (
    <NavigationContainer>
      {isAuthenticated ? <AppNavigator /> : <AuthNavigator />}
    </NavigationContainer>
  );
}

const styles = StyleSheet.create({
  loading: { flex: 1, alignItems: 'center', justifyContent: 'center' },
});
