// TODO test this
export const calcParentPath = (path: string): string => {
  const splitted = path.split('/')
  splitted.pop()
  return splitted.join('/')
}
