export type ReminderType = 'MEDICATION' | 'NAIL_CUT' | 'EAR_WASH';
export type HealthRecordType = 'VACCINATION' | 'VET_VISIT' | 'NOTE';

export interface Cat {
  id: string;
  ownerId: string;
  name: string;
  breed: string;
  birthdate: string;        // ISO 8601 date
  photoKey: string | null;  // S3 object key
  createdAt: string;        // ISO 8601 datetime
  updatedAt: string;
}

export interface MealReminder {
  id: string;
  catId: string;
  ownerId: string;
  label: string;            // e.g. "Breakfast"
  scheduledTime: string;    // "HH:MM" 24h format
  daysOfWeek: number[];     // 0=Sun 1=Mon ... 6=Sat
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface MedicineReminder {
  id: string;
  catId: string;
  ownerId: string;
  reminderType: ReminderType; // named reminderType to match backend camelCase serialization
  label: string;
  scheduledDate: string;    // ISO 8601 datetime
  isRecurring: boolean;
  intervalDays: number | null;
  isActive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface WeightLog {
  id: string;
  catId: string;
  ownerId: string;
  weightKg: number;
  loggedAt: string;         // ISO 8601 datetime
  note: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface HealthRecord {
  id: string;
  catId: string;
  ownerId: string;
  type: HealthRecordType;
  title: string;
  description: string;
  recordedAt: string;       // ISO 8601 datetime
  attachmentKey: string | null;
  createdAt: string;
  updatedAt: string;
}

// API response envelope types
export interface ApiSuccess<T> {
  data: T;
}

export type ApiResponse<T> = ApiSuccess<T>;

export interface ApiList<T> {
  data: T[];
  count: number;
}

export interface ApiError {
  error: {
    code: string;
    message: string;
  };
}
