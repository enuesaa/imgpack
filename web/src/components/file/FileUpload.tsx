import { Card, Section } from '@radix-ui/themes'
import {useDropzone} from 'react-dropzone'
import styles from './FileUpload.css'

export const FileUpload = () => {
  const {acceptedFiles, getRootProps: dropzoneRoot, getInputProps: dropzoneInput} = useDropzone()

  return (
    <Section {...dropzoneRoot()} className={styles.main} mt='4' mb='4' p='5'>
      <input {...dropzoneInput()} />
      {acceptedFiles.map(file => (
        <Card key={file.name}>{file.name}</Card>
      ))}
    </Section>
  )
}
