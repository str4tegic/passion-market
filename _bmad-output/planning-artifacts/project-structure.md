---
type: 'reference'
project_name: 'passion-market'
date: '2026-03-28'
---

# Structure Projet — passion-market

_Arbre complet du workspace. Référence canonique pour tous les agents d'implémentation._

```
passion-market/                          ← Cargo workspace root
├── Cargo.toml                           ← [workspace] members + [workspace.dependencies]
├── Cargo.lock
├── rustfmt.toml                         ← édition 2024, imports groupés
├── .clippy.toml                         ← deny: unwrap_used, expect_used (hors tests)
├── .env.example
├── .gitignore
├── docker-compose.yml                   ← PostgreSQL + RabbitMQ + MinIO (dev)
├── docker-compose.test.yml              ← PostgreSQL test isolée
│
├── .github/
│   └── workflows/
│       ├── ci.yml                       ← fmt --check + clippy -D warnings + tests
│       └── deploy.yml                   ← Railway deploy on main
│
├── migrations/                          ← SQLx migrations globales (un schéma par BC)
│   ├── 20260328000001_create_catalog_schema.sql
│   ├── 20260328000002_create_identity_schema.sql
│   ├── 20260328000003_create_order_schema.sql
│   └── 20260328000004_create_payment_schema.sql
│
├── crates/
│   │
│   ├── shared-kernel/                   ← types communs cross-BC (zéro logique métier)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── ids.rs                   ← UuidV7, newtype IDs par BC
│   │       ├── money.rs                 ← Money(amount_cents: u64, currency: &'static str)
│   │       ├── pagination.rs            ← Page<T>, PageParams
│   │       └── events.rs                ← EventEnvelope<T>, trait DomainEvent
│   │
│   ├── catalog-domain/                  ← BC Catalog — domaine pur, zéro infra, TDD
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── product.rs               ← Product aggregate, ProductStatus (Draft→Published→OutOfStock)
│   │   │   ├── shop.rs                  ← Shop aggregate
│   │   │   ├── stock.rs                 ← StockLevel value object
│   │   │   ├── favorite.rs              ← Favorite (buyer_id + product_id), alerte dispo
│   │   │   ├── events.rs                ← ProductPublished, StockReserved, StockReleased…
│   │   │   └── ports.rs                 ← traits: ProductRepository, ShopRepository,
│   │   │                                   FavoriteRepository, EventPublisher, MediaStorage
│   │   └── tests/
│   │       ├── product_lifecycle_test.rs
│   │       ├── stock_management_test.rs
│   │       └── favorite_alert_test.rs
│   │
│   ├── catalog-infra/                   ← BC Catalog — SQLx + RabbitMQ + R2/MinIO
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── pg_product_repository.rs
│   │       ├── pg_shop_repository.rs
│   │       ├── pg_favorite_repository.rs
│   │       ├── s3_media_storage.rs      ← aws-sdk-s3, endpoint configurable (MinIO/R2)
│   │       ├── rabbitmq_event_publisher.rs
│   │       ├── use_cases/               ← Application Services (orchestrent domaine + infra)
│   │       │   ├── mod.rs
│   │       │   ├── publish_product.rs   ← load → domain.publish() → save → dispatch events
│   │       │   ├── create_shop.rs
│   │       │   ├── update_stock.rs
│   │       │   └── toggle_favorite.rs
│   │       └── consumers/               ← Subscribers RabbitMQ (appelés par app-server)
│   │           ├── mod.rs
│   │           └── order_stock_consumer.rs  ← order.order.placed → reserve_stock
│   │
│   ├── catalog-api/                     ← BC Catalog — handlers axum (thin HTTP adapters)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── router.rs                ← /api/v1/products, /api/v1/shops, /api/v1/favorites
│   │       ├── sse.rs                   ← GET /api/v1/events — SSE stream (domain events → client)
│   │       ├── handlers/
│   │       │   ├── products.rs
│   │       │   ├── shops.rs
│   │       │   └── favorites.rs
│   │       └── dto/
│   │           ├── product_dto.rs       ← #[serde(rename_all = "camelCase")]
│   │           ├── shop_dto.rs
│   │           └── favorite_dto.rs
│   │
│   ├── identity-domain/                 ← BC Identity — domaine pur, TDD
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── user.rs                  ← User aggregate, Role(Maker/Buyer/Admin)
│   │   │   ├── maker_profile.rs         ← MakerProfile (storytelling, photo atelier, vidéo)
│   │   │   ├── credentials.rs           ← PasswordHash (argon2)
│   │   │   ├── events.rs                ← MakerApproved, MakerRejected, MakerRegistered…
│   │   │   └── ports.rs                 ← UserRepository, MakerRepository, EventPublisher
│   │   └── tests/
│   │       ├── user_registration_test.rs
│   │       └── maker_approval_test.rs
│   │
│   ├── identity-infra/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── pg_user_repository.rs
│   │       ├── pg_maker_repository.rs
│   │       ├── argon2_hasher.rs
│   │       ├── jwt_service.rs           ← access token (15min) + refresh token (7j)
│   │       ├── rabbitmq_event_publisher.rs
│   │       ├── use_cases/
│   │       │   ├── mod.rs
│   │       │   ├── register_user.rs
│   │       │   ├── login.rs
│   │       │   ├── register_maker.rs
│   │       │   └── approve_maker.rs     ← dispatch MakerApproved → email transactionnel
│   │       └── consumers/
│   │           └── mod.rs               ← (vide MVP — identity ne consomme pas d'events)
│   │
│   ├── identity-api/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── router.rs                ← /api/v1/auth, /api/v1/makers, /admin/makers
│   │       ├── middleware/
│   │       │   └── auth.rs              ← extrait JWT du cookie httpOnly, injecte Claims
│   │       ├── handlers/
│   │       │   ├── auth.rs              ← POST /login, /logout, /refresh
│   │       │   ├── makers.rs            ← GET /makers/:slug, PATCH /makers/me
│   │       │   └── admin_makers.rs      ← GET /admin/makers, POST /admin/makers/:id/approve
│   │       └── dto/
│   │           ├── auth_dto.rs
│   │           └── maker_dto.rs
│   │
│   ├── order-domain/                    ← BC Order — domaine pur, TDD
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── cart.rs                  ← Cart aggregate, CartItem
│   │   │   ├── order.rs                 ← Order aggregate, OrderStatus
│   │   │   ├── commission.rs            ← CommissionRate, mécanique Weekend Maker
│   │   │   ├── events.rs                ← OrderPlaced, OrderConfirmed, CartAbandoned…
│   │   │   └── ports.rs                 ← CartRepository, OrderRepository, StockService
│   │   └── tests/
│   │       ├── cart_test.rs
│   │       ├── order_placement_test.rs
│   │       └── commission_test.rs       ← invariants Weekend Maker (premier weekend du mois)
│   │
│   ├── order-infra/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── pg_cart_repository.rs
│   │       ├── pg_order_repository.rs
│   │       ├── rabbitmq_event_publisher.rs
│   │       ├── use_cases/
│   │       │   ├── mod.rs
│   │       │   ├── add_to_cart.rs
│   │       │   ├── checkout.rs          ← valide stock → crée order → dispatch OrderPlaced
│   │       │   └── abandon_cart.rs      ← scheduler → dispatch CartAbandoned
│   │       └── consumers/
│   │           ├── mod.rs
│   │           └── payment_confirmed_consumer.rs  ← payment.payment.confirmed → confirm order
│   │
│   ├── order-api/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── router.rs                ← /api/v1/carts, /api/v1/orders
│   │       ├── handlers/
│   │       │   ├── carts.rs
│   │       │   └── orders.rs
│   │       └── dto/
│   │           ├── cart_dto.rs
│   │           └── order_dto.rs
│   │
│   ├── payment-domain/                  ← BC Payment — domaine pur, TDD
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── payment.rs               ← Payment aggregate
│   │   │   ├── webhook.rs               ← StripeWebhookEvent parsing + vérification signature
│   │   │   ├── events.rs                ← PaymentConfirmed, PaymentFailed…
│   │   │   └── ports.rs                 ← PaymentGateway, EventPublisher
│   │   └── tests/
│   │       ├── payment_test.rs
│   │       └── webhook_test.rs
│   │
│   ├── payment-infra/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── stripe_gateway.rs        ← impl PaymentGateway
│   │       ├── rabbitmq_event_publisher.rs
│   │       ├── use_cases/
│   │       │   ├── mod.rs
│   │       │   ├── initiate_payment.rs  ← crée PaymentIntent Stripe → dispatch events
│   │       │   └── handle_webhook.rs    ← vérifie signature → dispatch PaymentConfirmed/Failed
│   │       └── consumers/
│   │           └── mod.rs               ← (vide MVP — payment ne consomme pas d'events)
│   │
│   ├── payment-api/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── router.rs                ← /api/v1/payments, /api/v1/webhooks/stripe
│   │       ├── handlers/
│   │       │   ├── payments.rs
│   │       │   └── stripe_webhook.rs    ← vérifie Stripe-Signature header avant tout
│   │       └── dto/
│   │           └── payment_dto.rs
│   │
│   └── app-server/                      ← binaire — compose tous les BCs
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs                  ← tokio::main — init tracing, config, db, messaging
│           ├── app.rs                   ← Router axum — merge routers BC + middleware global
│           ├── config.rs                ← AppConfig (DATABASE_URL, RABBITMQ_URL, R2_*, JWT_*)
│           ├── db.rs                    ← PgPool, sqlx::migrate!() au démarrage
│           ├── messaging.rs             ← connexion RabbitMQ, enregistre tous les consumers
│           ├── mailer.rs                ← service email transactionnel (Resend API)
│           └── observability.rs         ← init tracing-subscriber (JSON prod / pretty dev)
│
└── frontend/                            ← Next.js 16.x
    ├── package.json
    ├── next.config.ts                   ← rewrites: /api/* → http://localhost:3001
    ├── tailwind.config.ts
    ├── tsconfig.json
    ├── .env.local
    ├── .env.example
    └── src/
        ├── app/                         ← App Router Next.js
        │   ├── layout.tsx
        │   ├── page.tsx                 ← Home — catalogue (ISR, revalidate: 60s)
        │   ├── makers/
        │   │   └── [slug]/page.tsx      ← Profil maker public (ISR)
        │   ├── products/
        │   │   └── [id]/page.tsx        ← Fiche produit (ISR)
        │   ├── cart/page.tsx            ← Panier (SSR — données utilisateur)
        │   ├── checkout/page.tsx        ← Tunnel achat (SSR)
        │   ├── orders/page.tsx          ← Mes commandes (SSR)
        │   ├── dashboard/               ← Dashboard maker (SSR)
        │   │   ├── page.tsx             ← Vue d'ensemble commandes + stats
        │   │   ├── products/page.tsx    ← Gestion catalogue maker
        │   │   └── orders/page.tsx      ← Commandes reçues
        │   ├── admin/                   ← Admin (SSR, rôle Admin requis)
        │   │   └── makers/page.tsx      ← File d'approbation makers
        │   └── api/                     ← Route handlers Next.js si besoin de BFF
        ├── components/
        │   ├── ui/                      ← Button, Input, Badge, Modal (génériques)
        │   ├── catalog/                 ← ProductCard, MakerCard, CatalogGrid
        │   ├── cart/                    ← CartDrawer, CartItem, CartSummary
        │   ├── checkout/                ← CheckoutForm, OrderSummary, StripeElement
        │   ├── maker/                   ← MakerProfile, ShopHeader, VideoPlayer
        │   ├── notifications/           ← SseListener, NotificationToast (temps réel)
        │   └── admin/                   ← MakerApprovalQueue, MakerApprovalCard
        ├── contexts/
        │   ├── CartContext.tsx          ← panier persistant (localStorage + API sync)
        │   └── AuthContext.tsx          ← session utilisateur, rôle
        ├── lib/
        │   ├── api.ts                   ← fetch wrapper avec gestion RFC 7807
        │   ├── auth.ts                  ← helpers cookie JWT côté serveur Next.js
        │   └── format.ts                ← formatPrice(cents), formatDate(iso)
        └── types/
            └── api.ts                   ← types TS générés/alignés sur DTOs Rust
```

