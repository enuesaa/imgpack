import { Section } from '@radix-ui/themes'
import { useDropzone } from 'react-dropzone'
import styles from './FileUpload.css'
import { FileUploadCard } from './FileUploadCard'

export const FileUpload = () => {
  const {acceptedFiles, getRootProps: dropzoneRoot, getInputProps: dropzoneInput} = useDropzone({
    accept: {
      'image/*': ['.png', '.jpeg', '.jpg'],
    },
  })

  return (
    <Section {...dropzoneRoot()} className={styles.main} mt='4' mb='4' p='5'>
      <input {...dropzoneInput()} />
      {acceptedFiles.map(file => (
        <FileUploadCard key={file.name} file={file} />
      ))}
    </Section>
  )
}
