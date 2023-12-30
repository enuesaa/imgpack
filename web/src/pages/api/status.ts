import type { NextApiRequest, NextApiResponse } from 'next'
import { Storage } from '@google-cloud/storage'
import process from 'node:process'
import 'dotenv/config'

export type StatusApiResponse = {
  url: string
}
export default async function handler(req: NextApiRequest, res: NextApiResponse<StatusApiResponse>) {
  if (req.method !== 'POST') {
    res.status(404)
    return;
  }
  const id = req.body?.id

  const destBucketName = process.env['DEST_BUCKET_NAME'] ?? ''
  const keyFilename = process.env['KEY_FILENAME'] ?? ''
  const storage = new Storage({ keyFilename })
  const [url] = await storage
    .bucket(destBucketName)
    .file(`${id}-status.json`)
    .getSignedUrl({
      version: 'v4',
      action: 'read',
      contentType: 'image/png',
      expires: Date.now() + 15 * 60 * 1000, // 15 minutes
    })
  res.status(200).json({ url })
}
