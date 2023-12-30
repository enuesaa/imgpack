import { useQuery, useMutation, useQueryClient } from 'react-query'
import type { SignApiResponse } from '@/pages/api/sign'

export const useSign = () => useMutation<SignApiResponse>({
    mutationFn: async () => {
      const res = await fetch(`http://localhost:3000/api/sign`, {
        method: 'POST',
      })
      const body = await res.json()
      return body as SignApiResponse
    },
  })
