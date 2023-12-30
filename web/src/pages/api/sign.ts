import type { NextApiRequest, NextApiResponse } from 'next'
import { Storage } from '@google-cloud/storage'
import process from 'node:process'
import 'dotenv/config'
import { customAlphabet } from 'nanoid/non-secure'

export type SignApiResponse = {
  url: string
  id: string
}
export default async function handler(req: NextApiRequest, res: NextApiResponse<SignApiResponse>) {
  if (req.method !== 'POST') {
    res.status(404)
    return;
  }

  const nanoid = customAlphabet('1234567890abcdef', 10)
  const id = nanoid()

  const sourceBucketName = process.env['SOURCE_BUCKET_NAME'] ?? ''
  const keyFilename = process.env['KEY_FILENAME'] ?? ''
  const storage = new Storage({ keyFilename })
  const [url] = await storage
    .bucket(sourceBucketName)
    .file(id)
    .getSignedUrl({
      version: 'v4',
      action: 'write',
      contentType: 'application/octet-stream',
      expires: Date.now() + 15 * 60 * 1000, // 15 minutes
    })
  res.status(200).json({ url, id })
}
