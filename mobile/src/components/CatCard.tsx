import React from 'react';
import { StyleSheet } from 'react-native';
import { Card } from 'react-native-paper';
import type { Cat } from '../types';

interface Props {
  cat: Cat;
  onPress: () => void;
}

export function CatCard({ cat, onPress }: Props) {
  return (
    <Card style={styles.card} onPress={onPress} mode="elevated">
      <Card.Title
        title={cat.name}
        subtitle={cat.breed}
        left={props => (
          <Card.Cover
            {...props}
            style={styles.avatar}
            source={require('../assets/cat-placeholder.png')}
          />
        )}
      />
    </Card>
  );
}

const styles = StyleSheet.create({
  card: {
    marginHorizontal: 16,
    marginVertical: 6,
  },
  avatar: {
    width: 40,
    height: 40,
    borderRadius: 20,
  },
});
