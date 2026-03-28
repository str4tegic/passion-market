# Story 1.1 — Workspace Cargo & Environnement Docker Compose

## Métadonnées

| Champ | Valeur |
|-------|--------|
| **Story ID** | 1.1 |
| **Story Key** | `1-1-workspace-cargo-et-environnement-docker-compose` |
| **Epic** | Epic 1 — Socle Projet & Environnement |
| **Statut** | review |
| **Date de création** | 2026-03-28 |

---

## User Story

**En tant que** développeur,
**Je veux** un workspace Cargo configuré avec toutes les crates et un Docker Compose fonctionnel,
**Afin de** pouvoir développer et tester tous les BCs localement sans friction.

---

## Critères d'Acceptation (BDD)

**CA-1 — Compilation workspace**
- **Given** le répertoire racine du projet,
- **When** `cargo build --workspace` est exécuté,
- **Then** toutes les crates compilent sans erreur : `shared-kernel`, `catalog-domain`, `catalog-infra`, `catalog-api`, `identity-domain`, `identity-infra`, `identity-api`, `order-domain`, `order-infra`, `order-api`, `payment-domain`, `payment-infra`, `payment-api`, `app-server`
- **And** `cargo clippy --all-targets -- -D warnings` ne produit aucun avertissement
- **And** `cargo fmt --all -- --check` passe sans modification

**CA-2 — Services Docker Compose**
- **Given** le répertoire racine,
- **When** `docker compose up -d` est exécuté,
- **Then** PostgreSQL est accessible sur le port 5432
- **And** RabbitMQ est accessible sur le port 5672, UI management sur 15672
- **And** MinIO est accessible sur le port 9000, console sur 9001

**CA-3 — Migrations automatiques**
- **Given** `app-server` démarré avec `DATABASE_URL` configuré,
- **When** le binaire s'exécute,
- **Then** les migrations SQLx dans `/migrations/` sont appliquées automatiquement
- **And** les 4 schémas PostgreSQL (`catalog`, `identity`, `order`, `payment`) existent

**CA-4 — Isolation domaine vérifiable**
- **Given** les crates `{bc}-domain`,
- **When** `cargo test -p catalog-domain` (ou tout autre bc-domain) est exécuté,
- **Then** les tests passent sans dépendance tokio/sqlx/axum dans le graph de compilation

---

## Contexte Développeur

### Périmètre de cette story

Cette story est **greenfield pur** : le dépôt est vide. Il faut créer la structure complète du workspace Cargo, la configuration Docker Compose, les fichiers de lint, les migrations initiales et vérifier que tout compile. Aucune logique métier n'est implémentée ici — uniquement le squelette avec des `src/lib.rs` vides (ou quasi-vides) par crate.

### Ce que cette story NE fait PAS

- **Pas d'implémentation logique métier** dans les crates domaine (elles auront juste `pub mod` vides)
- **Pas de Next.js** (Story 1.2)
- **Pas de CI GitHub Actions** (Story 1.3)
- **Pas de handlers HTTP** ni de migrations de tables (uniquement les schémas)

---

## Exigences Techniques

### Stack & Versions

| Technologie | Version | Notes |
|-------------|---------|-------|
| Rust | édition 2024 | `cargo update` pour les dernières patchs |
| tokio | 1.47.x | LTS — runtime async |
| axum | 0.8.x | framework HTTP (écosystème Tower) |
| sqlx | 0.8.x | async SQL + compile-time checks |
| serde | 1.x | derive only dans les crates domaine |
| uuid | 1.x | feature `v7` + `serde` |
| thiserror | 2.x | DomainError dans les crates domaine |

### Règle fondamentale : isolation des crates domaine

**Les crates `*-domain` n'ont AUCUNE dépendance infra.** Dépendances autorisées uniquement :
- `shared-kernel` (workspace dep)
- `thiserror`
- `uuid` (features = ["v7", "serde"])
- `serde` (features = ["derive"])

**Interdit dans `*-domain/Cargo.toml`** : `tokio`, `sqlx`, `axum`, `lapin`, `aws-sdk-s3`.

