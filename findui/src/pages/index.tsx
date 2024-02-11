import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { ListTable } from '@/components/compress/ListTable'
import { useRouter } from 'next/router'

export default function Page() {
  const router = useRouter()
  const pathQuery = router.query.path
  const path = typeof pathQuery === 'string' ? pathQuery : './'

  return (
    <>
      <Header />
      <Main>
        <ListTable path={path} />
      </Main>
    </>
  )
}
