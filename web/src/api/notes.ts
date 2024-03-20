type Note = {
  name: string;
}
export const listNotes = async (): Promise<Note[]> => {
  return [{name: 'b'}]
}
