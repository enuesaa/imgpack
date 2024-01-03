import { useListFiles } from '@/lib/state'
import { FileUploadZone } from './FileUploadZone'
import { Section } from '@radix-ui/themes'
import { FileCard } from './FileCard'

export const FileUpload = () => {
  const files = useListFiles()

  return (
    <Section p='0'>
      {files.map((f, i) => (
        <FileCard key={i} file={f} />
      ))}
      <FileUploadZone />    
    </Section>
  )
}
