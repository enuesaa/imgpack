import type { NextApiRequest, NextApiResponse } from 'next'
import { Storage } from '@google-cloud/storage'
import { GoogleAuth } from 'google-auth-library'
import process from 'node:process'
import 'dotenv/config'

const bucketName = process.env['IMAGES_BUCKET_NAME'] ?? ''
const keyFilename = process.env['KEY_FILENAME'] ?? ''
const functionUrl = process.env['FUNCTION_URL'] ?? ''

type ApiRequest<T extends {}> = NextApiRequest & {
  body: T
}
export type InvokeApiReq = {
  id: string
}
export type InvokeApiRes = {
  id: string
  converted: boolean
  url: string
}

type FunctionRes = {
  converted: boolean
  filename: string
}
export default async function handler(req: ApiRequest<InvokeApiReq>, res: NextApiResponse<InvokeApiRes>) {
  if (req.method !== 'POST') {
    res.status(404)
    return;
  }

  const id = req.body?.id ?? ''
  if (id === '') {
    res.status(400)
    return;
  }
  console.log('invoke')

  const auth = new GoogleAuth({ keyFilename })
  const client = await auth.getIdTokenClient(functionUrl)
  const response = await client.request<FunctionRes>({
    method: 'POST',
    url: functionUrl,
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({
      filename: id,
    }),
  })

  const converted = response.data?.converted ?? false
  const filename = response.data?.filename ?? '' // todo

  const storage = new Storage({ keyFilename })
  const [url] = await storage
    .bucket(bucketName)
    .file('out.png')
    .getSignedUrl({
      version: 'v4',
      action: 'read',
      expires: Date.now() + 15 * 60 * 1000, // 15 minutes
    })

  res.status(200).json({ id, converted, url })
}
