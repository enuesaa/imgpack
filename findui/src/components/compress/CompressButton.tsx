import { useCompress } from '@/lib/api'
import { Button } from '@radix-ui/themes'
import { MouseEventHandler } from 'react'

type Props = {
  filename: string
  isCompressable: boolean
}
export const CompressButton = ({ filename, isCompressable }: Props) => {
  const compress = useCompress()

  const handleCompress: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    await compress.mutateAsync(filename)
  }

  if (!isCompressable) {
    return (
      <div>not compressable</div>
    )
  }

  if (compress.data?.success === true) {
    return (
      <div>compressed!</div>
    )
  }

  return (
    <Button onClick={handleCompress}>
      compress
    </Button>
  )
}
