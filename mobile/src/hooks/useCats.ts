import { useCallback, useState } from 'react';
import apiClient from '../services/apiClient';
import type { ApiList, ApiResponse, Cat } from '../types';

const PAGE_SIZE = 50;

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
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [offset, setOffset] = useState(0);
  const [hasMore, setHasMore] = useState(false);

  const fetchCats = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const res = await apiClient.get<ApiList<Cat>>('/cats', {
        params: { limit: PAGE_SIZE, offset: 0 },
      });
      setCats(res.data.data);
      setOffset(PAGE_SIZE);
      setHasMore(res.data.count === PAGE_SIZE);
    } catch {
      setError('Failed to load cats. Please try again.');
    } finally {
      setIsLoading(false);
    }
  }, []);

  const loadMoreCats = useCallback(async () => {
    if (!hasMore || isLoadingMore) {
      return;
    }
    setIsLoadingMore(true);
    try {
      const res = await apiClient.get<ApiList<Cat>>('/cats', {
        params: { limit: PAGE_SIZE, offset },
      });
      setCats(prev => [...prev, ...res.data.data]);
      setOffset(prev => prev + PAGE_SIZE);
      setHasMore(res.data.count === PAGE_SIZE);
    } catch {
      setError('Failed to load more cats. Please try again.');
    } finally {
      setIsLoadingMore(false);
    }
  }, [hasMore, isLoadingMore, offset]);

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

  return {
    cats,
    isLoading,
    isLoadingMore,
    hasMore,
    error,
    fetchCats,
    loadMoreCats,
    createCat,
    updateCat,
    deleteCat,
  };
}
