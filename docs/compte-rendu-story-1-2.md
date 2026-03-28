# Compte rendu — Story 1.2 : Application Next.js 16 & Monorepo Nx

> **Pour qui ?** Développeur PHP qui connaît Docker, découvre Rust et Next.js.
> **But :** Expliquer la structure frontend mise en place, pourquoi, et comment ça s'articule.

---

## Vue d'ensemble : qu'est-ce qu'on a construit ?

On a créé toute la structure du frontend. Encore une fois rien de visible pour l'utilisateur — juste une page "bientôt". Mais la structure est là pour accueillir toutes les features futures sans avoir à tout réorganiser.

En PHP/Symfony, c'est l'équivalent d'avoir configuré :
- La structure des bundles
- L'injection de dépendances
- La séparation entre le code métier et les controllers

---

## 1. Nx — le gestionnaire de monorepo

### Pourquoi un monorepo ?

Un **monorepo** c'est un seul dépôt Git qui contient plusieurs projets qui se connaissent entre eux. Ici :

```
frontend/
├── apps/web/          ← l'application Next.js (ce que voit l'utilisateur)
└── packages/          ← les bibliothèques partagées (future app mobile aussi)
    ├── ui/            ← composants visuels React
    ├── api-client/    ← communication avec l'API Rust
    └── hooks/         ← logique React partagée
```

**Pourquoi pas tout dans `apps/web/` ?**

Quand on développera l'app mobile React Native, elle utilisera exactement le même `api-client/` (les appels API) et les mêmes `hooks/` (la logique). Seuls les composants visuels (`ui/`) auront des versions différentes — HTML pour le web, composants natifs pour le mobile.

**Nx** est l'outil qui gère ce monorepo : il comprend les dépendances entre projets, ne recompile que ce qui a changé, et fournit des commandes unifiées.

```bash
# Depuis frontend/ — lance le dev de tous les projets qui en ont besoin
npm run dev

# Équivalent de :
cd apps/web && npm run dev  # mais Nx gère les dépendances
```

---

## 2. La structure des packages

### `packages/api-client/` — Parler à l'API Rust

Ce package contient tout ce qui concerne la communication avec le backend. Rien de React ici — du TypeScript pur. C'est important : ça signifie que l'app mobile pourra l'utiliser exactement pareil.

**Le fetch wrapper (`lib/fetch.ts`) :**

```typescript
// Tu envoies une requête POST...
const result = await apiPost<CartDto>('/api/v1/carts/current/items', {
  productId: '019123-...',
  quantity: 1,
})

// Si l'API Rust retourne une erreur RFC 7807 :
// { "type": "...", "title": "Stock insuffisant", "status": 422, "detail": "..." }
// → une ApiError est levée avec ces informations
```

**RFC 7807** c'est un standard pour les erreurs HTTP. Le backend Rust envoie toujours les erreurs dans ce format. Le frontend les capture toujours de la même façon. Pas de surprises.

**Les types DTOs (`types/common.ts`) :**

```typescript
// PageParams = comment on demande une liste paginée
{ page: 1, perPage: 20 }

// Page<T> = ce que l'API retourne pour une liste
{ items: [...], total: 150, page: 1, perPage: 20 }
```

Ces types sont les **mêmes** que les structs Rust `PageParams` et `Page<T>` dans `shared-kernel` — juste traduits en TypeScript. La règle camelCase s'applique des deux côtés (`perPage` en TS = `per_page` en Rust grâce à `serde rename_all`).

### `packages/ui/` — Les composants visuels

Ce package contiendra les composants React partagés (boutons, cards, formulaires...). Pour l'instant il n'y a qu'un seul élément : `MoneyDisplay`.

**`MoneyDisplay` — pourquoi un objet et pas une fonction ?**

```typescript
const price = MoneyDisplay.fromDto(2990, 'EUR')

price.format()          // "29,90 €"   — pour afficher
price.toDecimalString() // "29.90"     — pour un input
price.isZero()          // false
```

C'est ce qu'on appelle un **VO d'affichage** (Value Object). Il ne contient que de la logique d'affichage — jamais de règles métier.

**Règle importante :** les montants viennent toujours du backend en centimes (entiers). Jamais de float. `2990` = 29,90 €. `MoneyDisplay` fait uniquement la conversion pour l'affichage.

