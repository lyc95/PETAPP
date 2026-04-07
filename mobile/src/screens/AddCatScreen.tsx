import React, { useState } from 'react';
import { ScrollView, StyleSheet } from 'react-native';
import { Button, Snackbar, Text, TextInput } from 'react-native-paper';
import {
  launchImageLibrary,
  type ImagePickerResponse,
} from 'react-native-image-picker';
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import { useCats } from '../hooks/useCats';
import { uploadService } from '../services/uploadService';
import type { AppStackParamList } from '../navigation/RootNavigator';

type Props = NativeStackScreenProps<AppStackParamList, 'AddCat'>;

export function AddCatScreen({ navigation }: Props) {
  const { createCat } = useCats();
  const [name, setName] = useState('');
  const [breed, setBreed] = useState('');
  const [birthdate, setBirthdate] = useState('');
  const [photoKey, setPhotoKey] = useState<string | null>(null);
  const [uploading, setUploading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');

  const pickPhoto = async () => {
    const result: ImagePickerResponse = await launchImageLibrary({
      mediaType: 'photo',
      quality: 0.8,
    });

    if (result.didCancel || !result.assets?.[0]) {
      return;
    }

    const asset = result.assets[0];
    if (!asset.uri || !asset.fileName || !asset.type) {
      return;
    }

    setUploading(true);
    try {
      const key = await uploadService.upload(asset.uri, asset.fileName, asset.type);
      setPhotoKey(key);
    } catch {
      setError('Photo upload failed. Please try again.');
    } finally {
      setUploading(false);
    }
  };

  const handleSave = async () => {
    if (!name.trim() || !breed.trim() || !birthdate.trim()) {
      setError('Please fill in name, breed and birthdate.');
      return;
    }
    if (!/^\d{4}-\d{2}-\d{2}$/.test(birthdate)) {
      setError('Birthdate must be in YYYY-MM-DD format.');
      return;
    }

    setSaving(true);
    try {
      await createCat({
        name: name.trim(),
        breed: breed.trim(),
        birthdate,
        photoKey,
      });
      navigation.goBack();
    } catch {
      setError('Failed to save. Please try again.');
    } finally {
      setSaving(false);
    }
  };

  return (
    <ScrollView contentContainerStyle={styles.container}>
      <Text variant="titleLarge" style={styles.title}>
        Add a Cat
      </Text>

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
        mode="outlined"
        icon="camera"
        onPress={pickPhoto}
        loading={uploading}
        disabled={uploading || saving}
        style={styles.input}
      >
        {photoKey ? 'Photo selected ✓' : 'Add Photo (optional)'}
      </Button>

      <Button
        mode="contained"
        onPress={handleSave}
        loading={saving}
        disabled={saving || uploading}
        style={styles.saveButton}
      >
        Save
      </Button>

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
  title: {
    marginBottom: 20,
  },
  input: {
    marginBottom: 12,
  },
  saveButton: {
    marginTop: 8,
  },
});
