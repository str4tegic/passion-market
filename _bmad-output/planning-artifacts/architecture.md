---
stepsCompleted: [1, 2, 3, 4, 5, 6, 7, 8]
lastStep: 8
status: 'complete'
completedAt: '2026-03-28'
inputDocuments:
  - "_bmad-output/planning-artifacts/prd.md"
  - "docs/catalog-domain.md"
workflowType: 'architecture'
project_name: 'passion-market'
user_name: 'Francois'
date: '2026-03-28'
---

# Architecture Decision Document

_Ce document se construit collaborativement étape par étape. Les sections sont ajoutées au fil des décisions architecturales._

## Analyse du Contexte Projet

### Vue d'ensemble des exigences

**Exigences Fonctionnelles (37 au total) :**

- Identité & Comptes (FR1–FR6) : inscription maker/acheteur, authentification, profil maker (storytelling, photo atelier, vidéo fabrication)
- Administration & Validation (FR7–FR10) : file admin, approuver/refuser maker, notification email du résultat
- Boutique & Catalogue (FR11–FR18) : création boutique, cycle de vie produit (Draft → Published → OutOfStock), gestion stock automatique
- Découverte & Consultation (FR19–FR23) : page profil maker publique SEO, catalogue, favoris, alerte disponibilité
- Tunnel d'achat & Paiement (FR24–FR30) : panier persistant, commande Stripe, commission + mécanique Weekend Maker
- Notifications & Communication (FR31–FR33) : SSE maker temps réel, relance panier abandonné, infos expédition
- Conformité & Données (FR34–FR37) : RGPD, droit de rétractation 14j, zéro donnée carte en base

**Exigences Non-Fonctionnelles :**

- Performance : LCP < 2.5s, pages publiques < 2s sur 4G, actions critiques (panier, paiement) < 3s
- Sécurité : HTTPS TLS 1.2+, argon2/bcrypt, PCI-DSS via Stripe, RGPD, adresses livrées uniquement au maker concerné
- Accessibilité : WCAG 2.1 AA — navigation clavier, lecteurs d'écran (NVDA/VoiceOver), sous-titres vidéos de fabrication
- Maintenabilité : tests domaine Rust ≥ 80% (TDD pur — chaque test = un invariant domaine), BCs indépendants, API versionnée

**Approche de développement :**

TDD pur sur le domaine Rust : cycle rouge → vert → refacto. Chaque test encode un invariant métier. Le code de production découle des tests. La testabilité unitaire pure est une contrainte de conception : zéro dépendance infra dans les tests domaine.

**Complexité & Périmètre :**

- Domaine principal : full-stack web (Rust API + Next.js)
- Complexité : faible à moyenne (MVP délibérément réduit — < 100 makers, < 1 000 acheteurs)
- Composants architecturaux : 4 BCs Rust (Catalog, Identity, Order, Payment) + API layer + front Next.js + services externes

### Contraintes Techniques & Dépendances

- Architecture DDD par Bounded Context — `catalog-domain.md` est la référence de modélisation, pas du code existant
- Stripe : webhooks serveur obligatoires avant validation commande, aucun stockage de donnée carte
- Médias (photos, vidéos de fabrication) : hors base de données, stockage S3 + diffusion CDN
- SSE : domain events Rust streamés vers Next.js — WebSocket hors périmètre MVP (unidirectionnel suffisant)
- Next.js : ISR pour pages publiques SEO (makers, produits), SSR strict pour pages transactionnelles (panier, commande, dashboards)

### Préoccupations Transversales Identifiées

- **Auth/JWT** : 3 rôles (maker, acheteur, admin), tokens avec expiration, endpoints admin distincts et protégés
- **Domain Events inter-BCs** : Order consomme Catalog (reserve_stock/release_stock), Catalog émet vers Storefront
- **Email transactionnel** : confirmation commande, validation maker, relance panier — fiabilité ≥ 99% sur critiques
- **SSE** : canal temps réel unidirectionnel maker + acheteur depuis les domain events Rust
- **Médias** : upload + CDN, vidéos de fabrication avec sous-titres (WCAG)
- **Commission & Weekend Maker** : logique métier transversale (domaine Order/Payment, premier weekend de chaque mois)
- **RGPD** : droit à l'oubli cross-BC, collecte minimale, consentement explicite

## Structure Technique de Départ

### Domaine technologique

