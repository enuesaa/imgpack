import { Header } from '@/components/common/Header'
import { Main } from '@/components/common/Main'
import { CompressButton } from '@/components/compress/CompressButton'
import { useListFiles } from '@/lib/api'
import { Table } from '@radix-ui/themes'

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
                <Table.Cell><CompressButton filename={f.name} isCompressable={f.isCompressable} /></Table.Cell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table.Root>
      </Main>
    </>
  )
}
