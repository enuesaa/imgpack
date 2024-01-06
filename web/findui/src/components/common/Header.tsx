import Link from 'next/link'
import { Flex, Box, Container } from '@radix-ui/themes'
import styles from '@/components/common/Header.css'

export const Header = () => {
  return (
    <header className={styles.main}>
      <Container size='4'>
        <Flex>
          <Box grow='1'>
            <Link href='/' className={styles.heading}>
              imgpack
            </Link>
          </Box>
        </Flex>
      </Container>
    </header>
  )
}
