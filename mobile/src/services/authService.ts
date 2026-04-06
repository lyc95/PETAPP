import AsyncStorage from '@react-native-async-storage/async-storage';
import {
  AuthenticationDetails,
  CognitoUser,
  CognitoUserPool,
  CognitoUserSession,
} from 'amazon-cognito-identity-js';
import { COGNITO_CLIENT_ID, COGNITO_USER_POOL_ID, LOCAL_MODE } from '../config/env';

const LOCAL_SESSION_KEY = '@catcare/local_session';

const userPool = LOCAL_MODE
  ? null
  : new CognitoUserPool({ UserPoolId: COGNITO_USER_POOL_ID, ClientId: COGNITO_CLIENT_ID });

function makeUser(email: string): CognitoUser {
  if (!userPool) { throw new Error('Cognito is not configured'); }
  return new CognitoUser({ Username: email, Pool: userPool });
}

export const authService = {
  signIn(email: string, password: string): Promise<CognitoUserSession> {
    if (LOCAL_MODE) {
      // Accept any credentials locally — store a flag so session persists across restarts.
      return AsyncStorage.setItem(LOCAL_SESSION_KEY, email).then(
        () => null as unknown as CognitoUserSession,
      );
    }
    return new Promise((resolve, reject) => {
      const details = new AuthenticationDetails({
        Username: email,
        Password: password,
      });
      makeUser(email).authenticateUser(details, {
        onSuccess: resolve,
        onFailure: reject,
      });
    });
  },

  signUp(email: string, password: string): Promise<void> {
    return new Promise((resolve, reject) => {
      if (!userPool) { reject(new Error('Cognito is not configured')); return; }
      userPool.signUp(email, password, [], [], (err) => {
        if (err) {
          reject(err);
        } else {
          resolve();
        }
      });
    });
  },

  confirmSignUp(email: string, code: string): Promise<void> {
    return new Promise((resolve, reject) => {
      makeUser(email).confirmRegistration(code, true, (err) => {
        if (err) {
          reject(err);
        } else {
          resolve();
        }
      });
    });
  },

  signOut(): void {
    if (LOCAL_MODE) {
      AsyncStorage.removeItem(LOCAL_SESSION_KEY);
      return;
    }
    userPool?.getCurrentUser()?.globalSignOut({ onSuccess: () => {}, onFailure: () => {} });
    userPool?.getCurrentUser()?.signOut();
  },

  getCurrentSession(): Promise<CognitoUserSession | null> {
    if (LOCAL_MODE) {
      return AsyncStorage.getItem(LOCAL_SESSION_KEY).then(
        v => v ? (true as unknown as CognitoUserSession) : null,
      );
    }
    return new Promise((resolve) => {
      if (!userPool) { resolve(null); return; }
      const user = userPool.getCurrentUser();
      if (!user) {
        resolve(null);
        return;
      }
      user.getSession((err: Error | null, session: CognitoUserSession | null) => {
        if (err || !session?.isValid()) {
          resolve(null);
        } else {
          resolve(session);
        }
      });
    });
  },

  async getAccessToken(): Promise<string | null> {
    const session = await this.getCurrentSession();
    return session?.getAccessToken().getJwtToken() ?? null;
  },

  refreshSession(): Promise<string | null> {
    return new Promise((resolve) => {
      if (!userPool) { resolve(null); return; }
      const user = userPool.getCurrentUser();
      if (!user) {
        resolve(null);
        return;
      }
      user.getSession((err: Error | null, session: CognitoUserSession | null) => {
        if (err || !session) {
          resolve(null);
          return;
        }
        user.refreshSession(session.getRefreshToken(), (refreshErr, newSession) => {
          if (refreshErr || !newSession) {
            resolve(null);
          } else {
            resolve((newSession as CognitoUserSession).getAccessToken().getJwtToken());
          }
        });
      });
    });
  },
};
