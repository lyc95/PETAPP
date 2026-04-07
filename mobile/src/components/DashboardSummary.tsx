import React from 'react';
import { StyleSheet, View } from 'react-native';
import { Text } from 'react-native-paper';
import type { CatSummary } from '../hooks/useDashboardSummary';

interface Props {
  summary: CatSummary;
}

function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString(undefined, {
    month: 'short',
    day: 'numeric',
  });
}

export function DashboardSummary({ summary }: Props) {
  const { nextMed, lastWeight } = summary;

  if (!nextMed && !lastWeight) {
    return null;
  }

  return (
    <View style={styles.row}>
      {nextMed != null && (
        <Text variant="bodySmall" style={styles.chip}>
          💊 {formatDate(nextMed.scheduledDate)}
        </Text>
      )}
      {lastWeight != null && (
        <Text variant="bodySmall" style={styles.chip}>
          ⚖️ {lastWeight.weightKg} kg
        </Text>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  row: {
    flexDirection: 'row',
    gap: 12,
    paddingHorizontal: 16,
    paddingBottom: 10,
  },
  chip: {
    opacity: 0.65,
  },
});