### `packages/hooks/` — La logique React partagée

Vide pour l'instant — il accueillera les hooks React dans les stories suivantes :

```typescript
// Story 2+ : authentification
const { user, login, logout } = useAuth()

// Story 6+ : panier
const { items, total, addItem } = useCart()
```

Ces hooks combinent les appels API (`api-client`) et la gestion d'état (Context API React). Ils sont partagés entre l'app web et l'app mobile future.

---

## 3. `apps/web/` — L'application Next.js

### App Router — comment ça marche ?

En Next.js App Router, la structure des dossiers **définit** les URLs :

```
src/app/
├── page.tsx              → URL: /
├── makers/
│   └── [slug]/
│       └── page.tsx      → URL: /makers/artisan-dupont
└── products/
    └── [id]/
        └── page.tsx      → URL: /products/019123-...
```

C'est différent du routing Symfony où tu configures les routes dans des annotations ou un fichier YAML. Ici l'arborescence de fichiers **est** le routing.

`[slug]` et `[id]` sont des **paramètres dynamiques** — l'équivalent des paramètres de route Symfony `{slug}`.

### Server Components vs Client Components

C'est **la** nouveauté majeure de Next.js 13+. Deux types de composants React :

| | Server Component | Client Component |
|--|-----------------|-----------------|
| S'exécute où ? | Serveur uniquement | Navigateur (et serveur pour le SSR) |
| Peut faire des appels API ? | ✅ Directement | ✅ Via fetch |
| Peut utiliser useState/useEffect ? | ❌ Non | ✅ Oui |
| Déclaration | Par défaut | `"use client"` en première ligne |

```typescript
// apps/web/src/app/products/page.tsx — Server Component (par défaut)
// Tourne côté serveur — peut appeler l'API directement
import { getProducts } from '@passion-market/api-client'

export default async function ProductsPage() {
  const products = await getProducts({ page: 1, perPage: 20 })
  // products est déjà chargé ici, côté serveur
  return <ProductList products={products.items} />
}
```

```typescript
// packages/ui/src/components/AddToCartButton.tsx — Client Component
"use client"  // ← cette ligne le marque comme client component
import { useCart } from '@passion-market/hooks'

export function AddToCartButton({ productId }: { productId: string }) {
  const { addItem } = useCart()
  return <button onClick={() => addItem(productId)}>Ajouter</button>
}
```

**Analogie PHP :** le Server Component c'est comme un Controller Symfony qui fait sa requête Doctrine et passe les données à la vue. Le Client Component c'est le JavaScript dans la vue qui réagit aux clics.

### ISR vs SSR — le rendu selon les pages

```typescript
// pages/makers/[slug]/page.tsx — ISR (cache 60 secondes)
export const revalidate = 60  // ← cette ligne suffit

export default async function MakerPage({ params }) {
  const maker = await getMaker(params.slug)
  return <MakerProfile maker={maker} />
}
```

**ISR (Incremental Static Regeneration) :** la page est générée une fois et mise en cache. Toutes les 60 secondes, si quelqu'un la visite, Next.js regénère la page en arrière-plan. Le visiteur voit toujours une version rapide. Parfait pour les pages publiques (SEO, performance).

**SSR (Server-Side Rendering) :** la page est regénérée à chaque requête. Obligatoire pour les pages avec données utilisateur (panier, commandes, dashboard).

```typescript
// pages/cart/page.tsx — SSR (pas de revalidate = régénère à chaque fois)
export default async function CartPage() {
  const cart = await getCart()  // données de l'utilisateur connecté
  return <CartView cart={cart} />
}
```

**Analogie PHP :** ISR = page en cache Varnish régénérée automatiquement. SSR = Controller Symfony sans cache.

---

## 4. Le proxy vers l'API Rust

### Comment ça marche

```
Browser           Next.js             API Rust
   |                  |                   |
   |-- GET /api/v1/products -->|           |
   |                  |-- GET http://localhost:3001/api/v1/products -->|
   |                  |<-- { items: [...] } --------------------------------|
   |<-- { items: [...] } --|
```

Le navigateur ne sait pas que l'API Rust existe. Il parle à Next.js sur le port 3000. Next.js fait suivre vers l'API Rust sur le port 3001.