Cette contrainte est vérifiable à la compilation et doit être respectée à la lettre.

---

## Structure de Fichiers à Créer

Créer exactement cette structure (les fichiers marqués `[vide]` ont juste le contenu minimal) :

```
passion-market/
├── Cargo.toml                          ← workspace root [voir ci-dessous]
├── Cargo.lock                          ← généré automatiquement
├── rustfmt.toml                        ← [voir ci-dessous]
├── .clippy.toml                        ← [voir ci-dessous]
├── .env.example                        ← [voir ci-dessous]
├── .gitignore                          ← [voir ci-dessous]
├── docker-compose.yml                  ← [voir ci-dessous]
├── docker-compose.test.yml             ← PostgreSQL test isolée
│
├── migrations/
│   ├── 20260328000001_create_catalog_schema.sql
│   ├── 20260328000002_create_identity_schema.sql
│   ├── 20260328000003_create_order_schema.sql
│   └── 20260328000004_create_payment_schema.sql
│
└── crates/
    ├── shared-kernel/
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── ids.rs
    │       ├── money.rs
    │       ├── pagination.rs
    │       └── events.rs
    │
    ├── catalog-domain/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── catalog-infra/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── catalog-api/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    │
    ├── identity-domain/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── identity-infra/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── identity-api/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    │
    ├── order-domain/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── order-infra/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── order-api/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    │
    ├── payment-domain/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── payment-infra/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    ├── payment-api/
    │   ├── Cargo.toml
    │   └── src/lib.rs
    │
    └── app-server/
        ├── Cargo.toml
        └── src/
            ├── main.rs
            ├── config.rs
            └── db.rs
```

---

## Contenu des Fichiers Clés

### `Cargo.toml` (workspace root)

```toml
[workspace]
members = [
    "crates/shared-kernel",
    "crates/catalog-domain",
    "crates/catalog-infra",
    "crates/catalog-api",
    "crates/identity-domain",
    "crates/identity-infra",
    "crates/identity-api",
    "crates/order-domain",
    "crates/order-infra",
    "crates/order-api",
    "crates/payment-domain",
    "crates/payment-infra",
    "crates/payment-api",
    "crates/app-server",
]
resolver = "2"

[workspace.package]
edition = "2024"
version = "0.1.0"
authors = ["Francois"]

[workspace.dependencies]
# Async runtime
tokio = { version = "1.47", features = ["full"] }

# HTTP
axum = { version = "0.8", features = ["macros"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "trace"] }

# Database
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono", "migrate"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# IDs
uuid = { version = "1", features = ["v7", "serde"] }

# Errors
thiserror = "2"
anyhow = "1"

# Tracing
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Messaging (RabbitMQ)
lapin = "2"

# S3 / MinIO
aws-sdk-s3 = "1"
aws-config = "1"

# Config
dotenvy = "0.15"

# Internal crates
shared-kernel = { path = "crates/shared-kernel" }
catalog-domain = { path = "crates/catalog-domain" }
catalog-infra = { path = "crates/catalog-infra" }
catalog-api = { path = "crates/catalog-api" }
identity-domain = { path = "crates/identity-domain" }
identity-infra = { path = "crates/identity-infra" }
identity-api = { path = "crates/identity-api" }
order-domain = { path = "crates/order-domain" }
order-infra = { path = "crates/order-infra" }
order-api = { path = "crates/order-api" }
payment-domain = { path = "crates/payment-domain" }
payment-infra = { path = "crates/payment-infra" }
payment-api = { path = "crates/payment-api" }
```

### `crates/shared-kernel/Cargo.toml`

```toml
[package]
name = "shared-kernel"
version.workspace = true
edition.workspace = true

[dependencies]
serde = { workspace = true }
uuid = { workspace = true }
thiserror = { workspace = true }
```

### `crates/{bc}-domain/Cargo.toml` (même patron pour les 4 BCs)

