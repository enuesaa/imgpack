import { Card } from '@radix-ui/themes'
import { useUpload } from '@/lib/api'
import { Text } from '@radix-ui/themes'

type Props = {
  file: File
}
export const FileUploadCard = ({ file }: Props) => {
  const result = useUpload(file)

  return (
    <Card>
      <Text>{file.name} {result.id ?? ''}</Text>
    </Card>
  )
}
