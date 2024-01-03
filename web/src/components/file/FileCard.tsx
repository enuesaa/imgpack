import { Box, Card } from '@radix-ui/themes'
import { FileStatus } from './FileStatus'

type Props = {
  file: File
}
export const FileCard = ({ file }: Props) => {
  return (
    <Card mt='3' mb='3'>
      <Box display='inline-block'>
        <FileStatus file={file} />
      </Box>
      <Box display='inline-block'>
        {file.name}
      </Box>
    </Card>
  )
}
