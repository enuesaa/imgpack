import Link from 'next/link'
import { Container } from '@radix-ui/themes'
import styles from '@/components/common/Header.css'

export const Header = () => {
  return (
    <header className={styles.main}>
      <Container size='4'>
        <Link href='/' className={styles.heading}>
          imgpack
        </Link>
      </Container>
    </header>
  )
}
