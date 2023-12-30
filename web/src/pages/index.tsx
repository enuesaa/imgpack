import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { FileUpload } from '@/components/file/FileUpload'

export default function TopPage() {
  return (
    <>
      <Header />
      <Main>
        <FileUpload />
      </Main>
    </>
  )
}
