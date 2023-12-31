import { useListFiles } from '@/lib/api'
import { Table } from '@radix-ui/themes'
import { CompressButton } from './CompressButton'

type Props = {
  path: string;
}
export const ListTable = ({ path }: Props) => {
  const {data: files} = useListFiles(path)

  return (
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
  )
}