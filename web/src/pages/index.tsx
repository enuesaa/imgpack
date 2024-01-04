import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { FileUpload } from '@/components/file/FileUpload'
import { useUserAtomValue } from '@/firebase/auth'

export default function TopPage() {
  const user = useUserAtomValue()
  console.log(user)

  return (
    <>
      <Header />
      <Main>
        <FileUpload />
      </Main>
    </>
  )
}
