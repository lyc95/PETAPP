import { useCallback, useState } from 'react';
import apiClient from '../services/apiClient';
import type { ApiList, ApiResponse, WeightLog } from '../types';

const PAGE_SIZE = 50;

interface CreateWeightLogInput {
  weightKg: number;
  loggedAt: string;
  note?: string | null;
}

interface UpdateWeightLogInput {
  weightKg?: number;
  loggedAt?: string;
  note?: string | null;
}

export function useWeightLogs(catId: string) {
  const [logs, setLogs] = useState<WeightLog[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [offset, setOffset] = useState(0);
  const [hasMore, setHasMore] = useState(false);

  const fetchLogs = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const res = await apiClient.get<ApiList<WeightLog>>(`/cats/${catId}/weight-logs`, {
        params: { limit: PAGE_SIZE, offset: 0 },
      });
      setLogs(res.data.data);
      setOffset(PAGE_SIZE);
      setHasMore(res.data.count === PAGE_SIZE);
    } catch {
      setError('Failed to load weight logs. Please try again.');
    } finally {
      setIsLoading(false);
    }
  }, [catId]);

  const loadMoreLogs = useCallback(async () => {
    if (!hasMore || isLoadingMore) {
      return;
    }
    setIsLoadingMore(true);
    try {
      const res = await apiClient.get<ApiList<WeightLog>>(`/cats/${catId}/weight-logs`, {
        params: { limit: PAGE_SIZE, offset },
      });
      setLogs(prev => [...prev, ...res.data.data]);
      setOffset(prev => prev + PAGE_SIZE);
      setHasMore(res.data.count === PAGE_SIZE);
    } catch {
      setError('Failed to load more weight logs. Please try again.');
    } finally {
      setIsLoadingMore(false);
    }
  }, [catId, hasMore, isLoadingMore, offset]);

  const createLog = useCallback(
    async (input: CreateWeightLogInput): Promise<WeightLog> => {
      const res = await apiClient.post<ApiResponse<WeightLog>>(
        `/cats/${catId}/weight-logs`,
        input,
      );
      const log = res.data.data;
      setLogs(prev => [log, ...prev]);
      return log;
    },
    [catId],
  );

  const updateLog = useCallback(
    async (id: string, input: UpdateWeightLogInput): Promise<WeightLog> => {
      const res = await apiClient.patch<ApiResponse<WeightLog>>(`/weight-logs/${id}`, input);
      const updated = res.data.data;
      setLogs(prev => prev.map(l => (l.id === id ? updated : l)));
      return updated;
    },
    [],
  );

  const deleteLog = useCallback(async (id: string): Promise<void> => {
    await apiClient.delete(`/weight-logs/${id}`);
    setLogs(prev => prev.filter(l => l.id !== id));
  }, []);

  return {
    logs,
    isLoading,
    isLoadingMore,
    hasMore,
    error,
    fetchLogs,
    loadMoreLogs,
    createLog,
    updateLog,
    deleteLog,
  };
}
