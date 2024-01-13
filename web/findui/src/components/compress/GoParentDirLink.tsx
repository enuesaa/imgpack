import { Link } from '@radix-ui/themes'
import { FaArrowUp } from 'react-icons/fa'

// TODO test this
const calcParentPath = (path: string): string => {
  const splitted = path.split('/')
  splitted.pop()
  return splitted.join('/')
}

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