**Pourquoi ce proxy ?**
1. Évite les problèmes CORS (Cross-Origin Resource Sharing)
2. L'URL de l'API Rust change entre dev et prod — un seul endroit à configurer
3. Le navigateur ne voit jamais l'URL interne de l'API

### La configuration

```typescript
// apps/web/next.config.ts
async rewrites() {
  const apiUrl = process.env.API_URL ?? 'http://localhost:3001'
  return [{ source: '/api/:path*', destination: `${apiUrl}/api/:path*` }]
}
```

```bash
# apps/web/.env.local (développement)
API_URL=http://localhost:3001

# Railway (production) — variable Railway
API_URL=https://passion-market-api.railway.app
```

**Note importante :** `API_URL` n'a pas le préfixe `NEXT_PUBLIC_`. En Next.js, seules les variables préfixées `NEXT_PUBLIC_` sont envoyées au navigateur. Ici on veut que l'URL de l'API reste côté serveur uniquement — le navigateur ne la voit jamais.

---

## 5. La séparation des responsabilités (rappel)

```
apps/web/src/app/**/page.tsx    ← routing + chargement données (thin)
                                    ↓ importe depuis
packages/api-client/            ← appels API, types DTOs
packages/hooks/                 ← état React, orchestration
packages/ui/                    ← composants visuels, VOs d'affichage
```

**Un handler Next.js doit ressembler à ça :**

```typescript
// apps/web/src/app/products/page.tsx ✅
import { getProducts } from '@passion-market/api-client'
import { ProductCard } from '@passion-market/ui'

export default async function ProductsPage() {
  const products = await getProducts({ page: 1, perPage: 20 })
  return (
    <div>
      {products.items.map(p => <ProductCard key={p.id} product={p} />)}
    </div>
  )
}
```

**Un handler ne doit PAS ressembler à ça :**

```typescript
// apps/web/src/app/products/page.tsx ❌
export default async function ProductsPage() {
  const res = await fetch('/api/v1/products')    // fetch direct = interdit
  const data = await res.json()
  // transformation des données ici = interdit
  const products = data.items.filter((p: any) => p.stock > 0)
  ...
}
```

---

## 6. Lien avec le mobile (post-MVP)

Quand on démarrera React Native, voilà ce que ça donnera :

```
frontend/
├── apps/
│   ├── web/     ← Next.js, AUJOURD'HUI
│   └── mobile/  ← Expo/React Native, POST-MVP
└── packages/
    ├── api-client/   ← partagé web + mobile ✅ (aucun code React)
    ├── hooks/        ← partagé web + mobile ✅ (hooks React universels)
    └── ui/           ← web utilise les composants HTML
                         mobile aura ses propres composants natifs
```

Pour démarrer le mobile, on ajoute simplement `apps/mobile/` et on réutilise `@passion-market/api-client` et `@passion-market/hooks` directement. Aucun refactoring.

---

## 7. Commandes utiles

```bash
# Depuis frontend/

# Démarrer le développement
npm run dev          # Next.js sur http://localhost:3000

# Vérifier les types TypeScript (sans compiler)
npx tsc --noEmit     # depuis apps/web/

# Build de production
npm run build

# Vérifier le proxy (API Rust doit tourner)
curl http://localhost:3000/api/v1/health
# → proxifié vers http://localhost:3001/api/v1/health
```

---

## Résumé rapide

| Ce que tu connais en PHP | L'équivalent ici |
|--------------------------|-----------------|
| Bundle Symfony | Package Nx (`@passion-market/ui`) |
| Service container | Imports TypeScript depuis les packages |
| Controller mince | `apps/web/src/app/**/page.tsx` (thin handler) |
| Template Twig | Composant React (JSX) |
| Cache Varnish (régénération auto) | ISR Next.js (`export const revalidate = 60`) |
| Page sans cache | SSR Next.js (pas de `revalidate`) |
| `.env` Symfony | `.env.local` Next.js |
| Variable d'env côté serveur | Variable sans `NEXT_PUBLIC_` |
| Variable d'env exposée au client | Variable avec `NEXT_PUBLIC_` |
| Formatter prix (Twig filter) | `MoneyDisplay.format()` |
| `composer.json` | `package.json` workspace |