Full-stack — API Rust (axum) + Front-end Next.js, architecture Cargo workspace multi-crates.

### Décisions de stack

**Backend Rust :**
- Runtime : tokio 1.47.x (LTS)
- Framework HTTP : axum 0.8.x (écosystème Tokio, middleware Tower)
- SQL async : sqlx 0.8.x (compile-time query checking, PostgreSQL)
- Workspace Cargo édition 2024

**Frontend :**
- Next.js 16.x — App Router, TypeScript, Tailwind CSS
- SSR pour pages transactionnelles, ISR pour pages publiques SEO

**Commandes d'initialisation :**

```bash
# Workspace Rust — structure manuelle (pas de CLI dédié)
# Cargo.toml workspace à la racine + crates/ par BC

# Frontend Next.js
npx create-next-app@latest frontend \
  --typescript --tailwind --app --src-dir --import-alias "@/*"
```

### Structure Workspace

```
passion-market/                    ← Cargo workspace (root)
├── Cargo.toml                     ← [workspace] members = [...]
├── crates/
│   ├── shared-kernel/             ← types communs (Date, UUIDs base, traits)
│   │
│   ├── catalog-domain/            ← BC Catalog — domaine pur, TDD
│   ├── catalog-infra/             ← BC Catalog — SQLx + Postgres
│   ├── catalog-api/               ← BC Catalog — handlers axum
│   │
│   ├── identity-domain/           ← BC Identity — domaine pur, TDD
│   ├── identity-infra/
│   ├── identity-api/
│   │
│   ├── order-domain/              ← BC Order — domaine pur, TDD
│   ├── order-infra/
│   ├── order-api/
│   │
│   ├── payment-domain/            ← BC Payment — domaine pur, TDD
│   ├── payment-infra/
│   ├── payment-api/
│   │
│   └── app-server/                ← binaire — compose tous les BCs, point d'entrée axum
│
└── frontend/                      ← Next.js 16.x app
```

### Règle d'isolation domaine (TDD)

Les crates `*-domain` n'ont **aucune dépendance infrastructure** — pas de tokio, pas de sqlx, pas d'axum. Dépendances autorisées : `shared-kernel`, `thiserror`, `uuid`, `serde` (derive only). Les tests domaine sont synchrones et purs. Chaque test = un invariant métier.

## Décisions Architecturales Clés

### D1 — Isolation des données par BC

**Décision : Schémas PostgreSQL séparés par BC**

Un schema Postgres dédié par Bounded Context (`catalog`, `identity`, `order`, `payment`). Même instance PostgreSQL, isolation forte des données. Les jointures inter-BC sont interdites — la communication passe par les Domain Events, jamais par des jointures SQL.

### D2 — Domain Events inter-BCs

**Décision : RabbitMQ (Docker)**

Un message broker RabbitMQ conteneurisé (Docker Compose en dev, service managé en prod) assure la communication inter-BCs. Chaque BC publie ses events sur un exchange dédié (topic exchange). Les consumers sont les Application Services des autres BCs.

Les crates `*-domain` définissent un trait `EventPublisher` — l'infrastructure implémente ce trait. Le domaine ne connaît pas le broker.

### D3 — Authentification

**Décision : JWT en httpOnly cookies**

Access token + refresh token stockés en cookies httpOnly. Sécurité XSS native. Disponibles côté serveur Next.js pour le SSR. Middleware axum vérifie le cookie sur chaque requête protégée. Endpoints admin sur un préfixe distinct (`/admin`) avec vérification de rôle supplémentaire.

### D4 — Format d'erreur API

**Décision : RFC 7807 Problem Details**

Toutes les erreurs API suivent le standard HTTP Problem Details :
```json
{ "type": "...", "title": "...", "status": 422, "detail": "..." }
```
Cohérent sur toute l'API, facilite le débogage et la gestion côté Next.js.

### D5 — State management Next.js

**Décision : Context API uniquement**

État UI géré avec React Context natif (panier, session auth). Suffisant pour le MVP. Pas de dépendance externe côté état — simplifie le bundle et la maintenance.

### D7 — Application Services (Use Cases) et dispatch d'events

**Décision : Use cases dans `{bc}-infra`, consumers dans `{bc}-infra/consumers/`**

Les Application Services (use cases) vivent dans `{bc}-infra/src/use_cases/`. Ils orchestrent : charger l'agrégat → exécuter la logique domaine → persister → dispatcher les domain events via `EventPublisher`. Le dispatch se fait **après** la persistance, jamais avant.

