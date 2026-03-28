# Story 1.2 — Application Next.js 16 & Proxy vers l'API Rust

## Métadonnées

| Champ | Valeur |
|-------|--------|
| **Story ID** | 1.2 |
| **Story Key** | `1-2-application-nextjs-16-et-proxy-vers-lapi-rust` |
| **Epic** | Epic 1 — Socle Projet & Environnement |
| **Statut** | review |
| **Date de création** | 2026-03-28 |

---

## User Story

**En tant que** développeur,
**Je veux** une application Next.js 16 (App Router, TypeScript, Tailwind) configurée pour proxifier vers l'API Rust, organisée en monorepo Nx,
**Afin que** le frontend puisse communiquer avec le backend dès la première feature, et que la structure soit prête pour le mobile post-MVP.

---

## Critères d'Acceptation (BDD)

**CA-1 — Démarrage dev**
- **Given** le répertoire `frontend/apps/web/`,
- **When** `npm run dev` est exécuté depuis `frontend/`,
- **Then** Next.js démarre sur le port 3000 sans erreur TypeScript

**CA-2 — Proxy vers l'API Rust**
- **Given** `next.config.ts` avec les rewrites configurés via variable d'environnement,
- **When** une requête `GET /api/v1/health` est faite depuis le browser,
- **Then** elle est proxifiée vers `http://localhost:3001/api/v1/health`

**CA-3 — Page placeholder**
- **Given** l'App Router,
- **When** la page `/` est visitée,
- **Then** une page placeholder "passion-market — bientôt" est affichée (SSG)
- **And** aucune erreur 404 ou runtime dans la console

**CA-4 — Build production propre**
- **Given** `npm run build` depuis `frontend/apps/web/`,
- **When** le build est lancé,
- **Then** il se complète sans erreur TypeScript ni ESLint

**CA-5 — Packages partagés accessibles**
- **Given** le workspace Nx configuré,
- **When** `apps/web` importe depuis `@passion-market/ui` ou `@passion-market/api-client`,
- **Then** TypeScript résout les imports sans erreur

---

## Contexte Développeur

### Périmètre de cette story

