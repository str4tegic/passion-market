import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  async rewrites() {
    const apiUrl = process.env.API_URL ?? 'http://localhost:3001'
    return [
      {
        source: '/api/:path*',
        destination: `${apiUrl}/api/:path*`,
      },
    ]
  },

  // Permet à Next.js de transpiler les packages du monorepo
  transpilePackages: [
    '@passion-market/ui',
    '@passion-market/api-client',
    '@passion-market/hooks',
  ],
}

export default nextConfig