---

## Notes de référence

### Pattern Use Case (Application Service) dans `{bc}-infra`

```rust
// catalog-infra/src/use_cases/publish_product.rs
pub struct PublishProductUseCase {
    product_repo: Arc<dyn ProductRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl PublishProductUseCase {
    pub async fn execute(&self, product_id: ProductId) -> Result<(), ApplicationError> {
        let mut product = self.product_repo.find_by_id(product_id).await?;
        let event = product.publish()?;           // logique domaine pure
        self.product_repo.save(&product).await?;
        self.event_publisher.publish(event).await?; // dispatch après persistance
        Ok(())
    }
}
```

Le handler `catalog-api` est thin :
```rust
// catalog-api/src/handlers/products.rs
async fn publish_product(
    State(use_case): State<Arc<PublishProductUseCase>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    use_case.execute(ProductId(id)).await.into_response()
}
```

### Stockage médias — Stratégie dual-env

| Env | Solution | Config |
|-----|----------|--------|
| Dev | MinIO (Docker Compose) | `S3_ENDPOINT=http://localhost:9000` |
| Prod | Cloudflare R2 | `S3_ENDPOINT=https://{account}.r2.cloudflarestorage.com` |

Le crate `aws-sdk-s3` fonctionne sans modification — seul `S3_ENDPOINT` change.

### Observabilité

| Couche | Outil | Notes |
|--------|-------|-------|
| Traces/logs Rust | `tracing` + `tracing-subscriber` | JSON en prod, pretty en dev |
| Agrégation logs | Better Stack Logs (gratuit 1GB/j) | intégration Railway native |
| Métriques | `metrics` crate + `/metrics` endpoint | Prometheus scrape |
| Dashboard | Grafana Cloud (free tier) | scrape Railway metrics |

### Lint Rust (workspace)

`rustfmt.toml` :
```toml
edition = "2024"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

`Cargo.toml` workspace :
```toml
[workspace.metadata.clippy]
# Enforced in CI: cargo clippy -- -D warnings
```

CI enforce :
```yaml
- run: cargo fmt --all -- --check
- run: cargo clippy --all-targets --all-features -- -D warnings
```
