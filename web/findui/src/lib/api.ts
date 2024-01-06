// import { useMutation } from 'react-query'
// import type { SignApiRes } from '@/pages/api/sign'
// import { useEffect } from 'react'
// import type { InvokeApiReq, InvokeApiRes } from '@/pages/api/invoke'

// export const useSign = () =>
//   useMutation<SignApiRes>({
//     mutationFn: async () => {
//       const res = await fetch(`http://localhost:3000/api/sign`, {
//         method: 'POST',
//         headers: {
//           'Content-Type': 'application/json',
//         },
//       })
//       const body = await res.json()
//       return body as SignApiRes
//     },
//   })