Les handlers `{bc}-api` sont de purs adaptateurs HTTP thin : ils parsent la requête, appellent le use case injecté via `State<Arc<UseCase>>`, et retournent la réponse.

Les subscribers (consumers RabbitMQ) vivent dans `{bc}-infra/src/consumers/`. Ils sont enregistrés comme tâches background dans `app-server/src/messaging.rs` au démarrage.

```
handler (api) → use case (infra) → aggregate (domain) → repository (infra) → event publisher (infra)
consumer (infra) → use case (infra) → aggregate (domain) → ...
```

**Règle** : `{bc}-api` ne dépend jamais de `{bc}-infra`. Les use cases sont injectés via les traits définis dans `{bc}-domain/src/ports.rs`.

### D8 — Stockage médias (photos, vidéos)

**Décision : MinIO (dev) + Cloudflare R2 (prod) via `aws-sdk-s3`**

Le crate `aws-sdk-s3` est utilisé avec endpoint configurable — aucune modification de code entre les environnements. Seule la variable `S3_ENDPOINT` change.

| Env | Solution | Coût |
|-----|----------|------|
| Dev | MinIO (Docker Compose) | Gratuit, self-hosted |
| Prod | Cloudflare R2 | 10 Go gratuits, zéro frais d'egress |

Implémentation dans `catalog-infra/src/s3_media_storage.rs` via le trait `MediaStorage` défini dans `catalog-domain/src/ports.rs`.

### D9 — Observabilité

**Décision : tracing + Better Stack Logs + Grafana Cloud**

| Couche | Outil |
|--------|-------|
| Traces/logs Rust | `tracing` + `tracing-subscriber` (JSON prod, pretty dev) |
| Agrégation | Better Stack Logs (free tier, intégration Railway native) |
| Métriques | `metrics` crate + endpoint `/metrics` scrappable |
| Dashboard | Grafana Cloud (free tier) |

Initialisé dans `app-server/src/observability.rs` au démarrage.

**Lint Rust (workspace) :**
- `rustfmt.toml` : édition 2024, `imports_granularity = "Crate"`
- CI : `cargo fmt --all -- --check` + `cargo clippy --all-targets -- -D warnings`

### D6 — Déploiement MVP

**Décision : Railway**

Déploiement depuis GitHub, PostgreSQL managé inclus, zéro friction pour développeur solo. Environnements (dev/prod) gérés via variables Railway. Migration vers VPS si la scale le justifie post-MVP.

## Patterns d'Implémentation & Règles de Cohérence

### Points de conflit identifiés : 8 zones

### Naming — Base de données

| Zone | Convention | Exemple |
|------|-----------|---------|
| Tables | `snake_case` pluriel, préfixé du schema | `catalog.products`, `identity.users` |
| Colonnes | `snake_case` | `created_at`, `maker_id` |
| Clés étrangères | `{entité}_id` | `maker_id`, `order_id` |
| Index | `idx_{table}_{colonne(s)}` | `idx_products_maker_id` |
| Migrations | `{YYYYMMDDHHMMSS}_{description}.sql` | `20260328120000_create_products.sql` |

### Naming — API REST

| Zone | Convention | Exemple |
|------|-----------|---------|
| Préfixe | `/api/v1/` | `/api/v1/products` |
| Ressources | `kebab-case` pluriel | `/api/v1/maker-profiles` |
| Paramètres route | `:id` (axum) | `/api/v1/products/:id` |
| Query params | `snake_case` | `?maker_id=...&page=1` |
| Endpoints admin | `/admin/` préfixe séparé | `/admin/makers/:id/approve` |

### Naming — Code Rust

| Zone | Convention | Exemple |
|------|-----------|---------|
| Modules, fichiers | `snake_case` | `product_repository.rs` |
| Types, structs, enums | `PascalCase` | `ProductStatus`, `MakerProfile` |
| Fonctions, variables | `snake_case` | `find_by_maker_id()` |
| Constantes | `SCREAMING_SNAKE_CASE` | `MAX_STOCK_QUANTITY` |
| Traits domaine | nom métier + suffixe | `ProductRepository`, `EventPublisher` |

### Naming — Code TypeScript / Next.js

