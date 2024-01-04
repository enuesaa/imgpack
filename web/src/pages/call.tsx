import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { useUserAtomValue } from '@/firebase/auth'
import { Button } from '@radix-ui/themes'
import { MouseEventHandler } from 'react'
import { getFunctions, httpsCallable } from 'firebase/functions'
import { app } from '@/firebase/app'

export default function TopPage() {
  const user = useUserAtomValue()
  console.log(user)

  const handleClick: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    const fncs = getFunctions(app)
    const invoker = httpsCallable(fncs, 'imgpack-convert')
    const res = await invoker({})
    console.log(res)
  }

  return (
    <>
      <Header />
      <Main>
        <Button onClick={handleClick}>invoke</Button>
      </Main>
    </>
  )
}
