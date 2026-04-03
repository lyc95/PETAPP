import { useCallback, useState } from 'react';
import apiClient from '../services/apiClient';
import type { ApiList, ApiResponse, Cat } from '../types';

interface CreateCatInput {
  name: string;
  breed: string;
  birthdate: string;
  photoKey?: string | null;
}

interface UpdateCatInput {
  name?: string;
  breed?: string;
  birthdate?: string;
  photoKey?: string | null;
}

export function useCats() {
  const [cats, setCats] = useState<Cat[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchCats = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const res = await apiClient.get<ApiList<Cat>>('/cats');
      setCats(res.data.data);
    } catch {
      setError('Failed to load cats. Please try again.');
    } finally {
      setIsLoading(false);
    }
  }, []);

  const createCat = useCallback(async (input: CreateCatInput): Promise<Cat> => {
    const res = await apiClient.post<ApiResponse<Cat>>('/cats', input);
    const cat = res.data.data;
    setCats(prev => [...prev, cat]);
    return cat;
  }, []);

  const updateCat = useCallback(
    async (id: string, input: UpdateCatInput): Promise<Cat> => {
      const res = await apiClient.patch<ApiResponse<Cat>>(`/cats/${id}`, input);
      const updated = res.data.data;
      setCats(prev => prev.map(c => (c.id === id ? updated : c)));
      return updated;
    },
    [],
  );

  const deleteCat = useCallback(async (id: string): Promise<void> => {
    await apiClient.delete(`/cats/${id}`);
    setCats(prev => prev.filter(c => c.id !== id));
  }, []);

  return { cats, isLoading, error, fetchCats, createCat, updateCat, deleteCat };
}
