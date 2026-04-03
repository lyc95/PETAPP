import {
  AuthenticationDetails,
  CognitoUser,
  CognitoUserPool,
  CognitoUserSession,
} from 'amazon-cognito-identity-js';
import { COGNITO_CLIENT_ID, COGNITO_USER_POOL_ID } from '../config/env';

const userPool = new CognitoUserPool({
  UserPoolId: COGNITO_USER_POOL_ID,
  ClientId: COGNITO_CLIENT_ID,
});

function makeUser(email: string): CognitoUser {
  return new CognitoUser({ Username: email, Pool: userPool });
}

export const authService = {
  signIn(email: string, password: string): Promise<CognitoUserSession> {
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
    userPool.getCurrentUser()?.globalSignOut({ onSuccess: () => {}, onFailure: () => {} });
    userPool.getCurrentUser()?.signOut();
  },

  getCurrentSession(): Promise<CognitoUserSession | null> {
    return new Promise((resolve) => {
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
