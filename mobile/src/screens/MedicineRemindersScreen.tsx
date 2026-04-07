import React, { useCallback, useState } from 'react';
import { Alert, FlatList, StyleSheet, View } from 'react-native';
import {
  ActivityIndicator,
  Button,
  Dialog,
  FAB,
  Portal,
  Snackbar,
  Switch,
  Text,
  TextInput,
} from 'react-native-paper';
import { useFocusEffect } from '@react-navigation/native';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { MedicineReminderCard } from '../components/MedicineReminderCard';
import { useMedicineReminders } from '../hooks/useMedicineReminders';
import type { AppStackParamList } from '../navigation/RootNavigator';
import type { MedicineReminder, ReminderType } from '../types';

type Props = NativeStackScreenProps<AppStackParamList, 'MedicineReminders'>;

const REMINDER_TYPES: { value: ReminderType; label: string }[] = [
  { value: 'MEDICATION', label: 'Medication' },
  { value: 'NAIL_CUT', label: 'Nail Trim' },
  { value: 'EAR_WASH', label: 'Ear Wash' },
];

export function MedicineRemindersScreen({ route }: Props) {
  const { cat } = route.params;
  const { reminders, isLoading, error, fetchReminders, createReminder, updateReminder, deleteReminder } =
    useMedicineReminders(cat.id, cat.name);

  const [dialogVisible, setDialogVisible] = useState(false);
  const [snackbar, setSnackbar] = useState('');

  // Form state
  const [reminderType, setReminderType] = useState<ReminderType>('MEDICATION');
  const [label, setLabel] = useState('');
  const [scheduledDate, setScheduledDate] = useState('');
  const [isRecurring, setIsRecurring] = useState(false);
  const [intervalDays, setIntervalDays] = useState('');
  const [saving, setSaving] = useState(false);

  useFocusEffect(
    useCallback(() => {
      fetchReminders();
    }, [fetchReminders]),
  );

  const resetForm = () => {
    setReminderType('MEDICATION');
    setLabel('');
    setScheduledDate('');
    setIsRecurring(false);
    setIntervalDays('');
  };

  const handleCreate = async () => {
    if (!label.trim() || !scheduledDate.trim()) {
      setSnackbar('Label and scheduled date are required.');
      return;
    }
    if (isRecurring && !intervalDays.trim()) {
      setSnackbar('Interval days is required for recurring reminders.');
      return;
    }

    setSaving(true);
    try {
      await createReminder({
        reminderType,
        label: label.trim(),
        scheduledDate,
        isRecurring,
        intervalDays: isRecurring ? parseInt(intervalDays, 10) : null,
      });
      setDialogVisible(false);
      resetForm();
    } catch {
      setSnackbar('Failed to create reminder. Please try again.');
    } finally {
      setSaving(false);
    }
  };

  const handleToggleActive = async (reminder: MedicineReminder) => {
    try {
      await updateReminder(reminder.id, { isActive: !reminder.isActive });
    } catch {
      setSnackbar('Failed to update reminder.');
    }
  };

  const handleDelete = (reminder: MedicineReminder) => {
    Alert.alert('Delete Reminder', `Delete "${reminder.label}"?`, [
      { text: 'Cancel', style: 'cancel' },
      {
        text: 'Delete',
        style: 'destructive',
        onPress: async () => {
          try {
            await deleteReminder(reminder.id);
          } catch {
            setSnackbar('Failed to delete reminder.');
          }
        },
      },
    ]);
  };

  if (isLoading && reminders.length === 0) {
    return (
      <View style={styles.center}>
        <ActivityIndicator size="large" />
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <FlatList
        data={reminders}
        keyExtractor={item => item.id}
        renderItem={({ item }) => (
          <MedicineReminderCard
            reminder={item}
            onToggleActive={handleToggleActive}
            onDelete={handleDelete}
          />
        )}
        contentContainerStyle={styles.list}
        ListEmptyComponent={
          <Text style={styles.empty} variant="bodyLarge">
            No reminders yet. Tap + to add one.
          </Text>
        }
        refreshing={isLoading}
        onRefresh={fetchReminders}
      />

      <FAB icon="plus" style={styles.fab} onPress={() => setDialogVisible(true)} />

      <Portal>
        <Dialog visible={dialogVisible} onDismiss={() => setDialogVisible(false)}>
          <Dialog.Title>New Medicine Reminder</Dialog.Title>
          <Dialog.ScrollArea style={styles.dialogScroll}>
            {/* Type selector */}
            <View style={styles.typeRow}>
              {REMINDER_TYPES.map(t => (
                <Button
                  key={t.value}
                  mode={reminderType === t.value ? 'contained' : 'outlined'}
                  compact
                  style={styles.typeBtn}
                  onPress={() => setReminderType(t.value)}
                >
                  {t.label}
                </Button>
              ))}
            </View>

            <TextInput
              label="Label"
              value={label}
              onChangeText={setLabel}
              mode="outlined"
              style={styles.input}
            />
            <TextInput
              label="Scheduled Date (ISO 8601)"
              value={scheduledDate}
              onChangeText={setScheduledDate}
              placeholder="2026-05-01T10:00:00Z"
              mode="outlined"
              style={styles.input}
            />

            <View style={styles.switchRow}>
              <Text variant="bodyMedium">Recurring</Text>
              <Switch value={isRecurring} onValueChange={setIsRecurring} />
            </View>

            {isRecurring && (
              <TextInput
                label="Repeat every (days)"
                value={intervalDays}
                onChangeText={setIntervalDays}
                keyboardType="numeric"
                mode="outlined"
                style={styles.input}
              />
            )}
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
  empty: { textAlign: 'center', marginTop: 40, opacity: 0.6 },
  fab: { position: 'absolute', right: 16, bottom: 16 },
  dialogScroll: { paddingHorizontal: 0 },
  typeRow: { flexDirection: 'row', flexWrap: 'wrap', gap: 8, marginBottom: 12 },
  typeBtn: { flex: 1 },
  input: { marginBottom: 12 },
  switchRow: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    marginBottom: 12,
  },
});
