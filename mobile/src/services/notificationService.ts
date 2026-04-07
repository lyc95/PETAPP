import notifee, { AndroidImportance, TriggerType } from '@notifee/react-native';
import type { MedicineReminder } from '../types';

const CHANNEL_ID = 'medicine-reminders';

export async function createAndroidChannel(): Promise<void> {
  await notifee.createChannel({
    id: CHANNEL_ID,
    name: 'Medicine Reminders',
    importance: AndroidImportance.HIGH,
  });
}

export async function requestPermission(): Promise<boolean> {
  const settings = await notifee.requestPermission();
  return settings.authorizationStatus >= 1;
}

export async function scheduleMedicineNotification(
  reminder: MedicineReminder,
  catName: string,
): Promise<void> {
  // Cancel any previously scheduled notification for this reminder.
  await cancelNotification(reminder.id);

  if (!reminder.isActive) {
    return;
  }

  const triggerDate = new Date(reminder.scheduledDate);
  if (triggerDate <= new Date()) {
    // Date is in the past — skip scheduling.
    return;
  }

  const typeLabel: Record<string, string> = {
    MEDICATION: 'Medication',
    NAIL_CUT: 'Nail Trim',
    EAR_WASH: 'Ear Wash',
  };

  await notifee.createTriggerNotification(
    {
      id: reminder.id,
      title: `${catName} — ${typeLabel[reminder.reminderType] ?? reminder.reminderType}`,
      body: reminder.label,
      android: { channelId: CHANNEL_ID, pressAction: { id: 'default' } },
    },
    {
      type: TriggerType.TIMESTAMP,
      timestamp: triggerDate.getTime(),
    },
  );
}

export async function cancelNotification(reminderId: string): Promise<void> {
  await notifee.cancelNotification(reminderId);
}
