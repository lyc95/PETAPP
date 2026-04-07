import apiClient from './apiClient';
import { API_BASE_URL } from '../config/env';

export const uploadService = {
  /**
   * Upload a file to the backend (multipart POST) and return the object key.
   * The key can be used with `getFileUrl` to build a display URL.
   */
  async upload(localUri: string, fileName: string, contentType: string): Promise<string> {
    const formData = new FormData();
    formData.append('file', {
      uri: localUri,
      name: fileName,
      type: contentType,
    } as unknown as Blob);

    const { data } = await apiClient.post<{ data: { objectKey: string } }>(
      '/uploads/file',
      formData,
      { headers: { 'Content-Type': 'multipart/form-data' } },
    );

    return data.data.objectKey;
  },

  /**
   * Build the URL to display a file from its object key.
   * The backend serves files at GET /files/:key.
   */
  getFileUrl(objectKey: string): string {
    return `${API_BASE_URL}/files/${objectKey}`;
  },
};
