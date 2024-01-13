import { useListFiles } from '@/lib/api'
import { Link, Table } from '@radix-ui/themes'
import { CompressButton } from './CompressButton'
import { GoParentDirLink } from './GoParentDirLink'

type Props = {
  path: string;
}
export const ListTable = ({ path }: Props) => {
  const {data: files} = useListFiles(path)

  return (
    <>
      <Table.Root>
        <Table.Header>
          <Table.Row>
            <Table.ColumnHeaderCell>
              {typeof files?.path !== 'undefined' && files?.path !== '' && (
                <GoParentDirLink currentPath={files?.path} />
              )}
              filename
            </Table.ColumnHeaderCell>
            <Table.ColumnHeaderCell>compress</Table.ColumnHeaderCell>
          </Table.Row>
        </Table.Header>
        <Table.Body>
          {files?.items.map((f, i) => (
            <Table.Row key={i}>
              <Table.Cell>
                {f.isDir ? (<Link href={`/?path=${f.name}`}>~/{f.name}</Link>) : '~/' + f.name}
              </Table.Cell>
              <Table.Cell><CompressButton filename={f.name} isCompressable={f.isCompressable} /></Table.Cell>
            </Table.Row>
          ))}
        </Table.Body>
      </Table.Root>
    </>
  )
}