# Mobile App

## Language and Framework
- TypeScript in strict mode. No `any` types.
- React Native 0.74+. Functional components and hooks only — no class components.
- Navigation: `@react-navigation/native` with stack navigator.

## Screens
- Each screen is a single file in `src/screens/`.
- Screen components are default-exported.
- Use `useEffect` for data fetching on mount.
- Use `useFocusEffect` from React Navigation to refetch data when screen gains focus.

## Hooks
- Custom data hooks go in `src/hooks/` (e.g. `useCats`, `useReminders`).
- Return `{ data, loading, error, refetch }` from data hooks.
- Handle loading and error states in the screen, not the hook.

## API Client
- All API calls go through `src/services/apiClient.ts`.
- `apiClient` is an axios instance with `baseURL` from config.
- Auth interceptor injects `Authorization: Bearer <token>` header on every request.
- Never call `fetch()` directly — always use `apiClient`.

## Auth
- Use `amazon-cognito-identity-js` for sign-in, sign-up, and token management.
- Auth logic lives in `src/services/authService.ts`.
- Store tokens using Cognito SDK's built-in storage.
- On auth failure (401), redirect to SignInScreen.

## Types
- All shared TypeScript interfaces are defined in `src/types/index.ts`.
- Import types from `../types` — do not redefine inline.
- Match the field names and types from the API contract in CatsApp.md.

## Components
- Reusable UI components go in `src/components/`.
- Components receive data via props — no direct API calls inside components.
- Use `StyleSheet.create()` for styles, defined at the bottom of the file.

## Notifications
- Use `@notifee/react-native` for local notification scheduling.
- Notification logic lives in `src/services/notificationService.ts`.
- Re-register all active reminders from backend data on app launch.
- Request notification permissions on first launch.

## File Uploads
- Upload flow: call backend `/uploads/presign` → upload file to S3 via returned URL → save returned `objectKey` with the record.
- Upload logic lives in `src/services/uploadService.ts`.

## Config
- Environment values (API URL, Cognito IDs) go in `src/config/env.ts`.
- Never hardcode URLs or AWS resource IDs in screen or component files.

## Validation
- Run `npx tsc --noEmit` after modifying TypeScript files.
