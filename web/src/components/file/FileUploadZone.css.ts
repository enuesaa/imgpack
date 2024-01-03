import { style } from '@vanilla-extract/css'

export default {
  main: style({
    border: 'dotted 3px #fafafa',
    borderRadius: '15px',
    minHeight: '50vh',
    textAlign: 'center',
    color: 'var(--gray-a11)',
  }),
  input: style({
    display: 'inline-block',
    margin: '50px 0',
  })
}
