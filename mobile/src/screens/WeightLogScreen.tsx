import React, { useCallback, useState } from 'react';
import { Alert, FlatList, StyleSheet, View } from 'react-native';
import {
  ActivityIndicator,
  Button,
  Card,
  Dialog,
  FAB,
  Portal,
  Snackbar,
  Text,
  TextInput,
} from 'react-native-paper';
import { useFocusEffect } from '@react-navigation/native';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { WeightChart } from '../components/WeightChart';
import { useWeightLogs } from '../hooks/useWeightLogs';
import type { AppStackParamList } from '../navigation/RootNavigator';
import type { WeightLog } from '../types';

type Props = NativeStackScreenProps<AppStackParamList, 'WeightLog'>;

export function WeightLogScreen({ route }: Props) {
  const { cat } = route.params;
  const { logs, isLoading, error, fetchLogs, createLog, deleteLog } = useWeightLogs(cat.id);

  const [dialogVisible, setDialogVisible] = useState(false);
  const [snackbar, setSnackbar] = useState('');
  const [weightKg, setWeightKg] = useState('');
  const [note, setNote] = useState('');
  const [saving, setSaving] = useState(false);

  useFocusEffect(
    useCallback(() => {
      fetchLogs();
    }, [fetchLogs]),
  );

  const handleCreate = async () => {
    const kg = parseFloat(weightKg);
    if (isNaN(kg) || kg <= 0) {
      setSnackbar('Enter a valid weight in kg.');
      return;
    }
    setSaving(true);
    try {
      await createLog({
        weightKg: kg,
        loggedAt: new Date().toISOString(),
        note: note.trim() || null,
      });
      setDialogVisible(false);
      setWeightKg('');
      setNote('');
    } catch {
      setSnackbar('Failed to save weight. Please try again.');
    } finally {
      setSaving(false);
    }
  };

  const handleDelete = (log: WeightLog) => {
    Alert.alert('Delete Entry', `Delete this weight entry (${log.weightKg} kg)?`, [
      { text: 'Cancel', style: 'cancel' },
      {
        text: 'Delete',
        style: 'destructive',
        onPress: async () => {
          try {
            await deleteLog(log.id);
          } catch {
            setSnackbar('Failed to delete entry.');
          }
        },
      },
    ]);
  };

  if (isLoading && logs.length === 0) {
    return (
      <View style={styles.center}>
        <ActivityIndicator size="large" />
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <FlatList
        data={logs}
        keyExtractor={item => item.id}
        ListHeaderComponent={<WeightChart logs={logs} />}
        renderItem={({ item }) => (
          <Card style={styles.card}>
            <Card.Content style={styles.cardContent}>
              <View>
                <Text variant="titleMedium">{item.weightKg} kg</Text>
                <Text variant="bodySmall" style={styles.date}>
                  {new Date(item.loggedAt).toLocaleDateString(undefined, {
                    year: 'numeric',
                    month: 'short',
                    day: 'numeric',
                  })}
                </Text>
                {item.note != null && (
                  <Text variant="bodySmall" style={styles.note}>
                    {item.note}
                  </Text>
                )}
              </View>
              <Button
                icon="delete-outline"
                mode="text"
                compact
                textColor="red"
                onPress={() => handleDelete(item)}
              >
                {''}
              </Button>
            </Card.Content>
          </Card>
        )}
        contentContainerStyle={styles.list}
        ListEmptyComponent={
          <Text style={styles.empty} variant="bodyLarge">
            No entries yet. Tap + to log weight.
          </Text>
        }
        refreshing={isLoading}
        onRefresh={fetchLogs}
      />

      <FAB icon="plus" style={styles.fab} onPress={() => setDialogVisible(true)} />

      <Portal>
        <Dialog visible={dialogVisible} onDismiss={() => setDialogVisible(false)}>
          <Dialog.Title>Log Weight</Dialog.Title>
          <Dialog.Content>
            <TextInput
              label="Weight (kg)"
              value={weightKg}
              onChangeText={setWeightKg}
              keyboardType="decimal-pad"
              mode="outlined"
              style={styles.input}
            />
            <TextInput
              label="Note (optional)"
              value={note}
              onChangeText={setNote}
              mode="outlined"
              style={styles.input}
            />
          </Dialog.Content>
          <Dialog.Actions>
            <Button onPress={() => setDialogVisible(false)}>Cancel</Button>
            <Button onPress={handleCreate} loading={saving} disabled={saving}>
              Save
            </Button>
          </Dialog.Actions>
        </Dialog>
      </Portal>

      <Snackbar visible={!!snackbar} onDismiss={() => setSnackbar('')} duration={4000}>
        {snackbar || error || ''}
      </Snackbar>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1 },
  center: { flex: 1, alignItems: 'center', justifyContent: 'center' },
  list: { padding: 16 },
  card: { marginBottom: 12 },
  cardContent: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center' },
  date: { opacity: 0.6, marginTop: 2 },
  note: { opacity: 0.7, fontStyle: 'italic', marginTop: 4 },
  empty: { textAlign: 'center', marginTop: 40, opacity: 0.6 },
  fab: { position: 'absolute', right: 16, bottom: 16 },
  input: { marginBottom: 12 },
});
