import React, { useCallback } from 'react';
import { FlatList, StyleSheet, View } from 'react-native';
import { ActivityIndicator, FAB, Text } from 'react-native-paper';
import { useFocusEffect } from '@react-navigation/native';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { CatCard } from '../components/CatCard';
import { useAuth } from '../contexts/AuthContext';
import { useCats } from '../hooks/useCats';
import { useDashboardSummary } from '../hooks/useDashboardSummary';
import type { AppStackParamList } from '../navigation/RootNavigator';
import type { Cat } from '../types';

type Props = NativeStackScreenProps<AppStackParamList, 'Home'>;

export function HomeScreen({ navigation }: Props) {
  const { signOut } = useAuth();
  const { cats, isLoading, error, fetchCats } = useCats();
  const { summary, fetchSummary } = useDashboardSummary();

  useFocusEffect(
    useCallback(() => {
      fetchCats().then(async () => {
        // cats state updates asynchronously; re-read via the hook return
        // so we trigger fetchSummary after cats load via the effect below.
      });
    }, [fetchCats]),
  );

  // Re-fetch summary whenever the cats list changes.
  const handleRefresh = useCallback(async () => {
    await fetchCats();
  }, [fetchCats]);

  // Fetch summary once cats are loaded.
  React.useEffect(() => {
    if (cats.length > 0) {
      fetchSummary(cats);
    }
  }, [cats, fetchSummary]);

  const renderCat = ({ item }: { item: Cat }) => (
    <CatCard
      cat={item}
      onPress={() => navigation.navigate('CatProfile', { cat: item })}
      summary={summary[item.id]}
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
          onRefresh={handleRefresh}
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