```toml
# Exemple pour catalog-domain — reproduire pour identity-domain, order-domain, payment-domain
[package]
name = "catalog-domain"
version.workspace = true
edition.workspace = true

[dependencies]
shared-kernel = { workspace = true }
serde = { workspace = true }
uuid = { workspace = true }
thiserror = { workspace = true }

# INTERDIT : tokio, sqlx, axum, lapin, aws-sdk-s3
```

### `crates/{bc}-infra/Cargo.toml` (même patron pour les 4 BCs)

```toml
# Exemple pour catalog-infra
[package]
name = "catalog-infra"
version.workspace = true
edition.workspace = true

[dependencies]
catalog-domain = { workspace = true }
shared-kernel = { workspace = true }
tokio = { workspace = true }
sqlx = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
uuid = { workspace = true }
thiserror = { workspace = true }
anyhow = { workspace = true }
tracing = { workspace = true }
lapin = { workspace = true }
```

### `crates/{bc}-api/Cargo.toml` (même patron pour les 4 BCs)

```toml
# Exemple pour catalog-api
[package]
name = "catalog-api"
version.workspace = true
edition.workspace = true

[dependencies]
catalog-domain = { workspace = true }
shared-kernel = { workspace = true }
axum = { workspace = true }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
uuid = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }

# NE PAS dépendre de catalog-infra ici — injection via traits domaine
```

### `crates/app-server/Cargo.toml`

```toml
[package]
name = "app-server"
version.workspace = true
edition.workspace = true

[[bin]]
name = "app-server"
path = "src/main.rs"

[dependencies]
shared-kernel = { workspace = true }
catalog-domain = { workspace = true }
catalog-infra = { workspace = true }
catalog-api = { workspace = true }
identity-domain = { workspace = true }
identity-infra = { workspace = true }
identity-api = { workspace = true }
order-domain = { workspace = true }
order-infra = { workspace = true }
order-api = { workspace = true }
payment-domain = { workspace = true }
payment-infra = { workspace = true }
payment-api = { workspace = true }
tokio = { workspace = true }
axum = { workspace = true }
sqlx = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
dotenvy = { workspace = true }
anyhow = { workspace = true }
```

### `crates/app-server/src/main.rs`

```rust
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    // TODO Story 1.1 : init tracing minimal pour vérifier le démarrage
    tracing_subscriber::fmt().with_env_filter("info").init();

    tracing::info!("passion-market app-server starting");

    // TODO Story 1.1 : connexion DB + migrations
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = sqlx::PgPool::connect(&database_url).await?;
    sqlx::migrate!("../../migrations").run(&pool).await?;

    tracing::info!("migrations applied successfully");

    // TODO Story 2+ : démarrer le serveur axum
    // Pour l'instant on vérifie juste que tout compile et les migrations passent

    Ok(())
}
```

**Note importante :** Le chemin `../../migrations` dans `sqlx::migrate!()` est relatif au fichier source. Depuis `crates/app-server/src/main.rs`, le chemin vers `/migrations/` à la racine du workspace est `../../migrations`. Ajuster si la structure diffère.

Alternative plus robuste : utiliser `SQLX_OFFLINE=true` + `sqlx migrate run` CLI séparément.

### `crates/app-server/src/config.rs`

```rust
#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub rabbitmq_url: String,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            rabbitmq_url: std::env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string()),
            s3_endpoint: std::env::var("S3_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),
            s3_bucket: std::env::var("S3_BUCKET")
                .unwrap_or_else(|_| "passion-market".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-in-prod".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()?,
        })
    }
}
```

### `crates/shared-kernel/src/lib.rs`

```rust
pub mod events;
pub mod ids;
pub mod money;
pub mod pagination;
```

### `crates/shared-kernel/src/ids.rs`

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Génère un nouvel UUID v7
pub fn new_id() -> Uuid {
    Uuid::now_v7()
}

// Newtype IDs par BC — à utiliser dans les crates domaine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CatalogId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IdentityId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrderId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaymentId(pub Uuid);
```

### `crates/shared-kernel/src/money.rs`

```rust
use serde::{Deserialize, Serialize};

/// Montant en centimes — jamais de float pour les montants financiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount_cents: u64,
    pub currency: &'static str,
}

