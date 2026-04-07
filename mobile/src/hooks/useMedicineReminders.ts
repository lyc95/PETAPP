import { useCallback, useState } from 'react';
import apiClient from '../services/apiClient';
import {
  cancelNotification,
  scheduleMedicineNotification,
} from '../services/notificationService';
import type { ApiList, ApiResponse, MedicineReminder } from '../types';

const PAGE_SIZE = 50;

interface CreateMedicineReminderInput {
  reminderType: string;
  label: string;
  scheduledDate: string;
  isRecurring: boolean;
  intervalDays?: number | null;
}

interface UpdateMedicineReminderInput {
  reminderType?: string;
  label?: string;
  scheduledDate?: string;
  isRecurring?: boolean;
  intervalDays?: number | null;
  isActive?: boolean;
}

export function useMedicineReminders(catId: string, catName: string) {
  const [reminders, setReminders] = useState<MedicineReminder[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [offset, setOffset] = useState(0);
  const [hasMore, setHasMore] = useState(false);

  const fetchReminders = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const res = await apiClient.get<ApiList<MedicineReminder>>(
        `/cats/${catId}/medicine-reminders`,
        { params: { limit: PAGE_SIZE, offset: 0 } },
      );
      setReminders(res.data.data);
      setOffset(PAGE_SIZE);
      setHasMore(res.data.count === PAGE_SIZE);
    } catch {
      setError('Failed to load reminders. Please try again.');
    } finally {
      setIsLoading(false);
    }
  }, [catId]);

  const loadMoreReminders = useCallback(async () => {
    if (!hasMore || isLoadingMore) {
      return;
    }
    setIsLoadingMore(true);
    try {
      const res = await apiClient.get<ApiList<MedicineReminder>>(
        `/cats/${catId}/medicine-reminders`,
        { params: { limit: PAGE_SIZE, offset } },
      );
      setReminders(prev => [...prev, ...res.data.data]);
      setOffset(prev => prev + PAGE_SIZE);
      setHasMore(res.data.count === PAGE_SIZE);
    } catch {
      setError('Failed to load more reminders. Please try again.');
    } finally {
      setIsLoadingMore(false);
    }
  }, [catId, hasMore, isLoadingMore, offset]);

  const createReminder = useCallback(
    async (input: CreateMedicineReminderInput): Promise<MedicineReminder> => {
      const res = await apiClient.post<ApiResponse<MedicineReminder>>(
        `/cats/${catId}/medicine-reminders`,
        input,
      );
      const reminder = res.data.data;
      setReminders(prev => [...prev, reminder]);
      await scheduleMedicineNotification(reminder, catName);
      return reminder;
    },
    [catId, catName],
  );

  const updateReminder = useCallback(
    async (id: string, input: UpdateMedicineReminderInput): Promise<MedicineReminder> => {
      const res = await apiClient.patch<ApiResponse<MedicineReminder>>(
        `/medicine-reminders/${id}`,
        input,
      );
      const updated = res.data.data;
      setReminders(prev => prev.map(r => (r.id === id ? updated : r)));
      await scheduleMedicineNotification(updated, catName);
      return updated;
    },
    [catName],
  );

  const deleteReminder = useCallback(async (id: string): Promise<void> => {
    await apiClient.delete(`/medicine-reminders/${id}`);
    setReminders(prev => prev.filter(r => r.id !== id));
    await cancelNotification(id);
  }, []);

  return {
    reminders,
    isLoading,
    isLoadingMore,
    hasMore,
    error,
    fetchReminders,
    loadMoreReminders,
    createReminder,
    updateReminder,
    deleteReminder,
  };
}
