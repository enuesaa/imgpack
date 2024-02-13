import { Link } from '@radix-ui/themes'
import { FaArrowUp } from 'react-icons/fa'
import { calcParentPath } from '@/lib/path'

type Props = {
  currentPath: string
}
export const GoParentDirLink = ({ currentPath }: Props) => {
  const parentPath = calcParentPath(currentPath)

  return (
    <Link href={`/?path=${parentPath}`} mr='2'>
      <FaArrowUp />
    </Link>
  )
}