impl Money {
    pub fn eur(amount_cents: u64) -> Self {
        Self { amount_cents, currency: "EUR" }
    }
}
```

### `crates/shared-kernel/src/events.rs`

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Enveloppeur standard pour tous les domain events publiés sur RabbitMQ
/// Format JSON : { "event_type": "...", "aggregate_id": "...", "occurred_at": "...", "version": 1, "data": {...} }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T: Serialize> {
    pub event_type: String,
    pub aggregate_id: Uuid,
    pub occurred_at: String, // ISO 8601 UTC
    pub version: u32,
    pub data: T,
}

impl<T: Serialize> EventEnvelope<T> {
    pub fn new(event_type: impl Into<String>, aggregate_id: Uuid, data: T) -> Self {
        Self {
            event_type: event_type.into(),
            aggregate_id,
            occurred_at: chrono_now_utc(),
            version: 1,
            data,
        }
    }
}

fn chrono_now_utc() -> String {
    // Stub pour Story 1.1 — sera remplacé par chrono::Utc::now() quand chrono est ajouté
    "2026-01-01T00:00:00Z".to_string()
}
```

**Note :** Ajouter `chrono = { version = "0.4", features = ["serde"] }` au workspace dependencies pour `EventEnvelope`. Sinon garder le stub pour cette story.

### `crates/shared-kernel/src/pagination.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageParams {
    pub page: u32,
    pub per_page: u32,
}

