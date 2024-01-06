import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { useCompress, useListFiles } from '@/lib/api'
import { Button, Table } from '@radix-ui/themes'
import { MouseEventHandler } from 'react'

export default function Page() {
  const {data: files} = useListFiles()

  return (
    <>
      <Header />
      <Main>
        <Table.Root>
          <Table.Header>
            <Table.Row>
              <Table.ColumnHeaderCell>filename</Table.ColumnHeaderCell>
              <Table.ColumnHeaderCell>compress</Table.ColumnHeaderCell>
            </Table.Row>
          </Table.Header>
          <Table.Body>
            {files?.items.map((f, i) => (
              <Table.Row key={i}>
                <Table.Cell>{f.name}</Table.Cell>
                <Table.Cell><CompressButton filename={f.name} /></Table.Cell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table.Root>
      </Main>
    </>
  )
}

type Props = {
  filename: string
}
const CompressButton = ({ filename }: Props) => {
  const compress = useCompress()

  const handleCompress: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    await compress.mutateAsync(filename)
  }
  console.log(compress.data)

  if (compress.data?.success === true) {
    return (
      <div>compressed!</div>
    )
  }

  return (
    <Button onClick={handleCompress}>
      compress
    </Button>
  )
}
