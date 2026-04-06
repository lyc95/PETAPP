import apiClient from './apiClient';

// Base64url encode an S3 key (slashes must not appear in URL path segments).
// React Native provides global btoa at runtime via the Hermes engine.
declare function btoa(input: string): string;

function toBase64Url(str: string): string {
  return btoa(unescape(encodeURIComponent(str)))
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=/g, '');
}

export const uploadService = {
  /**
   * Gets a presigned PUT URL from the backend, uploads the file directly to
   * S3, and returns the S3 object key to store on the record.
   */
  async presignAndUpload(
    localUri: string,
    fileName: string,
    contentType: string,
  ): Promise<string> {
    const { data } = await apiClient.post<{
      data: { uploadUrl: string; objectKey: string };
    }>('/uploads/presign', { fileName, contentType });

    const { uploadUrl, objectKey } = data.data;

    // Fetch the file as a blob and PUT it directly to S3.
    const fileRes = await fetch(localUri);
    const blob = await fileRes.blob();

    const uploadRes = await fetch(uploadUrl, {
      method: 'PUT',
      headers: { 'Content-Type': contentType },
      body: blob,
    });

    if (!uploadRes.ok) {
      throw new Error(`S3 upload failed: ${uploadRes.status}`);
    }

    return objectKey;
  },

  /**
   * Returns a short-lived presigned GET URL for an S3 object key.
   */
  async getDownloadUrl(objectKey: string): Promise<string> {
    const encoded = toBase64Url(objectKey);
    const { data } = await apiClient.get<{ data: { downloadUrl: string } }>(
      `/files/${encoded}/url`,
    );
    return data.data.downloadUrl;
  },
};