impl Default for PageParams {
    fn default() -> Self {
        Self { page: 1, per_page: 20 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}
```

### Crates domaine — `src/lib.rs` minimal

Chaque crate `*-domain` a un `src/lib.rs` vide ou presque :

```rust
// catalog-domain/src/lib.rs
// Squelette Story 1.1 — implémentation Story 4.1
```

Idem pour `identity-domain`, `order-domain`, `payment-domain`.

Les crates `*-infra` et `*-api` ont aussi des `src/lib.rs` vides pour l'instant.

---

### `docker-compose.yml`

```yaml
version: "3.9"

services:
  postgres:
    image: postgres:16-alpine
    ports:
      - "5432:5432"
    environment:
      POSTGRES_USER: passion
      POSTGRES_PASSWORD: passion
      POSTGRES_DB: passion_market
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U passion"]
      interval: 5s
      timeout: 5s
      retries: 5

  rabbitmq:
    image: rabbitmq:3.13-management-alpine
    ports:
      - "5672:5672"
      - "15672:15672"
    environment:
      RABBITMQ_DEFAULT_USER: passion
      RABBITMQ_DEFAULT_PASS: passion
    healthcheck:
      test: ["CMD", "rabbitmq-diagnostics", "ping"]
      interval: 10s
      timeout: 10s
      retries: 5

  minio:
    image: minio/minio:latest
    ports:
      - "9000:9000"
      - "9001:9001"
    environment:
      MINIO_ROOT_USER: minioadmin
      MINIO_ROOT_PASSWORD: minioadmin
    command: server /data --console-address ":9001"
    volumes:
      - minio_data:/data

volumes:
  postgres_data:
  minio_data:
```

### `docker-compose.test.yml`

```yaml
version: "3.9"

services:
  postgres-test:
    image: postgres:16-alpine
    ports:
      - "5433:5432"
    environment:
      POSTGRES_USER: passion
      POSTGRES_PASSWORD: passion
      POSTGRES_DB: passion_market_test
    tmpfs:
      - /var/lib/postgresql/data
```

---

### Migrations SQLx

**`migrations/20260328000001_create_catalog_schema.sql`**
```sql
CREATE SCHEMA IF NOT EXISTS catalog;
```

**`migrations/20260328000002_create_identity_schema.sql`**
```sql
CREATE SCHEMA IF NOT EXISTS identity;
```

**`migrations/20260328000003_create_order_schema.sql`**
```sql
CREATE SCHEMA IF NOT EXISTS "order";
```

**`migrations/20260328000004_create_payment_schema.sql`**
```sql
CREATE SCHEMA IF NOT EXISTS payment;
```

**Note :** `order` est un mot réservé PostgreSQL — utiliser des guillemets `"order"` dans le SQL.

---

### `rustfmt.toml`

```toml
edition = "2024"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

### `.clippy.toml`

```toml
# Désactivé pour les tests — ces lints ne s'appliquent qu'au code de production
# Dans CI : cargo clippy --all-targets -- -D warnings
```

**Note :** `unwrap_used` et `expect_used` sont fréquemment utilisés dans les tests — les configurer comme deny uniquement en dehors des modules de test si nécessaire, mais pour la Story 1.1, clippy de base (`-D warnings`) suffit.

### `.env.example`

```bash
# Base de données
DATABASE_URL=postgres://passion:passion@localhost:5432/passion_market

# RabbitMQ
RABBITMQ_URL=amqp://passion:passion@localhost:5672

# MinIO / S3
S3_ENDPOINT=http://localhost:9000
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin
S3_BUCKET=passion-market

# JWT
JWT_SECRET=dev-secret-change-in-prod-minimum-32-chars
JWT_ACCESS_TTL_SECONDS=900
JWT_REFRESH_TTL_SECONDS=604800

# Serveur
PORT=3001
RUST_LOG=info
```

### `.gitignore`

```gitignore
/target
.env
.env.local
*.env
!.env.example
*.pem
```

---

## Ordre d'Implémentation Recommandé

1. **Créer `Cargo.toml` workspace root** avec tous les membres
2. **Créer tous les `crates/*/Cargo.toml`** avec les dépendances correctes (respecter l'isolation domaine)
3. **Créer tous les `src/lib.rs`** vides (ou quasi-vides)
4. **Créer `shared-kernel/src/*.rs`** avec le contenu ci-dessus
5. **Créer `app-server/src/main.rs`**, `config.rs`, `db.rs`
6. **Vérifier** `cargo build --workspace` → corriger les erreurs
7. **Créer `rustfmt.toml`** et `docker-compose.yml`
8. **Créer `migrations/`** avec les 4 fichiers SQL
9. **Créer `.env`** depuis `.env.example`, lancer `docker compose up -d`
10. **Vérifier** `cargo clippy --all-targets -- -D warnings`
11. **Vérifier** `cargo fmt --all -- --check`
12. **Tester les migrations** : lancer `app-server`, vérifier les schémas dans PostgreSQL

---

## Commandes de Vérification

```bash
# Compilation workspace complète
cargo build --workspace

# Lint
cargo clippy --all-targets -- -D warnings

# Format check
cargo fmt --all -- --check

# Tests (vides pour l'instant)
cargo test --workspace

# Démarrer les services
docker compose up -d

# Vérifier PostgreSQL
psql postgres://passion:passion@localhost:5432/passion_market -c "\dn"
# Doit afficher : catalog, identity, order, payment

# Vérifier isolation domaine (aucune dépendance infra)
cargo tree -p catalog-domain
# Ne doit PAS contenir : tokio, sqlx, axum, lapin
```

---

## Points d'Attention (Pièges à Éviter)

1. **`order` est un mot réservé SQL** → utiliser `"order"` avec guillemets dans les migrations PostgreSQL
2. **Chemin `sqlx::migrate!()`** → relatif au fichier source Rust, pas au workspace root ; vérifier avec `cargo build` que le chemin est correct
3. **`resolver = "2"`** dans le workspace Cargo.toml → obligatoire avec l'édition 2024
4. **`version.workspace = true`** dans les crates → évite les duplications de version
5. **Ne pas ajouter `chrono`** aux workspace deps si les crates domaine n'en ont pas besoin — rester minimaliste pour Story 1.1
6. **`SQLX_OFFLINE`** : si sqlx vérifie les requêtes à la compilation, il faut une DB accessible ou `SQLX_OFFLINE=true`. Pour Story 1.1, aucune requête sqlx n'est écrite dans les crates — seulement dans `app-server` pour les migrations.

---

## Définition de "Done"

- [x] `cargo build --workspace` → vert
- [x] `cargo clippy --all-targets -- -D warnings` → vert (zéro warning)
- [x] `cargo fmt --all -- --check` → vert (aucune modification)
- [x] `docker compose up -d` → PostgreSQL + RabbitMQ + MinIO accessibles
- [x] `cargo run -p app-server` → migrations appliquées, 4 schémas créés dans PostgreSQL
- [x] `cargo tree -p catalog-domain` → ne contient pas tokio/sqlx/axum
- [x] Fichiers `.env.example`, `rustfmt.toml`, `.gitignore` présents

---

## Dev Agent Record

### Completion Notes

Implémentation complétée le 2026-03-28 :

- Workspace Cargo transformé depuis une crate unique en workspace 14 membres (`shared-kernel` + 4×3 BCs + `app-server`)
- Toutes les crates compilent : `cargo build --workspace` ✅
- Clippy zéro warning : `cargo clippy --all-targets -- -D warnings` ✅
- Format check passe : `cargo fmt --all -- --check` ✅
- Docker Compose : PostgreSQL 16, RabbitMQ 3.13-management, MinIO — 3 services up ✅
- Migrations SQLx : 4 schémas créés automatiquement au démarrage (`catalog`, `identity`, `order`, `payment`) ✅
- Isolation domaine vérifiée : aucune dépendance tokio/sqlx/axum dans les 4 crates `*-domain` ✅

**Ajustement notable :** `rustfmt.toml` simplifié — les options `imports_granularity` et `group_imports` sont nightly-only sur la toolchain stable utilisée (stable-x86_64-unknown-linux-gnu). Conservé uniquement `edition = "2024"`.

**Ajustement notable :** `#[allow(dead_code)]` sur `AppConfig` pour les champs qui seront utilisés dans les stories 2+.

---

## File List

### Nouveaux fichiers créés

- `Cargo.toml` (modifié — converti en workspace root)
- `rustfmt.toml`
- `.clippy.toml`
- `.env.example`
- `.gitignore` (modifié)
- `docker-compose.yml`
- `docker-compose.test.yml`
- `migrations/20260328000001_create_catalog_schema.sql`
- `migrations/20260328000002_create_identity_schema.sql`
- `migrations/20260328000003_create_order_schema.sql`
- `migrations/20260328000004_create_payment_schema.sql`
- `crates/shared-kernel/Cargo.toml`
- `crates/shared-kernel/src/lib.rs`
- `crates/shared-kernel/src/ids.rs`
- `crates/shared-kernel/src/money.rs`
- `crates/shared-kernel/src/events.rs`
- `crates/shared-kernel/src/pagination.rs`
- `crates/catalog-domain/Cargo.toml`
- `crates/catalog-domain/src/lib.rs`
- `crates/identity-domain/Cargo.toml`
- `crates/identity-domain/src/lib.rs`
- `crates/order-domain/Cargo.toml`
- `crates/order-domain/src/lib.rs`
- `crates/payment-domain/Cargo.toml`
- `crates/payment-domain/src/lib.rs`
- `crates/catalog-infra/Cargo.toml`
- `crates/catalog-infra/src/lib.rs`
- `crates/identity-infra/Cargo.toml`
- `crates/identity-infra/src/lib.rs`
- `crates/order-infra/Cargo.toml`
- `crates/order-infra/src/lib.rs`
- `crates/payment-infra/Cargo.toml`
- `crates/payment-infra/src/lib.rs`
- `crates/catalog-api/Cargo.toml`
- `crates/catalog-api/src/lib.rs`
- `crates/identity-api/Cargo.toml`
- `crates/identity-api/src/lib.rs`
- `crates/order-api/Cargo.toml`
- `crates/order-api/src/lib.rs`
- `crates/payment-api/Cargo.toml`
- `crates/payment-api/src/lib.rs`
- `crates/app-server/Cargo.toml`
- `crates/app-server/src/main.rs`
- `crates/app-server/src/config.rs`
- `crates/app-server/src/db.rs`

---

## Change Log

| Date | Description |
|------|-------------|
| 2026-03-28 | Story 1.1 implémentée — workspace Cargo 14 crates, Docker Compose, migrations SQLx 4 schémas, isolation domaine vérifiée |
