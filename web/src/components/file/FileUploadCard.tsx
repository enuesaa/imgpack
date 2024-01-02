import { Button, Card } from '@radix-ui/themes'
import { useUpload } from '@/lib/api'
import { Text } from '@radix-ui/themes'
import { MouseEventHandler } from 'react'

type Props = {
  file: File
}
export const FileUploadCard = ({ file }: Props) => {
  const result = useUpload(file)

  const handleDownload: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    if (result.url === undefined) {
      return
    }
    const res = await fetch(result.url)
    const img = await res.blob()
    const url = window.URL.createObjectURL(img);
    const link = document.createElement('a')
    link.href = url
    link.download = 'out.png'
    link.click()
  }

  return (
    <Card onClick={(e) => e.stopPropagation()}>
      <Text>
        {file.name}
        {result.converted ? (<Button onClick={handleDownload}>download</Button>) : (<></>)}
      </Text>
    </Card>
  )
}