Cette story pose la structure frontend **monorepo Nx** avec :
- `apps/web/` — Next.js 16 App Router
- `packages/ui/` — composants React client partagés (squelette pour l'instant)
- `packages/api-client/` — queries, commands, types DTOs, fetch wrapper (squelette)
- `packages/hooks/` — hooks React avec Context API (squelette)

Pour cette story : **uniquement le squelette et la configuration**. Aucune logique métier, aucun composant réel — juste ce qui permet de valider les 5 CAs.

### Ce que cette story NE fait PAS

- Pas d'implémentation de composants UI réels (Story 4+)
- Pas d'implémentation de l'api-client réel (Story 2+)
- Pas de setup mobile React Native / Expo (post-MVP)
- Pas de CI (Story 1.3)

---

## Architecture Frontend Décidée

### Structure monorepo Nx

```
frontend/                           ← Nx workspace root
├── package.json                    ← workspace npm (workspaces: ["apps/*", "packages/*"])
├── nx.json                         ← configuration Nx
├── tsconfig.base.json              ← tsconfig partagé entre tous les packages
│
├── apps/
│   └── web/                        ← Next.js 16, App Router
│       ├── package.json
│       ├── next.config.ts          ← proxy API_URL + config Next.js
│       ├── tailwind.config.ts
│       ├── tsconfig.json
│       ├── .env.local              ← API_URL=http://localhost:3001
│       ├── .env.example
│       └── src/
│           └── app/
│               ├── layout.tsx      ← root layout
│               └── page.tsx        ← placeholder "bientôt"
│
└── packages/
    ├── ui/                         ← composants React client partagés
    │   ├── package.json            ← name: "@passion-market/ui"
    │   ├── tsconfig.json
    │   └── src/
    │       ├── index.ts            ← exports
    │       └── lib/
    │           └── money.ts        ← MoneyDisplay (VO d'affichage)
    │
    ├── api-client/                 ← fetch wrapper + types DTOs
    │   ├── package.json            ← name: "@passion-market/api-client"
    │   ├── tsconfig.json
    │   └── src/
    │       ├── index.ts
    │       ├── lib/
    │       │   └── fetch.ts        ← apiGet, apiPost avec gestion RFC 7807
    │       ├── types/
    │       │   └── common.ts       ← PageParams, Page<T>, ApiError
    │       ├── queries/            ← (vide — Story 4+)
    │       └── commands/           ← (vide — Story 2+)
    │
    └── hooks/                      ← hooks React + Context API
        ├── package.json            ← name: "@passion-market/hooks"
        ├── tsconfig.json
        └── src/
            └── index.ts
```

### Principe clé : handlers thin dans Next.js

```typescript
// apps/web/src/app/products/page.tsx  ← CORRECT
import { getProducts } from '@passion-market/api-client'
import { ProductCard } from '@passion-market/ui'

export default async function ProductsPage() {
  const products = await getProducts({ page: 1 })
  return products.items.map(p => <ProductCard key={p.id} product={p} />)
}

// apps/web/src/app/products/page.tsx  ← INCORRECT (logique dans le handler)
export default async function ProductsPage() {
  const res = await fetch('/api/v1/products')    // ← fetch direct = interdit
  const data = await res.json()
  return data.items.map((p: any) => ...)          // ← any = interdit
}
```

### Séparation des objets

| Type | Emplacement | Règle |
|------|-------------|-------|
| VO domaine | Rust `*-domain` uniquement | Invariants métier |
| DTO | `packages/api-client/src/types/` | Transport JSON pur, pas de méthodes |
| VO d'affichage | `packages/ui/src/lib/` | Formatage uniquement, jamais de règle métier |
| État UI | `packages/hooks/src/` | Context API, dispatch uniquement |

---

## Exigences Techniques

### Stack & Versions

| Technologie | Version | Notes |
|-------------|---------|-------|
| Node.js | 20 LTS | |
| Next.js | 16.x | App Router obligatoire |
| TypeScript | 5.x | strict mode |
| Tailwind CSS | 3.x | |
| Nx | 20.x | gestionnaire monorepo |

### Stratégie de rendu Next.js

| Page | Mode | Config |
|------|------|--------|
| `/` (accueil) | SSG/ISR | `export const revalidate = 60` |
| `/makers/[slug]` | ISR | `export const revalidate = 60` |
| `/products/[id]` | ISR | `export const revalidate = 60` |
| `/cart`, `/checkout`, `/orders` | SSR | pas de `revalidate` |
| `/dashboard/**` | SSR | pas de `revalidate` |
| `/admin/**` | SSR | pas de `revalidate` |

### Proxy API — via variable d'environnement

```typescript
// apps/web/next.config.ts
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
}
```

```bash
# apps/web/.env.local (développement)
API_URL=http://localhost:3001

# Railway (production) — variable d'environnement Railway
# API_URL=https://ton-app.railway.app
```

**Important :** `API_URL` n'est pas préfixée `NEXT_PUBLIC_` — elle est lue côté serveur uniquement (dans `next.config.ts`). Ne jamais exposer l'URL interne de l'API au client.

---

## Contenu des Fichiers Clés

### `frontend/package.json` (workspace root)

```json
{
  "name": "passion-market-frontend",
  "private": true,
  "workspaces": ["apps/*", "packages/*"],
  "scripts": {
    "dev": "nx run web:dev",
    "build": "nx run web:build",
    "lint": "nx run-many --target=lint"
  },
  "devDependencies": {
    "nx": "^20.0.0",
    "@nx/next": "^20.0.0",
    "typescript": "^5.0.0"
  }
}
```

### `frontend/nx.json`

```json
{
  "$schema": "./node_modules/nx/schemas/nx-schema.json",
  "defaultBase": "main",
  "namedInputs": {
    "default": ["{projectRoot}/**/*", "sharedGlobals"],
    "sharedGlobals": ["{workspaceRoot}/tsconfig.base.json"]
  },
  "targetDefaults": {
    "build": { "cache": true },
    "dev": { "cache": false }
  }
}
```

### `frontend/tsconfig.base.json`

```json
{
  "compilerOptions": {
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "paths": {
      "@passion-market/ui": ["packages/ui/src/index.ts"],
      "@passion-market/api-client": ["packages/api-client/src/index.ts"],
      "@passion-market/hooks": ["packages/hooks/src/index.ts"]
    }
  }
}
```

### `frontend/apps/web/next.config.ts`

```typescript
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
}

export default nextConfig
```

### `frontend/apps/web/src/app/layout.tsx`

```typescript
import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'passion-market',
  description: 'La marketplace des makers passionnés',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="fr">
      <body>{children}</body>
    </html>
  )
}
```

### `frontend/apps/web/src/app/page.tsx`

```typescript
export default function HomePage() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center">
      <h1 className="text-4xl font-bold">passion-market</h1>
      <p className="mt-4 text-gray-500">bientôt</p>
    </main>
  )
}
```

### `frontend/packages/api-client/src/lib/fetch.ts`

```typescript
export class ApiError extends Error {
  constructor(
    public readonly status: number,
    public readonly title: string,
    public readonly detail: string,
  ) {
    super(title)
  }
}

export async function apiGet<T>(url: string): Promise<T> {
  const res = await fetch(url, { cache: 'no-store' })
  if (!res.ok) {
    const body = await res.json()
    throw new ApiError(body.status, body.title, body.detail)
  }
  return res.json() as Promise<T>
}

export async function apiPost<T>(url: string, body: unknown): Promise<T> {
  const res = await fetch(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!res.ok) {
    const err = await res.json()
    throw new ApiError(err.status, err.title, err.detail)
  }
  return res.json() as Promise<T>
}
```

### `frontend/packages/api-client/src/types/common.ts`

```typescript
export interface PageParams {
  page: number
  perPage: number
}

export interface Page<T> {
  items: T[]
  total: number
  page: number
  perPage: number
}
```

### `frontend/packages/ui/src/lib/money.ts`

```typescript
/** VO d'affichage — formatage uniquement, aucune règle métier */
export class MoneyDisplay {
  constructor(
    private readonly cents: number,
    private readonly currency: string = 'EUR',
  ) {}

  format(): string {
    return new Intl.NumberFormat('fr-FR', {
      style: 'currency',
      currency: this.currency,
    }).format(this.cents / 100)
  }

  isZero(): boolean {
    return this.cents === 0
  }

  static fromDto(cents: number, currency = 'EUR'): MoneyDisplay {
    return new MoneyDisplay(cents, currency)
  }
}
```

---

## Ordre d'Implémentation Recommandé

1. Créer `frontend/` avec `package.json` workspace + `nx.json` + `tsconfig.base.json`
2. Créer `apps/web/` via `npx create-next-app@latest` avec les flags appropriés
3. Déplacer le résultat dans `apps/web/`, ajuster `package.json`
4. Créer les 3 `packages/` avec leur `package.json` et `tsconfig.json`
5. Configurer `next.config.ts` avec le proxy `API_URL`
6. Créer `apps/web/.env.local` et `.env.example`
7. Vérifier `npm run dev` → port 3000
8. Vérifier le proxy : `curl http://localhost:3000/api/v1/health` (l'API Rust doit tourner)
9. Vérifier `npm run build` → zéro erreur TypeScript/ESLint

---

## Commandes de Vérification

```bash
# Depuis frontend/
npm install
npm run dev          # Next.js sur :3000

# Test proxy (API Rust doit tourner : cargo run -p app-server)
curl http://localhost:3000/api/v1/health

# Build production
npm run build

# Vérifier que les imports de packages fonctionnent
# (aucune erreur TypeScript dans apps/web/)
```

---

## Points d'Attention

1. **`npx create-next-app`** crée le projet à la racine — bien le déplacer dans `apps/web/`
2. **`API_URL` sans `NEXT_PUBLIC_`** — lue uniquement dans `next.config.ts` côté serveur
3. **`tsconfig.base.json`** doit être référencé par chaque `tsconfig.json` des packages via `"extends": "../../tsconfig.base.json"`
4. **`"use client"`** obligatoire sur tout composant de `packages/ui/` qui utilise des hooks React (useState, useContext, etc.)
5. **Mobile futur** — quand Expo arrive, il suffira d'ajouter `apps/mobile/` et de réutiliser `@passion-market/api-client` et `@passion-market/hooks` directement. Seul `@passion-market/ui` aura des variantes RN.

---

## Définition de "Done"

- [ ] `npm run dev` démarre sur :3000 sans erreur TypeScript
- [ ] Page `/` affiche "passion-market — bientôt"
- [ ] Proxy `/api/*` → `http://localhost:3001` via `API_URL` dans `.env.local`
- [ ] `npm run build` propre (zéro erreur TS + ESLint)
- [ ] Imports `@passion-market/ui`, `@passion-market/api-client`, `@passion-market/hooks` résolus par TypeScript
- [ ] `packages/api-client/src/lib/fetch.ts` contient `apiGet` et `apiPost` avec gestion RFC 7807
- [ ] `packages/ui/src/lib/money.ts` contient `MoneyDisplay`
- [ ] Fichiers `apps/web/.env.example` et `.env.local` présents

---

## Dev Agent Record

### Completion Notes

Implémentation complétée le 2026-03-28 :

- Monorepo Nx initialisé avec 3 packages + 1 app
- `next build` propre, `tsc --noEmit` zéro erreur
- Page `/` sert le HTML "passion-market — bientôt" via SSG
- Proxy `API_URL` configuré dans `next.config.ts`
- Imports `@passion-market/ui`, `@passion-market/api-client`, `@passion-market/hooks` résolus
- `MoneyDisplay` (VO d'affichage) et `apiGet`/`apiPost`/`ApiError` (RFC 7807) implémentés
- `transpilePackages` ajouté à `next.config.ts` pour les packages monorepo

**Note :** Next.js a auto-ajouté `allowJs`, `noEmit`, `resolveJsonModule`, `isolatedModules` au `tsconfig.json` de `apps/web/` — comportement normal du premier `next build`.

---

## Définition de "Done"

- [x] `npm run dev` démarre sur :3000 sans erreur TypeScript
- [x] Page `/` affiche "passion-market — bientôt"
- [x] Proxy `/api/*` → `http://localhost:3001` via `API_URL` dans `.env.local`
- [x] `npm run build` propre (zéro erreur TS + ESLint)
- [x] Imports `@passion-market/ui`, `@passion-market/api-client`, `@passion-market/hooks` résolus par TypeScript
- [x] `packages/api-client/src/lib/fetch.ts` contient `apiGet`, `apiPost`, `apiPatch`, `apiDelete` avec gestion RFC 7807
- [x] `packages/ui/src/lib/money.ts` contient `MoneyDisplay`
- [x] Fichiers `apps/web/.env.example` et `.env.local` présents

---

## File List

- `frontend/package.json`
- `frontend/nx.json`
- `frontend/tsconfig.base.json`
- `frontend/.gitignore`
- `frontend/packages/api-client/package.json`
- `frontend/packages/api-client/tsconfig.json`
- `frontend/packages/api-client/src/index.ts`
- `frontend/packages/api-client/src/lib/fetch.ts`
- `frontend/packages/api-client/src/types/common.ts`
- `frontend/packages/ui/package.json`
- `frontend/packages/ui/tsconfig.json`
- `frontend/packages/ui/src/index.ts`
- `frontend/packages/ui/src/lib/money.ts`
- `frontend/packages/hooks/package.json`
- `frontend/packages/hooks/tsconfig.json`
- `frontend/packages/hooks/src/index.ts`
- `frontend/apps/web/package.json`
- `frontend/apps/web/tsconfig.json`
- `frontend/apps/web/next.config.ts`
- `frontend/apps/web/tailwind.config.ts`
- `frontend/apps/web/postcss.config.mjs`
- `frontend/apps/web/.eslintrc.json`
- `frontend/apps/web/.env.example`
- `frontend/apps/web/.env.local`
- `frontend/apps/web/src/app/globals.css`
- `frontend/apps/web/src/app/layout.tsx`
- `frontend/apps/web/src/app/page.tsx`

## Change Log

| Date | Description |
|------|-------------|
| 2026-03-28 | Story 1.2 — Monorepo Nx, Next.js 16 App Router, packages partagés, proxy API_URL |
