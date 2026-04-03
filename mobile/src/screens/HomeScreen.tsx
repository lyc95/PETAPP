import React, { useCallback } from 'react';
import { FlatList, StyleSheet, View } from 'react-native';
import { ActivityIndicator, FAB, Text } from 'react-native-paper';
import { useFocusEffect } from '@react-navigation/native';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { CatCard } from '../components/CatCard';
import { useAuth } from '../contexts/AuthContext';
import { useCats } from '../hooks/useCats';
import type { AppStackParamList } from '../navigation/RootNavigator';
import type { Cat } from '../types';

type Props = NativeStackScreenProps<AppStackParamList, 'Home'>;

export function HomeScreen({ navigation }: Props) {
  const { signOut } = useAuth();
  const { cats, isLoading, error, fetchCats } = useCats();

  // Refetch whenever this screen gains focus (covers post-add/edit/delete).
  useFocusEffect(
    useCallback(() => {
      fetchCats();
    }, [fetchCats]),
  );

  const renderCat = ({ item }: { item: Cat }) => (
    <CatCard
      cat={item}
      onPress={() => navigation.navigate('CatProfile', { cat: item })}
    />
  );

  return (
    <View style={styles.container}>
      {isLoading && cats.length === 0 ? (
        <ActivityIndicator style={styles.loader} />
      ) : error ? (
        <Text style={styles.error}>{error}</Text>
      ) : cats.length === 0 ? (
        <Text style={styles.empty}>No cats yet. Tap + to add one!</Text>
      ) : (
        <FlatList
          data={cats}
          keyExtractor={c => c.id}
          renderItem={renderCat}
          contentContainerStyle={styles.list}
          onRefresh={fetchCats}
          refreshing={isLoading}
        />
      )}

      <FAB
        icon="plus"
        style={styles.fab}
        onPress={() => navigation.navigate('AddCat')}
      />

      <FAB
        icon="logout"
        style={styles.signOutFab}
        size="small"
        onPress={signOut}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1 },
  list: { paddingVertical: 8 },
  loader: { flex: 1, justifyContent: 'center' },
  error: { textAlign: 'center', margin: 24, opacity: 0.6 },
  empty: { textAlign: 'center', margin: 48, opacity: 0.5 },
  fab: { position: 'absolute', right: 16, bottom: 16 },
  signOutFab: { position: 'absolute', right: 16, bottom: 84 },
});
