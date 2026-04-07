import { useCallback, useState } from 'react';
import apiClient from '../services/apiClient';
import type { ApiList, ApiResponse, HealthRecord } from '../types';

const PAGE_SIZE = 50;

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
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [offset, setOffset] = useState(0);
  const [hasMore, setHasMore] = useState(false);

  const fetchRecords = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const res = await apiClient.get<ApiList<HealthRecord>>(`/cats/${catId}/health-records`, {
        params: { limit: PAGE_SIZE, offset: 0 },
      });
      setRecords(res.data.data);
      setOffset(PAGE_SIZE);
      setHasMore(res.data.count === PAGE_SIZE);
    } catch {
      setError('Failed to load health records. Please try again.');
    } finally {
      setIsLoading(false);
    }
  }, [catId]);

  const loadMoreRecords = useCallback(async () => {
    if (!hasMore || isLoadingMore) {
      return;
    }
    setIsLoadingMore(true);
    try {
      const res = await apiClient.get<ApiList<HealthRecord>>(`/cats/${catId}/health-records`, {
        params: { limit: PAGE_SIZE, offset },
      });
      setRecords(prev => [...prev, ...res.data.data]);
      setOffset(prev => prev + PAGE_SIZE);
      setHasMore(res.data.count === PAGE_SIZE);
    } catch {
      setError('Failed to load more health records. Please try again.');
    } finally {
      setIsLoadingMore(false);
    }
  }, [catId, hasMore, isLoadingMore, offset]);

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

  return {
    records,
    isLoading,
    isLoadingMore,
    hasMore,
    error,
    fetchRecords,
    loadMoreRecords,
    createRecord,
    updateRecord,
    deleteRecord,
  };
}
