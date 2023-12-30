import { globalStyle, style } from '@vanilla-extract/css'

const main = style({
  height: '50px',
  lineHeight: '50px',
  fontSize: '25px',
  fontWeight: 'bold',
  marginBottom: '20px',
})

export default {
  main,
  heading: style({
    color: 'white',
    margin: '0 10px',
    textDecoration: 'none',
  }),
}