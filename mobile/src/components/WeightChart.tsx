import React from 'react';
import { StyleSheet, View } from 'react-native';
import { LineChart } from 'react-native-gifted-charts';
import { Text } from 'react-native-paper';
import type { WeightLog } from '../types';

interface Props {
  logs: WeightLog[];
}

export function WeightChart({ logs }: Props) {
  if (logs.length < 2) {
    return (
      <View style={styles.empty}>
        <Text variant="bodySmall" style={styles.emptyText}>
          Add at least 2 entries to see the chart.
        </Text>
      </View>
    );
  }

  // logs arrive newest-first from the API — reverse for chart (oldest → newest left→right)
  const ordered = [...logs].reverse();
  const data = ordered.map(l => ({ value: l.weightKg }));

  const weights = ordered.map(l => l.weightKg);
  const min = Math.min(...weights);
  const max = Math.max(...weights);

  return (
    <View style={styles.container}>
      <LineChart
        data={data}
        width={280}
        height={160}
        color="#6750A4"
        thickness={2}
        dataPointsColor="#6750A4"
        startFillColor="rgba(103,80,164,0.2)"
        endFillColor="rgba(103,80,164,0)"
        areaChart
        hideRules
        yAxisOffset={Math.max(0, min - 0.5)}
        maxValue={max + 0.5}
        noOfSections={4}
        yAxisTextStyle={styles.axisText}
        xAxisLabelTextStyle={styles.axisText}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    marginVertical: 8,
  },
  empty: {
    padding: 16,
    alignItems: 'center',
  },
  emptyText: {
    opacity: 0.6,
    fontStyle: 'italic',
  },
  axisText: {
    fontSize: 10,
    color: '#888',
  },
});
