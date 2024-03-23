export const normalizePath = (path: string|null): string => {
  if (path === '') {
    return './'
  }
  if (path === null) {
    return './'
  }
  return path
}

// TODO test this
export const calcParentPath = (path: string): string => {
  const splitted = path.split('/')
  splitted.pop()
  return splitted.join('/')
}
