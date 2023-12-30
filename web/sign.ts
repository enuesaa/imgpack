import { Storage } from '@google-cloud/storage'
import process from 'node:process'
import 'dotenv/config'

const sourceBucketName = process.env['SOURCE_BUCKET_NAME'] ?? ''
const keyFilename = process.env['KEY_FILENAME'] ?? ''

const storage = new Storage({
  keyFilename: keyFilename,
})

const [url] = await storage
  .bucket(sourceBucketName)
  .file("aaa")
  .getSignedUrl({
    version: 'v4',
    action: 'write',
    contentType: 'application/octet-stream',
    expires: Date.now() + 15 * 60 * 1000, // 15 minutes
  })

console.log(url)
