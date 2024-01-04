import { getAuth } from 'firebase/auth'
import { atom, useAtomValue } from 'jotai'
import { app } from './app'

type User = {
  logined: boolean
  uid: null|string
}

const userAtom = atom<User>({
  logined: false,
  uid: null,
})
userAtom.onMount = (setAtom) => {
  const auth = getAuth(app)
  const unsubscriber = auth.onAuthStateChanged(user => {
    if (user === null) {
      setAtom({logined: false, uid: null})
      return;
    }
    
    setAtom({logined: true, uid: user.uid})
  })

  return () => { unsubscriber() }
}

export const useUserAtomValue = () => useAtomValue(userAtom)