| Zone | Convention | Exemple |
|------|-----------|---------|
| Fichiers composants | `PascalCase.tsx` | `ProductCard.tsx` |
| Fichiers utilitaires | `camelCase.ts` | `formatPrice.ts` |
| Composants | `PascalCase` | `<MakerProfile />` |
| Hooks | `use` + PascalCase | `useCart()`, `useAuth()` |
| Variables/fonctions | `camelCase` | `makerId`, `fetchProducts()` |

### Format — Sérialisation JSON (Rust ↔ Next.js)

- Toutes les structs Rust exposées en API portent `#[serde(rename_all = "camelCase")]`
- Query params REST : `snake_case` (convention HTTP)
- Dates : ISO 8601 UTC — `"2026-03-28T12:00:00Z"`
- IDs : UUID v7 en string — `"019123..."`
- Montants : centimes en entier — `{ "amount": 2990, "currency": "EUR" }`

### Structure — Tests Rust

Tests unitaires purs → `#[cfg(test)]` inline dans le fichier source. Tests d'intégration domaine → `tests/` à la racine de la crate.

```
crates/catalog-domain/
├── src/
│   └── product.rs      ← #[cfg(test)] inline pour invariants unitaires
└── tests/
    └── product_test.rs  ← tests d'intégration domaine (pas de mocks infra)
```

### Communication — RabbitMQ Events

| Zone | Convention | Exemple |
|------|-----------|---------|
| Exchange | `{bc}.events` (topic) | `catalog.events`, `order.events` |
| Routing key | `{bc}.{aggregate}.{verbe_passé}` | `catalog.product.published`, `order.order.placed` |

Enveloppeur event standardisé (obligatoire) :

```json
{
  "event_type": "catalog.product.published",
  "aggregate_id": "019123-...",
  "occurred_at": "2026-03-28T12:00:00Z",
  "version": 1,
  "data": { "maker_id": "...", "title": "...", "stock": 3 }
}
```

### Process — Gestion d'erreurs Rust → HTTP

Chaîne : `DomainError` → `ApplicationError` → `ApiError` (impl `IntoResponse` axum → RFC 7807)

| DomainError | HTTP Status |
|-------------|-------------|
| `NotFound` | 404 |
| `ValidationError` | 422 |
| `Unauthorized` | 401 |
| `Forbidden` | 403 |
| `Conflict` | 409 |
| Autres | 500 |

### Règles obligatoires pour tous les agents

1. **Zéro jointure SQL inter-schémas** — communication via events RabbitMQ uniquement
2. **Crates `*-domain` : zéro dépendance tokio/sqlx/axum** — compilable et vérifiable
3. **`#[serde(rename_all = "camelCase")]`** obligatoire sur toutes les structs exposées en API
4. **UUID v7** pour tous les IDs métier
5. **Toutes les erreurs API → RFC 7807** via `ApiError`
6. **Enveloppeur event RabbitMQ standardisé** — jamais de payload nu

## Structure Projet & Frontières

### Arbre complet du projet

Voir fichier annexe : [`project-structure.md`](./project-structure.md)

L'arbre complet avec toutes les annotations est maintenu séparément pour éviter d'alourdir ce document.

### Frontières architecturales

**Frontières API :**
- API Rust écoute sur `:3001` — Next.js proxifie via `next.config.ts` rewrites
- Préfixe public : `/api/v1/*` — authentification optionnelle
- Préfixe protégé : `/api/v1/*` + cookie JWT valide
- Préfixe admin : `/admin/*` + JWT rôle `Admin`
- Webhooks : `/api/v1/webhooks/stripe` — vérification signature HMAC Stripe

**Frontières données :**
- Schémas PostgreSQL : `catalog`, `identity`, `order`, `payment` — aucune jointure inter-schéma
- Médias (photos, vidéos) : S3 uniquement, jamais en base — URLs CDN stockées dans `catalog`
- Sessions : cookies httpOnly côté client, aucun état serveur

**Flux de données — commande :**
```
Next.js (SSR) → POST /api/v1/carts/:id/checkout
  → order-api → order-domain (validation métier)
  → order-infra → payment-api → stripe_gateway
  → Stripe webhook → payment-api → PaymentConfirmed event → RabbitMQ
  → order-infra (consumer) → OrderConfirmed → RabbitMQ
  → catalog-infra (consumer) → StockReserved
  → SSE → Next.js notifications
```

### Mapping FR → structure

