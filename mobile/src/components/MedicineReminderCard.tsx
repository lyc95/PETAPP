import React from 'react';
import { StyleSheet, View } from 'react-native';
import { Card, Chip, IconButton, Switch, Text } from 'react-native-paper';
import type { MedicineReminder, ReminderType } from '../types';

interface Props {
  reminder: MedicineReminder;
  onToggleActive: (reminder: MedicineReminder) => void;
  onDelete: (reminder: MedicineReminder) => void;
}

const TYPE_LABELS: Record<ReminderType, string> = {
  MEDICATION: 'Medication',
  NAIL_CUT: 'Nail Trim',
  EAR_WASH: 'Ear Wash',
};

const TYPE_ICONS: Record<ReminderType, string> = {
  MEDICATION: 'pill',
  NAIL_CUT: 'content-cut',
  EAR_WASH: 'water',
};

function formatDate(iso: string): string {
  const d = new Date(iso);
  return d.toLocaleDateString(undefined, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

export function MedicineReminderCard({ reminder, onToggleActive, onDelete }: Props) {
  const typeLabel = TYPE_LABELS[reminder.reminderType] ?? reminder.reminderType;
  const typeIcon = TYPE_ICONS[reminder.reminderType] ?? 'bell';

  return (
    <Card style={[styles.card, !reminder.isActive && styles.inactive]}>
      <Card.Content>
        <View style={styles.header}>
          <Chip icon={typeIcon} compact>
            {typeLabel}
          </Chip>
          <View style={styles.actions}>
            <Switch
              value={reminder.isActive}
              onValueChange={() => onToggleActive(reminder)}
            />
            <IconButton
              icon="delete-outline"
              size={20}
              onPress={() => onDelete(reminder)}
            />
          </View>
        </View>

        <Text variant="titleMedium" style={styles.label}>
          {reminder.label}
        </Text>
        <Text variant="bodySmall" style={styles.date}>
          {formatDate(reminder.scheduledDate)}
        </Text>
        {reminder.isRecurring && reminder.intervalDays != null && (
          <Text variant="bodySmall" style={styles.recurring}>
            Repeats every {reminder.intervalDays} day{reminder.intervalDays !== 1 ? 's' : ''}
          </Text>
        )}
      </Card.Content>
    </Card>
  );
}

const styles = StyleSheet.create({
  card: {
    marginBottom: 12,
  },
  inactive: {
    opacity: 0.5,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 8,
  },
  actions: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  label: {
    marginBottom: 4,
  },
  date: {
    opacity: 0.7,
  },
  recurring: {
    marginTop: 4,
    opacity: 0.6,
    fontStyle: 'italic',
  },
});
