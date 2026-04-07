import { useCallback, useState } from 'react';
import apiClient from '../services/apiClient';
import type { ApiList, ApiResponse, WeightLog } from '../types';

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
  const [error, setError] = useState<string | null>(null);

  const fetchLogs = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const res = await apiClient.get<ApiList<WeightLog>>(`/cats/${catId}/weight-logs`);
      setLogs(res.data.data);
    } catch {
      setError('Failed to load weight logs. Please try again.');
    } finally {
      setIsLoading(false);
    }
  }, [catId]);

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

  const updateLog = useCallback(async (id: string, input: UpdateWeightLogInput): Promise<WeightLog> => {
    const res = await apiClient.patch<ApiResponse<WeightLog>>(`/weight-logs/${id}`, input);
    const updated = res.data.data;
    setLogs(prev => prev.map(l => (l.id === id ? updated : l)));
    return updated;
  }, []);

  const deleteLog = useCallback(async (id: string): Promise<void> => {
    await apiClient.delete(`/weight-logs/${id}`);
    setLogs(prev => prev.filter(l => l.id !== id));
  }, []);

  return { logs, isLoading, error, fetchLogs, createLog, updateLog, deleteLog };
}
