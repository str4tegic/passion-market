import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'passion-market',
  description: 'La marketplace des makers passionnés',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="fr">
      <body>{children}</body>
    </html>
  )
}
