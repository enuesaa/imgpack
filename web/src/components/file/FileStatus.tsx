import { Button } from '@radix-ui/themes'
import { useUpload } from '@/lib/api'
import { MouseEventHandler } from 'react'

type Props = {
  file: File
}
export const FileStatus = ({ file }: Props) => {
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
    link.download = `imgpack-${file.name}`
    link.click()
  }

  if (result.success === false) {
    return (
      <Button disabled variant='solid' size='1' mr='3'>
        converting
      </Button>
    )
  }

  return (
    <Button onClick={handleDownload} variant='solid' size='1' mr='3' style={{ cursor: 'pointer' }}>
      download
    </Button>
  )
}
