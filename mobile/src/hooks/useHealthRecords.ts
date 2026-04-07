import { useCallback, useState } from 'react';
import apiClient from '../services/apiClient';
import type { ApiList, ApiResponse, HealthRecord } from '../types';

interface CreateHealthRecordInput {
  recordType: string;
  title: string;
  description: string;
  recordedAt: string;
  attachmentKey?: string | null;
}

interface UpdateHealthRecordInput {
  recordType?: string;
  title?: string;
  description?: string;
  recordedAt?: string;
  attachmentKey?: string | null;
}

export function useHealthRecords(catId: string) {
  const [records, setRecords] = useState<HealthRecord[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchRecords = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const res = await apiClient.get<ApiList<HealthRecord>>(`/cats/${catId}/health-records`);
      setRecords(res.data.data);
    } catch {
      setError('Failed to load health records. Please try again.');
    } finally {
      setIsLoading(false);
    }
  }, [catId]);

  const createRecord = useCallback(
    async (input: CreateHealthRecordInput): Promise<HealthRecord> => {
      const res = await apiClient.post<ApiResponse<HealthRecord>>(
        `/cats/${catId}/health-records`,
        input,
      );
      const record = res.data.data;
      setRecords(prev => [record, ...prev]);
      return record;
    },
    [catId],
  );

  const updateRecord = useCallback(
    async (id: string, input: UpdateHealthRecordInput): Promise<HealthRecord> => {
      const res = await apiClient.patch<ApiResponse<HealthRecord>>(
        `/health-records/${id}`,
        input,
      );
      const updated = res.data.data;
      setRecords(prev => prev.map(r => (r.id === id ? updated : r)));
      return updated;
    },
    [],
  );

  const deleteRecord = useCallback(async (id: string): Promise<void> => {
    await apiClient.delete(`/health-records/${id}`);
    setRecords(prev => prev.filter(r => r.id !== id));
  }, []);

  return { records, isLoading, error, fetchRecords, createRecord, updateRecord, deleteRecord };
}
