import React, { useCallback, useState } from 'react';
import { Alert, FlatList, StyleSheet, View } from 'react-native';
import {
  ActivityIndicator,
  Button,
  Card,
  Chip,
  Dialog,
  FAB,
  Portal,
  Snackbar,
  Text,
  TextInput,
} from 'react-native-paper';
import { useFocusEffect } from '@react-navigation/native';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { useHealthRecords } from '../hooks/useHealthRecords';
import type { AppStackParamList } from '../navigation/RootNavigator';
import type { HealthRecord, HealthRecordType } from '../types';

type Props = NativeStackScreenProps<AppStackParamList, 'HealthRecords'>;

const RECORD_TYPES: { value: HealthRecordType; label: string; icon: string }[] = [
  { value: 'VACCINATION', label: 'Vaccination', icon: 'needle' },
  { value: 'VET_VISIT', label: 'Vet Visit', icon: 'stethoscope' },
  { value: 'NOTE', label: 'Note', icon: 'note-text' },
];

function typeLabel(type: HealthRecordType): string {
  return RECORD_TYPES.find(t => t.value === type)?.label ?? type;
}

function typeIcon(type: HealthRecordType): string {
  return RECORD_TYPES.find(t => t.value === type)?.icon ?? 'file-document';
}

export function HealthRecordsScreen({ route }: Props) {
  const { cat } = route.params;
  const { records, isLoading, error, fetchRecords, createRecord, deleteRecord } =
    useHealthRecords(cat.id);

  const [dialogVisible, setDialogVisible] = useState(false);
  const [snackbar, setSnackbar] = useState('');

  // Form state
  const [recordType, setRecordType] = useState<HealthRecordType>('VACCINATION');
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [saving, setSaving] = useState(false);

  useFocusEffect(
    useCallback(() => {
      fetchRecords();
    }, [fetchRecords]),
  );

  const resetForm = () => {
    setRecordType('VACCINATION');
    setTitle('');
    setDescription('');
  };

  const handleCreate = async () => {
    if (!title.trim() || !description.trim()) {
      setSnackbar('Title and description are required.');
      return;
    }
    setSaving(true);
    try {
      await createRecord({
        recordType,
        title: title.trim(),
        description: description.trim(),
        recordedAt: new Date().toISOString(),
      });
      setDialogVisible(false);
      resetForm();
    } catch {
      setSnackbar('Failed to create record. Please try again.');
    } finally {
      setSaving(false);
    }
  };

  const handleDelete = (record: HealthRecord) => {
    Alert.alert('Delete Record', `Delete "${record.title}"?`, [
      { text: 'Cancel', style: 'cancel' },
      {
        text: 'Delete',
        style: 'destructive',
        onPress: async () => {
          try {
            await deleteRecord(record.id);
          } catch {
            setSnackbar('Failed to delete record.');
          }
        },
      },
    ]);
  };

  if (isLoading && records.length === 0) {
    return (
      <View style={styles.center}>
        <ActivityIndicator size="large" />
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <FlatList
        data={records}
        keyExtractor={item => item.id}
        renderItem={({ item }) => (
          <Card style={styles.card}>
            <Card.Content>
              <View style={styles.cardHeader}>
                <Chip icon={typeIcon(item.type)} compact>
                  {typeLabel(item.type)}
                </Chip>
                <Button
                  icon="delete-outline"
                  mode="text"
                  compact
                  textColor="red"
                  onPress={() => handleDelete(item)}
                >
                  {''}
                </Button>
              </View>
              <Text variant="titleMedium" style={styles.cardTitle}>
                {item.title}
              </Text>
              <Text variant="bodySmall" style={styles.description}>
                {item.description}
              </Text>
              <Text variant="bodySmall" style={styles.date}>
                {new Date(item.recordedAt).toLocaleDateString(undefined, {
                  year: 'numeric',
                  month: 'short',
                  day: 'numeric',
                })}
              </Text>
            </Card.Content>
          </Card>
        )}
        contentContainerStyle={styles.list}
        ListEmptyComponent={
          <Text style={styles.empty} variant="bodyLarge">
            No records yet. Tap + to add one.
          </Text>
        }
        refreshing={isLoading}
        onRefresh={fetchRecords}
      />

      <FAB icon="plus" style={styles.fab} onPress={() => setDialogVisible(true)} />

      <Portal>
        <Dialog visible={dialogVisible} onDismiss={() => setDialogVisible(false)}>
          <Dialog.Title>New Health Record</Dialog.Title>
          <Dialog.ScrollArea style={styles.dialogScroll}>
            {/* Type selector */}
            <View style={styles.typeRow}>
              {RECORD_TYPES.map(t => (
                <Button
                  key={t.value}
                  mode={recordType === t.value ? 'contained' : 'outlined'}
                  compact
                  style={styles.typeBtn}
                  onPress={() => setRecordType(t.value)}
                >
                  {t.label}
                </Button>
              ))}
            </View>

            <TextInput
              label="Title"
              value={title}
              onChangeText={setTitle}
              mode="outlined"
              style={styles.input}
            />
            <TextInput
              label="Description"
              value={description}
              onChangeText={setDescription}
              mode="outlined"
              multiline
              numberOfLines={3}
              style={styles.input}
            />
          </Dialog.ScrollArea>
          <Dialog.Actions>
            <Button onPress={() => setDialogVisible(false)}>Cancel</Button>
            <Button onPress={handleCreate} loading={saving} disabled={saving}>
              Create
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
  cardHeader: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center', marginBottom: 8 },
  cardTitle: { marginBottom: 4 },
  description: { opacity: 0.8, marginBottom: 4 },
  date: { opacity: 0.55 },
  empty: { textAlign: 'center', marginTop: 40, opacity: 0.6 },
  fab: { position: 'absolute', right: 16, bottom: 16 },
  dialogScroll: { paddingHorizontal: 0 },
  typeRow: { flexDirection: 'row', flexWrap: 'wrap', gap: 8, marginBottom: 12 },
  typeBtn: { flex: 1 },
  input: { marginBottom: 12 },
});
