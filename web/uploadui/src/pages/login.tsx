import { type LoginData, login } from '@/firebase/login'
import { Button, Flex, Text, TextField } from '@radix-ui/themes'
import { useForm } from 'react-hook-form'

export default function Page() {
  const { register, handleSubmit } = useForm<LoginData>()

  return (
    <form onSubmit={handleSubmit(async (data) => await login(data))}>
      <Flex direction='column' gap='3'>
        <label>
          <Text as='div' size='2' mb='1' weight='bold'>Email</Text>
          <TextField.Input {...register('email')} />
        </label>
        <label>
          <Text as='div' size='2' mb='1' weight='bold'>Password</Text>
          <TextField.Input type='password' {...register('password')} />
        </label>

        <Button type='submit'>login</Button>
      </Flex>
    </form>
  )
}
