import type { NextApiRequest, NextApiResponse } from 'next'
import { Storage } from '@google-cloud/storage'
import process from 'node:process'
import { customAlphabet } from 'nanoid/non-secure'
import 'dotenv/config'

const bucketName = process.env['IMAGES_BUCKET_NAME'] ?? ''
const keyFilename = process.env['KEY_FILENAME'] ?? ''

export type SignApiReq = {}
export type SignApiRes = {
  url: string
  id: string
}
export default async function handler(req: NextApiRequest, res: NextApiResponse<SignApiRes>) {
  if (req.method !== 'POST') {
    res.status(404)
    return;
  }

  const nanoid = customAlphabet('1234567890abcdef', 10)
  const id = nanoid()
  const filename = `input/${id}`

  const storage = new Storage({ keyFilename })
  const [url] = await storage
    .bucket(bucketName)
    .file(filename)
    .getSignedUrl({
      version: 'v4',
      action: 'write',
      contentType: 'image/png',
      expires: Date.now() + 15 * 60 * 1000, // 15 minutes
    })

  res.status(200).json({ url, id })
}
