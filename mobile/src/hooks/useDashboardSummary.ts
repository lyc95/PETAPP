import { useCallback, useState } from 'react';
import apiClient from '../services/apiClient';
import type { ApiList, Cat, MedicineReminder, WeightLog } from '../types';

export interface CatSummary {
  nextMed: MedicineReminder | null;
  lastWeight: WeightLog | null;
}

export type DashboardSummary = Record<string, CatSummary>;

export function useDashboardSummary() {
  const [summary, setSummary] = useState<DashboardSummary>({});
  const [isLoading, setIsLoading] = useState(false);

  const fetchSummary = useCallback(async (cats: Cat[]) => {
    if (cats.length === 0) return;
    setIsLoading(true);
    try {
      const results = await Promise.all(
        cats.map(async cat => {
          const [medsRes, weightsRes] = await Promise.all([
            apiClient
              .get<ApiList<MedicineReminder>>(`/cats/${cat.id}/medicine-reminders`)
              .catch(() => null),
            apiClient
              .get<ApiList<WeightLog>>(`/cats/${cat.id}/weight-logs`)
              .catch(() => null),
          ]);

          const meds = medsRes?.data.data ?? [];
          const weights = weightsRes?.data.data ?? [];

          const now = new Date();
          const upcoming = meds
            .filter(r => r.isActive && new Date(r.scheduledDate) > now)
            .sort(
              (a, b) =>
                new Date(a.scheduledDate).getTime() - new Date(b.scheduledDate).getTime(),
            );

          return {
            catId: cat.id,
            summary: {
              nextMed: upcoming[0] ?? null,
              // weight-logs arrive newest-first from the API
              lastWeight: weights[0] ?? null,
            } satisfies CatSummary,
          };
        }),
      );

      const next: DashboardSummary = {};
      for (const { catId, summary: s } of results) {
        next[catId] = s;
      }
      setSummary(next);
    } finally {
      setIsLoading(false);
    }
  }, []);

  return { summary, isLoading, fetchSummary };
}