| FR | Composant |
|----|-----------|
| FR1–FR6 Identité | `identity-*` crates + `app/dashboard`, `app/admin` |
| FR7–FR10 Admin | `identity-api/handlers/admin_makers.rs` + `app/admin` |
| FR11–FR18 Boutique/Catalogue | `catalog-*` crates + `app/products`, `app/dashboard/products` |
| FR19–FR23 Découverte | `catalog-api` + `app/makers/[slug]`, `app/products/[id]` (ISR) |
| FR24–FR30 Achat/Paiement | `order-*` + `payment-*` + `app/cart`, `app/checkout` |
| FR31–FR33 Notifications | `app-server/messaging.rs` + `components/notifications/` |
| FR34–FR37 RGPD/Conformité | cross-BC — droit à l'oubli via `identity-api` |

## Résultats de Validation Architecture

### Cohérence des décisions ✅

- tokio 1.47 + axum 0.8 + sqlx 0.8 : même écosystème, versions compatibles
- RabbitMQ (topic exchange) + PostgreSQL (schémas isolés) : cohérent avec l'isolation BC
- JWT httpOnly + middleware axum + SSR Next.js : cookie accessible côté serveur pour le SSR
- RFC 7807 + `camelCase` JSON : cohérent, sans conflit
- ISR (pages publiques) + SSR (pages transactionnelles) + Railway : pas de conflit de déploiement
- Use cases dans `{bc}-infra` + handlers thin dans `{bc}-api` : isolation respectée

### Couverture des exigences ✅

| Catégorie | Couverture |
|---|---|
| FR1–FR6 Identité | `identity-*`, JWT, argon2 ✅ |
| FR7–FR10 Admin | `/admin/makers` + rôle JWT ✅ |
| FR11–FR18 Boutique/Catalogue | `catalog-*`, cycle Draft→Published→OutOfStock ✅ |
| FR19–FR23 Découverte/Favoris | `catalog-*`, ISR, `favorite.rs` dans domaine ✅ |
| FR24–FR30 Achat/Paiement | `order-*` + `payment-*` + Stripe webhook ✅ |
| FR31–FR33 Notifications | SSE dans `catalog-api/sse.rs`, `messaging.rs` ✅ |
| FR34–FR37 RGPD | cross-BC via `identity-api` ✅ |
| Performance LCP < 2.5s | ISR + CDN R2 ✅ |
| Sécurité PCI-DSS | Stripe délégué, zéro carte en base ✅ |
| Maintenabilité TDD ≥ 80% | structure domaine pur, tests par invariant ✅ |

### Checklist de complétude

**Analyse du contexte :**
- [x] Exigences fonctionnelles (37 FR) analysées
- [x] NFRs identifiées (performance, sécurité, WCAG, maintenabilité)
- [x] Contraintes techniques cartographiées
- [x] Préoccupations transversales documentées

**Décisions architecturales (D1–D9) :**
- [x] Stack technique avec versions vérifiées
- [x] Isolation données par schéma PostgreSQL
- [x] Communication inter-BC via RabbitMQ
- [x] Authentification JWT httpOnly
- [x] Format d'erreur RFC 7807
- [x] State management Next.js
- [x] Déploiement Railway
- [x] Application Services + dispatch events
- [x] Stockage médias MinIO/R2
- [x] Observabilité + lint

**Patterns d'implémentation :**
- [x] Conventions de nommage (DB, API, Rust, TypeScript)
- [x] Sérialisation JSON (camelCase, UUID v7, ISO 8601, centimes)
- [x] Organisation des tests Rust
- [x] Enveloppeur RabbitMQ standardisé
- [x] Chaîne d'erreurs Rust → HTTP

**Structure projet :**
- [x] Arbre complet dans `project-structure.md`
- [x] Frontières API définies
- [x] Flux de données documenté
- [x] Mapping FR → composants

### Statut : PRÊT POUR L'IMPLÉMENTATION

**Niveau de confiance : Élevé**

**Points forts :**
- Isolation domaine stricte — testabilité unitaire pure garantie
- Structure workspace scalable — chaque BC est indépendant
- Patterns anti-conflit exhaustifs pour les agents IA
- Décisions de stack pragmatiques (boring tech)

**Points à surveiller en implémentation :**
- WCAG sous-titres vidéos — traiter au niveau composant `VideoPlayer`
- Droit à l'oubli RGPD — implémenter en dernier, après que les schémas sont stables
- Commission Weekend Maker — invariant temporel complexe, couvrir avec des tests de dates limites
