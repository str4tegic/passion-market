# Compte rendu — Story 1.1 : Workspace Cargo & Environnement Docker Compose

> **Pour qui ?** Ce document est écrit pour un développeur PHP qui connaît Docker mais découvre Rust.
> **But :** Expliquer ce qui a été fait, pourquoi, et comment ça s'articule avec ce que tu connais déjà.

---

## Vue d'ensemble : qu'est-ce qu'on a construit ?

On a posé les fondations du projet. Rien de visible pour l'utilisateur encore — c'est l'équivalent de :
- Initialiser un projet Composer avec tous ses packages
- Configurer Docker Compose pour la base de données et les services
- Mettre en place la structure de dossiers du projet

En PHP/Symfony tu aurais fait : `composer create-project symfony/skeleton`, puis configuré `.env`, `docker-compose.yml`, etc. Ici c'est la même idée mais en Rust.

---

## 1. Le Workspace Cargo — l'équivalent de Composer

### En PHP tu connais...
```json
// composer.json
{
  "name": "mon-projet",
  "require": {
    "symfony/framework-bundle": "^7.0",
    "doctrine/orm": "^3.0"
  }
}
```

### En Rust c'est...
```toml
# Cargo.toml (à la racine)
[workspace]
members = [
    "crates/shared-kernel",
    "crates/catalog-domain",
    ...
]
```

**Cargo** est le gestionnaire de paquets de Rust (comme Composer).
**Cargo.toml** est l'équivalent de `composer.json`.
**Cargo.lock** est l'équivalent de `composer.lock` — il fige les versions exactes.

Un **workspace** Cargo, c'est un projet qui contient plusieurs sous-projets (appelés **crates**). C'est comme si ton projet Symfony était découpé en plusieurs packages Composer indépendants qui se connaissent entre eux.

### Pourquoi plusieurs crates ?

Le projet est découpé en **4 Bounded Contexts** (BCs) — des zones métier indépendantes :
- **Catalog** — les produits, boutiques, stocks
- **Identity** — les utilisateurs, authentification
- **Order** — les paniers, commandes
- **Payment** — les paiements Stripe

Chaque BC a **3 crates** :
| Crate | Rôle | Analogie PHP/Symfony |
|-------|------|----------------------|
| `*-domain` | Logique métier pure | Entités + Services métier, sans base de données |
| `*-infra` | Accès base de données, RabbitMQ | Repositories Doctrine, connexions |
| `*-api` | Routes HTTP | Controllers Symfony |

Et une crate transversale :
- **`shared-kernel`** — types partagés entre tous les BCs (comme un package utilitaire)
- **`app-server`** — le binaire qui lance tout (comme `public/index.php` mais en exécutable)

---

## 2. La structure de fichiers créée

```
passion-market/
├── Cargo.toml              ← "composer.json" du workspace
├── Cargo.lock              ← "composer.lock" — ne pas modifier à la main
├── rustfmt.toml            ← config du formateur de code (comme .editorconfig + php-cs-fixer)
├── .env.example            ← variables d'environnement (comme en Symfony)
├── .gitignore
│
├── docker-compose.yml      ← PostgreSQL + RabbitMQ + MinIO
├── docker-compose.test.yml ← PostgreSQL de test séparée
│
├── migrations/             ← Scripts SQL (comme les migrations Doctrine)
│   ├── 20260328000001_create_catalog_schema.sql
│   ├── 20260328000002_create_identity_schema.sql
│   ├── 20260328000003_create_order_schema.sql
│   └── 20260328000004_create_payment_schema.sql
│
└── crates/
    ├── shared-kernel/      ← types communs
    ├── catalog-domain/     ← logique métier Catalog
    ├── catalog-infra/      ← BDD + RabbitMQ Catalog
    ├── catalog-api/        ← routes HTTP Catalog
    ├── identity-domain/    ← ... (même structure pour les 4 BCs)
    ├── ...
    └── app-server/         ← point d'entrée, lance tout
```

---

## 3. Docker Compose — ce que tu connais déjà

On a 3 services configurés :

```yaml
services:
  postgres:    # Base de données relationnelle (comme MySQL mais mieux)
  rabbitmq:    # File de messages (pour la communication entre BCs)
  minio:       # Stockage de fichiers compatible S3 (photos, vidéos)
```

