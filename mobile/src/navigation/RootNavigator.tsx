import React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';
import { StyleSheet, View } from 'react-native';
import { Text } from 'react-native-paper';

// Placeholder screen — replaced in Phase 2 with auth flow
function BootstrapScreen() {
  return (
    <View style={styles.container}>
      <Text variant="headlineMedium">Cat Care</Text>
      <Text variant="bodyMedium">Phase 1 bootstrap complete</Text>
    </View>
  );
}

const Stack = createStackNavigator();

export function RootNavigator() {
  return (
    <NavigationContainer>
      <Stack.Navigator screenOptions={{ headerShown: false }}>
        <Stack.Screen name="Bootstrap" component={BootstrapScreen} />
      </Stack.Navigator>
    </NavigationContainer>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: 'center',
    justifyContent: 'center',
    gap: 8,
  },
});
