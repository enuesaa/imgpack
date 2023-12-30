import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { useSign } from '@/lib/api'
import { Button, Text } from '@radix-ui/themes'
import { MouseEventHandler } from 'react'

export default function TopPage() {
  const sign = useSign()

  const handleSign: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    await sign.mutateAsync()
  }

  return (
    <>
      <Header />
      <Main>
        <Button onClick={handleSign}>get signed url</Button>
        <Text>{sign.data?.url}</Text>
      </Main>
    </>
  )
}
