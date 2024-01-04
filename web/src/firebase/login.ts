import { getAuth, signInWithEmailAndPassword } from 'firebase/auth'
import { app } from './app'

export type LoginData = {
  email: string;
  password: string;
}
export const login = async (logindata: LoginData): Promise<boolean> => {
  const auth = getAuth(app)
  try {
    const usercred = await signInWithEmailAndPassword(auth, logindata.email, logindata.password)
    console.log(usercred)
  } catch (e) {
    console.log(e)
    return false
  }
  return true
}