### PostgreSQL
Remplace MySQL. Même idée, syntaxe SQL très proche. Il tourne sur le port **5432** (MySQL c'est 3306).

Credentials : `passion` / `passion`, base : `passion_market`.

### RabbitMQ
C'est une **file de messages** (message broker). Quand un produit est vendu, au lieu que le service "Order" appelle directement le service "Catalog" pour réduire le stock, il envoie un message dans RabbitMQ. Le service Catalog le lit et met à jour le stock.

C'est le même principe que si tu utilisais Symfony Messenger avec un transport AMQP.

- Port **5672** : connexion applicative
- Port **15672** : interface web d'administration (utile pour déboguer)

### MinIO
Stockage de fichiers compatible avec l'API S3 d'AWS. On l'utilise en local pour stocker les photos de produits et vidéos de fabrication. En production, on utilisera Cloudflare R2 (même API S3, juste un endpoint différent).

- Port **9000** : API S3
- Port **9001** : console web

---

## 4. Les migrations SQLx — l'équivalent de Doctrine Migrations

En Symfony tu ferais :
```bash
php bin/console doctrine:migrations:generate
php bin/console doctrine:migrations:migrate
```

En Rust avec SQLx, les migrations sont des fichiers `.sql` dans `/migrations/`. Elles s'exécutent **automatiquement** au démarrage de l'application via ce code :

```rust
// app-server/src/db.rs
sqlx::migrate!("../../migrations").run(&pool).await?;
```

Pour cette Story 1.1, on a créé seulement les **schémas PostgreSQL** (les namespaces de tables) :

```sql
CREATE SCHEMA IF NOT EXISTS catalog;   -- comme un namespace pour les tables Catalog
CREATE SCHEMA IF NOT EXISTS identity;
CREATE SCHEMA IF NOT EXISTS "order";   -- "order" en guillemets car mot réservé SQL
CREATE SCHEMA IF NOT EXISTS payment;
```

Les tables elles-mêmes seront créées dans les stories suivantes (Story 4.1 pour catalog, Story 2.1 pour identity, etc.).

**Pourquoi des schémas séparés ?** Pour isoler les données de chaque BC. Les tables `catalog.products` et `identity.users` ne peuvent pas se faire de jointures SQL directement — elles communiquent via RabbitMQ. C'est la règle d'or du projet.

---

## 5. shared-kernel — les types communs

Ce crate contient des briques de base partagées entre tous les BCs :

### `ids.rs` — Identifiants UUID v7

```rust
// Chaque entité a un ID de type UUID v7 (auto-ordonné chronologiquement)
pub struct CatalogId(pub Uuid);
pub struct IdentityId(pub Uuid);
// etc.
```

**UUID v7** : une version d'UUID qui s'ordonne chronologiquement (contrairement à v4 qui est aléatoire). Utile pour les performances d'index PostgreSQL.

### `money.rs` — Les montants financiers

```rust
pub struct Money {
    pub amount_cents: u64,  // 29,90€ = 2990
    pub currency: &'static str,  // "EUR"
}
```

**Règle importante :** on ne stocke jamais les prix en float (`29.90`). Un `float` en informatique ne peut pas représenter exactement certains nombres décimaux, ce qui cause des erreurs d'arrondi. On stocke toujours en **centimes** (entiers). C'est la même règle qu'avec Stripe.

### `events.rs` — L'enveloppe des messages RabbitMQ

```rust
pub struct EventEnvelope<T> {
    pub event_type: String,     // "catalog.product.published"
    pub aggregate_id: Uuid,     // ID du produit concerné
    pub occurred_at: String,    // "2026-03-28T12:00:00Z"
    pub version: u32,           // 1
    pub data: T,                // les données spécifiques à l'événement
}
```

Chaque message envoyé dans RabbitMQ sera emballé dans cette enveloppe. Ça standardise le format et facilite le débogage.

### `pagination.rs` — La pagination

```rust
pub struct PageParams { pub page: u32, pub per_page: u32 }
pub struct Page<T> { pub items: Vec<T>, pub total: u64, ... }
```

L'équivalent du `Paginator` de Doctrine.

---

## 6. La règle d'or : isolation des crates domaine

C'est **la règle la plus importante** du projet.

Les crates `*-domain` (catalog-domain, identity-domain, etc.) **n'ont le droit d'utiliser que** :
- `shared-kernel`
- `serde` (sérialisation JSON)
- `uuid`
- `thiserror` (gestion des erreurs)

**Interdit dans les crates domaine :**
- `tokio` (runtime async — moteur d'exécution)
- `sqlx` (base de données)
- `axum` (HTTP)
- `lapin` (RabbitMQ)

**Pourquoi cette règle ?**

En PHP/Symfony, il arrive souvent qu'une entité Doctrine `User` soit directement liée à la base de données via les annotations `#[ORM\Entity]`. Si tu veux tester la logique métier de cette entité, tu dois monter toute l'infrastructure Doctrine.

Ici, la **logique métier** (le domaine) est **complètement indépendante** de la base de données. Tu peux tester toutes les règles métier avec des tests unitaires simples, sans base de données, sans Docker.

```
crate *-domain  : logique pure, testable sans rien démarrer
crate *-infra   : implémente l'accès BDD/RabbitMQ
crate *-api     : gère les requêtes HTTP
crate app-server: colle tout ensemble
```

C'est le pattern **DDD** (Domain-Driven Design) avec **architecture hexagonale** (ports & adapters).

---

## 7. app-server — le point d'entrée

```rust
// crates/app-server/src/main.rs
#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();           // charge le .env (comme en Symfony)

    // Init des logs
    tracing_subscriber::fmt()...init();

    // Connexion à PostgreSQL
    let pool = db::create_pool(&cfg.database_url).await?;

    // Migrations automatiques
    db::run_migrations(&pool).await?;

    // TODO Story 2+ : démarrer le serveur HTTP axum
}
```

**`#[tokio::main]`** : en Rust, tout le code async tourne sur un runtime. Tokio est ce runtime (comme Node.js est le runtime de JavaScript). Cette annotation dit "lance le runtime Tokio pour ce programme".

**`async/await`** : ça existe aussi en PHP depuis PHP 8.1 avec les Fibers, mais c'est bien moins courant. En Rust c'est central.

---

## 8. Les commandes de validation exécutées

```bash
# Compilation de tout le workspace
cargo build --workspace
# → Équivalent de "php -l" sur tout le projet + compilation des dépendances
# → RÉSULTAT : ✅ 14 crates compilées sans erreur

# Analyse statique du code (plus stricte que les warnings)
cargo clippy --all-targets -- -D warnings
# → Équivalent de PHPStan/Psalm niveau max
# → RÉSULTAT : ✅ zéro warning

# Vérification du format du code
cargo fmt --all -- --check
# → Équivalent de php-cs-fixer --dry-run
# → RÉSULTAT : ✅ tout est bien formaté

# Lancement des services Docker
docker compose up -d
# → PostgreSQL (:5432), RabbitMQ (:5672/:15672), MinIO (:9000/:9001)
# → RÉSULTAT : ✅ 3 services actifs

# Démarrage de l'app (applique les migrations)
cargo run -p app-server
# → Équivalent de "php bin/console doctrine:migrations:migrate" + démarrage serveur
# → RÉSULTAT : ✅ 4 schémas créés dans PostgreSQL

# Vérification de l'isolation domaine
cargo tree -p catalog-domain | grep -E "(tokio|sqlx|axum)"
# → RÉSULTAT : ✅ aucune dépendance infra trouvée
```

---

## 9. Ce qui vient ensuite

**Story 1.2** — Application Next.js 16 + proxy vers l'API Rust
- Créer le frontend en TypeScript/React
- Configurer le proxy pour rediriger `/api/*` vers l'API Rust

**Story 1.3** — CI GitHub Actions
- Automatiser les 3 commandes de vérification (fmt, clippy, tests) à chaque Pull Request
- Déploiement automatique sur Railway

**Story 2.1** — Domaine Identity (TDD)
- Premier vrai code métier : les entités `User`, `Role`, `Credentials`
- Écriture en TDD (tests d'abord, code ensuite)

---

## Résumé rapide

| Ce que tu connais en PHP | L'équivalent Rust dans ce projet |
|--------------------------|----------------------------------|
| `composer.json` | `Cargo.toml` |
| `composer.lock` | `Cargo.lock` |
| Packages Composer | Crates Cargo |
| Symfony Bundles | Bounded Contexts (BCs) |
| Entités Doctrine | Aggregates dans `*-domain` |
| Repositories Doctrine | Traits de port dans `*-domain`, implémentés dans `*-infra` |
| Controllers Symfony | Handlers dans `*-api` |
| `public/index.php` | `app-server` (binaire compilé) |
| Symfony Messenger (AMQP) | RabbitMQ via lapin |
| Migrations Doctrine | Migrations SQLx dans `/migrations/` |
| PHPStan/Psalm | `cargo clippy` |
| php-cs-fixer | `cargo fmt` |
| `.env` Symfony | `.env` + `dotenvy` crate |
