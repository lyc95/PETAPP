import React, { useState } from 'react';
import { Alert, ScrollView, StyleSheet } from 'react-native';
import { Button, Divider, Snackbar, Text, TextInput } from 'react-native-paper';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { useCats } from '../hooks/useCats';
import type { AppStackParamList } from '../navigation/RootNavigator';
import type { Cat } from '../types';


type Props = NativeStackScreenProps<AppStackParamList, 'CatProfile'>;

export function CatProfileScreen({ route, navigation }: Props) {
  const { cat: initial } = route.params;
  const { updateCat, deleteCat } = useCats();

  const [editing, setEditing] = useState(false);
  const [cat, setCat] = useState<Cat>(initial);
  const [name, setName] = useState(initial.name);
  const [breed, setBreed] = useState(initial.breed);
  const [birthdate, setBirthdate] = useState(initial.birthdate);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');

  const handleSave = async () => {
    if (!name.trim() || !breed.trim() || !birthdate.trim()) {
      setError('All fields are required.');
      return;
    }
    if (!/^\d{4}-\d{2}-\d{2}$/.test(birthdate)) {
      setError('Birthdate must be in YYYY-MM-DD format.');
      return;
    }
    setSaving(true);
    try {
      const updated = await updateCat(cat.id, {
        name: name.trim(),
        breed: breed.trim(),
        birthdate,
      });
      setCat(updated);
      setEditing(false);
    } catch {
      setError('Failed to save changes.');
    } finally {
      setSaving(false);
    }
  };

  const handleDelete = () => {
    Alert.alert(
      'Delete Cat',
      `Are you sure you want to delete ${cat.name}?`,
      [
        { text: 'Cancel', style: 'cancel' },
        {
          text: 'Delete',
          style: 'destructive',
          onPress: async () => {
            try {
              await deleteCat(cat.id);
              navigation.goBack();
            } catch {
              setError('Failed to delete. Please try again.');
            }
          },
        },
      ],
    );
  };

  const handleCancel = () => {
    setName(cat.name);
    setBreed(cat.breed);
    setBirthdate(cat.birthdate);
    setEditing(false);
  };

  return (
    <ScrollView contentContainerStyle={styles.container}>
      {editing ? (
        <>
          <TextInput
            label="Name"
            value={name}
            onChangeText={setName}
            mode="outlined"
            style={styles.input}
          />
          <TextInput
            label="Breed"
            value={breed}
            onChangeText={setBreed}
            mode="outlined"
            style={styles.input}
          />
          <TextInput
            label="Birthdate (YYYY-MM-DD)"
            value={birthdate}
            onChangeText={setBirthdate}
            mode="outlined"
            keyboardType="numbers-and-punctuation"
            style={styles.input}
          />
          <Button
            mode="contained"
            onPress={handleSave}
            loading={saving}
            disabled={saving}
            style={styles.button}
          >
            Save Changes
          </Button>
          <Button mode="outlined" onPress={handleCancel} style={styles.button}>
            Cancel
          </Button>
        </>
      ) : (
        <>
          <Text variant="headlineMedium">{cat.name}</Text>
          <Text variant="bodyLarge" style={styles.field}>
            Breed: {cat.breed}
          </Text>
          <Text variant="bodyLarge" style={styles.field}>
            Birthdate: {cat.birthdate}
          </Text>

          <Divider style={styles.divider} />

          <Button
            mode="contained"
            icon="pencil"
            onPress={() => setEditing(true)}
            style={styles.button}
          >
            Edit
          </Button>
          <Button
            mode="outlined"
            icon="pill"
            onPress={() => navigation.navigate('MedicineReminders', { cat })}
            style={styles.button}
          >
            Medicine Reminders
          </Button>
          <Button
            mode="outlined"
            icon="scale"
            onPress={() => navigation.navigate('WeightLog', { cat })}
            style={styles.button}
          >
            Weight Log
          </Button>
          <Button
            mode="outlined"
            icon="medical-bag"
            onPress={() => navigation.navigate('HealthRecords', { cat })}
            style={styles.button}
          >
            Health Records
          </Button>
          <Button
            mode="outlined"
            icon="delete"
            onPress={handleDelete}
            textColor="red"
            style={styles.button}
          >
            Delete
          </Button>
        </>
      )}

      <Snackbar visible={!!error} onDismiss={() => setError('')} duration={4000}>
        {error}
      </Snackbar>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    padding: 24,
  },
  field: {
    marginTop: 8,
    opacity: 0.8,
  },
  divider: {
    marginVertical: 24,
  },
  input: {
    marginBottom: 12,
  },
  button: {
    marginBottom: 12,
  },
});
