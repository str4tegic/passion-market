# Story 2.2 — Inscription Maker (FR1)

Status: ready-for-dev

## Story

En tant que visiteur,
Je veux créer un compte maker avec mon email et un mot de passe,
Afin de pouvoir commencer à configurer ma boutique sur passion-market.

## Acceptance Criteria

**CA-1 — Création compte réussie**
**Given** `POST /api/v1/auth/register/maker` avec `{ email, password, name }`,
**When** l'email n'existe pas encore en base,
**Then** le compte est créé avec rôle `Maker`, statut `PendingValidation`
**And** le mot de passe est hashé via argon2id (jamais stocké en clair)
**And** la réponse HTTP est `201 Created` avec `{ userId, email, role: "maker" }`

**CA-2 — Email déjà utilisé**
**Given** `POST /api/v1/auth/register/maker` avec un email déjà enregistré,
**When** la requête est traitée,
**Then** la réponse est `409 Conflict` au format RFC 7807

**CA-3 — Frontend : redirection succès**
**Given** la page `/auth/register/maker`,
**When** le formulaire est soumis avec des données valides,
**Then** l'utilisateur est redirigé vers `/auth/register/maker/confirmation` avec le message "Compte créé — en attente de validation"

**CA-4 — Frontend : validation inline**
**Given** le formulaire d'inscription,
**When** l'email ou le mot de passe est invalide (format ou longueur),
**Then** un message d'erreur inline est affiché sous le champ concerné, sans rechargement de page

## Tasks / Subtasks

### Domaine — identity-domain

- [ ] Ajouter `async_trait` dans `Cargo.toml` (`identity-domain`) (AC: CA-1)
  - `async_trait` est un proc macro sans dépendance tokio/sqlx/axum — isolement domaine préservé
- [ ] `identity-domain/src/ports.rs` — ajouter deux nouveaux traits (AC: CA-1)
  - [ ] `trait PasswordHasher` (synchrone — CPU-bound) : `fn hash(&self, raw: &str) -> Result<PasswordHash, DomainError>`
  - [ ] `#[async_trait] trait RegisterMakerPort: Send + Sync` : `async fn execute(&self, email: String, password: String, name: String) -> Result<IdentityId, DomainError>`
- [ ] `identity-domain/src/user.rs` — ajouter `Serialize/Deserialize` sur `UserStatus` (AC: CA-1)
  - `UserStatus` n'a pas ces derives — nécessaire pour la persistance SQL et les logs

### Infra — identity-infra

- [ ] Ajouter `argon2`, `async_trait` dans `identity-infra/Cargo.toml` (AC: CA-1)
- [ ] Ajouter `argon2` dans `Cargo.toml` (workspace) : `argon2 = "0.5"` (AC: CA-1)
- [ ] `identity-infra/src/argon2_hasher.rs` — `pub struct Argon2PasswordHasher`, implémente `PasswordHasher` (AC: CA-1)
  - [ ] `hash(&self, raw)` : valide via `PasswordHash::validate_password_strength(raw)?`, hash via argon2id, retourne `PasswordHash(hash_string)` en construction **directe** (bypasse `PasswordHash::new()`)
  - [ ] Test TDD rouge en premier : hash d'un mot de passe valide → `PasswordHash` dont la valeur commence par `$argon2`
- [ ] `identity-infra/src/use_cases/register_maker.rs` — `pub struct RegisterMakerUseCase { pool: PgPool, hasher: Argon2PasswordHasher }`, implémente `RegisterMakerPort` (AC: CA-1, CA-2)
  - [ ] Test TDD rouge en premier : use case avec vraie DB (test d'intégration, voir section Tests)
  - [ ] Implémenter `execute()` async :
    1. Valider email (format basique, non vide)
    2. Valider + hasher password via `self.hasher.hash(&password)?`
    3. Vérifier unicité email : `SELECT id FROM identity.users WHERE email = $1` → `409 Conflict` si trouvé
    4. Générer `id = IdentityId(Uuid::now_v7())`
    5. Appeler `User::register(id, email, hash, Role::Maker, IsoDateTime::new(Utc::now()...)?)` → `(user, event)`
    6. Insérer en base : `INSERT INTO identity.users (id, email, password_hash, role, status, created_at)`
    7. Log de l'événement (Noop publisher pour cette story)
    8. Retourner `Ok(user.id)`
- [ ] `identity-infra/src/lib.rs` — exposer `pub mod argon2_hasher; pub mod use_cases;` (AC: CA-1)

### Migration SQL

- [ ] `migrations/20260402000001_create_identity_users.sql` — créer la table `identity.users` (AC: CA-1)

### API — identity-api

- [ ] `identity-api/Cargo.toml` — ajouter `async_trait` (AC: CA-1)
- [ ] `identity-api/src/errors.rs` — `pub struct ApiError`, implémente `IntoResponse` (RFC 7807) (AC: CA-1, CA-2)
  - [ ] Test TDD rouge en premier : `ApiError::from(DomainError::Conflict(...))` → `StatusCode::CONFLICT`
  - [ ] `From<DomainError>` : `Conflict` → 409, `ValidationError` → 422, `NotFound` → 404, `Unauthorized` → 401, `Forbidden` → 403, autres → 500
- [ ] `identity-api/src/handlers/register_maker.rs` — handler axum thin (AC: CA-1, CA-2)
  - [ ] `RegisterMakerRequest` : `{ email: String, password: String, name: String }` avec `#[serde(rename_all = "camelCase")]`
  - [ ] `RegisterMakerResponse` : `{ userId: String, email: String, role: String }` avec `#[serde(rename_all = "camelCase")]`
  - [ ] Handler : `State(uc): State<Arc<dyn RegisterMakerPort>>` + `Json(body): Json<RegisterMakerRequest>`
  - [ ] Retourne `(StatusCode::CREATED, Json(response))` ou `ApiError`
- [ ] `identity-api/src/router.rs` — `pub fn identity_router(uc: Arc<dyn RegisterMakerPort>) -> Router` (AC: CA-1)
  - Route : `POST /api/v1/auth/register/maker` → handler `register_maker`
- [ ] `identity-api/src/lib.rs` — exposer `pub use router::identity_router; pub mod errors; pub mod handlers;`

### Binaire — app-server

- [ ] `app-server/src/main.rs` — monter le router identity + injection des dépendances (AC: CA-1)
  - Remplacer le `// TODO Story 2+` par l'instantiation et le merge du router identity

### Frontend — Next.js

- [ ] `frontend/packages/api-client/src/commands/auth.ts` — `registerMaker(payload)` via `apiPost` (AC: CA-3, CA-4)
  - [ ] Décommenter `export * from './commands/auth'` dans `index.ts`
- [ ] Page `frontend/apps/web/src/app/auth/register/maker/page.tsx` — formulaire `'use client'` (AC: CA-3, CA-4)
  - Champs : email, password, name
  - Validation inline (React state + validation on blur)
  - Redirection vers `/auth/register/maker/confirmation` si 201
  - Affichage `detail` du RFC 7807 si erreur API
- [ ] Page `frontend/apps/web/src/app/auth/register/maker/confirmation/page.tsx` — message statique (AC: CA-3)

### CI — Préparation SQLx offline

- [ ] Après avoir écrit les queries SQLx, exécuter `cargo sqlx prepare --workspace` (avec `DATABASE_URL` locale) pour générer `.sqlx/`
  - Le CI a `SQLX_OFFLINE=true` — **sans ce fichier, le CI cassera**
  - Le répertoire `.sqlx/` n'existe pas encore — le créer et le commiter

## Dev Notes

### CRITIQUE — `argon2` absent du workspace

`argon2` n'est **pas encore** dans `Cargo.toml`. Il faut l'ajouter en deux endroits :

```toml
# Cargo.toml (workspace [workspace.dependencies])
argon2 = "0.5"

# crates/identity-infra/Cargo.toml
argon2 = { workspace = true }
```

### CRITIQUE — Construction de `PasswordHash` dans l'infra

`PasswordHash::new(raw: String)` dans le domaine valide la force et stocke la string telle quelle (placeholder — commentaire "In a real implementation, you would hash the password here"). L'infra **ne doit pas** appeler `PasswordHash::new(argon2_hash)` car cela validerait la string hashée au lieu du mot de passe brut.

Le champ est `pub` — construction directe autorisée dans l'infra :

```rust
// identity-infra/src/argon2_hasher.rs
use argon2::{Argon2, PasswordHasher as _, password_hash::{SaltString, rand_core::OsRng}};
use identity_domain::{errors::DomainError, password_hash::PasswordHash};

pub struct Argon2PasswordHasher;

impl identity_domain::ports::PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, raw: &str) -> Result<PasswordHash, DomainError> {
        PasswordHash::validate_password_strength(raw)?;          // valide le brut
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(raw.as_bytes(), &salt)
            .map_err(|_| DomainError::ValidationError("hashing failed".to_string()))?
            .to_string();
        Ok(PasswordHash(hash))                                    // construction directe
    }
}
```

### CRITIQUE — `RegisterMakerUseCase` : approche async directe (sans pont sync)

Le use case infra est entièrement async et **contourne** la fonction `identity_domain::use_cases::register_user()` (qui attend des traits synchrones). Il appelle les méthodes de l'aggregate directement :

```rust
use identity_domain::{
    password_hash::PasswordHash,
    ports::RegisterMakerPort,
    user::{Role, User},
};
use shared_kernel::{ids::IdentityId, iso_date_time::IsoDateTime};
use uuid::Uuid;

#[async_trait::async_trait]
impl RegisterMakerPort for RegisterMakerUseCase {
    async fn execute(&self, email: String, password: String, name: String) 
        -> Result<IdentityId, DomainError> 
    {
        let hash = self.hasher.hash(&password)?;

        let existing = sqlx::query_scalar!(
            "SELECT id FROM identity.users WHERE email = $1", email
        ).fetch_optional(&self.pool).await
         .map_err(|e| DomainError::ValidationError(e.to_string()))?;
        if existing.is_some() {
            return Err(DomainError::Conflict("email already exists".to_string()));
        }

        let id = IdentityId(Uuid::now_v7());
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = IsoDateTime::new(now)?;
        let (user, _event) = User::register(id, email.clone(), hash, Role::Maker, created_at)?;

        sqlx::query!(
            "INSERT INTO identity.users (id, email, password_hash, role, status, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6)",
            user.id.0 as Uuid,
            user.email,
            user.password_hash.0,
            "Maker",
            "PendingValidation",
            user.created_at.to_string()  // IsoDateTime → String ISO 8601
        ).execute(&self.pool).await
         .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(user.id)
    }
}
```

La logique `identity_domain::use_cases::register_user()` reste valide pour les tests domaine purs (sync mocks). Le use case infra n'a pas à la réutiliser.

### CRITIQUE — `IsoDateTime::to_string()` → vérifier l'API

`IsoDateTime` est dans `shared-kernel`. Vérifier que la méthode d'accès à la string interne est publique (le champ ou une méthode `as_str()`/`to_string()`). Si le champ est `IsoDateTime(pub String)`, utiliser `.0` directement.

### Migration SQL — `identity.users`

La migration `20260328000002_create_identity_schema.sql` crée uniquement le schéma `identity`. Créer une **nouvelle** migration :

```sql
-- migrations/20260402000001_create_identity_users.sql
CREATE TABLE IF NOT EXISTS identity.users (
    id            UUID        PRIMARY KEY,
    email         TEXT        NOT NULL UNIQUE,
    password_hash TEXT        NOT NULL,
    role          TEXT        NOT NULL CHECK (role IN ('Maker', 'Buyer', 'Admin')),
    status        TEXT        NOT NULL CHECK (status IN ('PendingValidation', 'Active')),
    created_at    TEXT        NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_identity_users_email ON identity.users(email);
```

`created_at` est stocké en `TEXT` pour correspondre à `IsoDateTime(String)` dans le domaine. Convention naming migrations : `{YYYYMMDDHHMMSS}_{description}.sql`. Les migrations sont dans `/migrations/` (racine workspace) — `db.rs` fait `sqlx::migrate!("../../migrations")` depuis `crates/app-server`.

### ApiError — RFC 7807

```rust
// identity-api/src/errors.rs
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use identity_domain::errors::DomainError;
use serde_json::json;

pub struct ApiError {
    status: StatusCode,
    title: &'static str,
    detail: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({
            "type": "about:blank",
            "title": self.title,
            "status": self.status.as_u16(),
            "detail": self.detail,
        });
        (self.status, Json(body)).into_response()
    }
}

impl From<DomainError> for ApiError {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::Conflict(msg) =>
                ApiError { status: StatusCode::CONFLICT, title: "Conflict", detail: msg },
            DomainError::ValidationError(msg) =>
                ApiError { status: StatusCode::UNPROCESSABLE_ENTITY, title: "Unprocessable Entity", detail: msg },
            DomainError::NotFound =>
                ApiError { status: StatusCode::NOT_FOUND, title: "Not Found", detail: "resource not found".into() },
            DomainError::Unauthorized =>
                ApiError { status: StatusCode::UNAUTHORIZED, title: "Unauthorized", detail: "unauthorized".into() },
            DomainError::Forbidden =>
                ApiError { status: StatusCode::FORBIDDEN, title: "Forbidden", detail: "forbidden".into() },
        }
    }
}
```

### Injection dans app-server

`identity-api` **ne dépend pas** de `identity-infra`. L'injection est faite dans `app-server/src/main.rs` :

```rust
use std::sync::Arc;

// Instanciation infra
let hasher = identity_infra::Argon2PasswordHasher;
let register_uc: Arc<dyn identity_domain::ports::RegisterMakerPort> =
    Arc::new(identity_infra::use_cases::register_maker::RegisterMakerUseCase::new(pool.clone(), hasher));

// Merge router
let app = Router::new()
    .merge(health::router())
    .merge(identity_api::identity_router(register_uc));
```

### SQLx offline — action requise AVANT commit

Le CI utilise `SQLX_OFFLINE=true` (défini en story 1.3). Le répertoire `.sqlx/` **n'existe pas encore**. Après avoir écrit les queries `sqlx::query!` dans `identity-infra` :

```bash
# Terminal avec DATABASE_URL pointant sur la DB locale (Docker Compose)
cargo sqlx prepare --workspace
```

Le répertoire `.sqlx/` généré doit être **commité**. Sans ce fichier, le CI cassera.

### Tests TDD — ordre obligatoire (rouge → vert)

**Tests à écrire avant le code :**

1. `identity-infra/tests/argon2_hasher_test.rs` :
   - `hash_valide_retourne_hash_argon2` → vérifie que la valeur retournée commence par `$argon2`
   - `hash_password_trop_court_retourne_erreur` → `DomainError::ValidationError`

2. `identity-api/tests/register_maker_handler_test.rs` (avec axum TestClient) :
   - `inscription_maker_valide_retourne_201` → mock use case via `Arc<MockRegisterMakerPort>`
   - `email_doublon_retourne_409_rfc7807` → mock retourne `DomainError::Conflict`

3. `identity-infra/tests/register_maker_use_case_test.rs` (test d'intégration DB) :
   - Nécessite une `PgPool` connectée (`DATABASE_URL` dans l'env de test)
   - `inscription_maker_valide_insere_en_base` → vérifie que la ligne apparaît dans `identity.users`
   - `email_doublon_retourne_conflict` → deux inscriptions avec le même email

### Conventions établies (story 2.1)

- `imports_granularity = "Crate"` dans `rustfmt.toml` — grouper les `use` par crate
- `cargo clippy --all-targets -- -D warnings` doit passer — éliminer tous les warnings
- UUID v7 : `Uuid::now_v7()` (crate `uuid` avec feature `v7` déjà dans workspace)
- `#[serde(rename_all = "camelCase")]` sur `RegisterMakerRequest` et `RegisterMakerResponse`
- `UserStatus` manque `#[derive(Serialize, Deserialize)]` — l'ajouter dans `user.rs` (nécessaire pour cohérence, pas utilisé dans cette story mais évite la dette)

### Frontend — api-client

Le fichier `frontend/packages/api-client/src/commands/auth.ts` n'existe pas encore. L'import dans `index.ts` est commenté (`// export * from './commands/auth'`). À créer et décommenter.

```typescript
// frontend/packages/api-client/src/commands/auth.ts
import { apiPost } from '../lib/fetch'

export interface RegisterMakerPayload {
  email: string
  password: string
  name: string
}

export interface RegisterMakerResponse {
  userId: string
  email: string
  role: string
}

export const registerMaker = (payload: RegisterMakerPayload) =>
  apiPost<RegisterMakerResponse>('/api/v1/auth/register/maker', payload)
```

### Project Structure Notes

Fichiers nouveaux ou modifiés dans cette story :

```
Cargo.toml                                           ← + argon2 dans [workspace.dependencies]
migrations/
  20260402000001_create_identity_users.sql           ← NEW
.sqlx/                                               ← NEW (généré par cargo sqlx prepare)
crates/
  identity-domain/
    Cargo.toml                                       ← + async_trait
    src/
      ports.rs                                       ← + PasswordHasher, RegisterMakerPort traits
      user.rs                                        ← + Serialize/Deserialize sur UserStatus
  identity-infra/
    Cargo.toml                                       ← + argon2, async_trait
    src/
      lib.rs                                         ← exposer argon2_hasher, use_cases
      argon2_hasher.rs                               ← NEW
      use_cases/
        mod.rs                                       ← NEW
        register_maker.rs                            ← NEW
  identity-api/
    Cargo.toml                                       ← + async_trait
    src/
      lib.rs                                         ← exposer identity_router, errors, handlers
      errors.rs                                      ← NEW (ApiError + IntoResponse + From<DomainError>)
      router.rs                                      ← NEW
      handlers/
        mod.rs                                       ← NEW
        register_maker.rs                            ← NEW
  app-server/
    src/
      main.rs                                        ← remplacer TODO par injection + merge identity router
frontend/
  packages/api-client/src/
    index.ts                                         ← décommenter export commands/auth
    commands/
      auth.ts                                        ← NEW
  apps/web/src/app/
    auth/register/maker/
      page.tsx                                       ← NEW ('use client', formulaire)
      confirmation/
        page.tsx                                     ← NEW (message statique)
```

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story-2.2] — critères d'acceptation complets
- [Source: _bmad-output/planning-artifacts/architecture.md#D7] — use cases dans `{bc}-infra/src/use_cases/`, injection via traits domaine
- [Source: _bmad-output/planning-artifacts/architecture.md#Process-Gestion-erreurs] — chaîne `DomainError → ApplicationError → ApiError`
- [Source: _bmad-output/planning-artifacts/architecture.md#Règles-obligatoires] — `camelCase`, RFC 7807, UUID v7, zéro dep infra dans `*-domain`
- [Source: crates/identity-domain/src/use_cases.rs] — `register_user()` et `RegisterUserCommand` (infra bypasse pour l'async, mais pattern documenté)
- [Source: crates/identity-domain/src/ports.rs] — traits synchrones existants `UserRepository`, `EventPublisher`
- [Source: crates/identity-domain/src/password_hash.rs] — `PasswordHash(pub String)`, `validate_password_strength()`
- [Source: crates/identity-domain/src/domain_services.rs] — `UniqueEmailSpecification` (logique répliquée en async dans use case infra)
- [Source: crates/app-server/src/db.rs] — `sqlx::migrate!("../../migrations")` → migrations dans `/migrations/`
- [Source: migrations/20260328000002_create_identity_schema.sql] — schéma `identity` déjà créé
- [Source: crates/identity-api/Cargo.toml] — ne dépend pas de `identity-infra` (règle à respecter)
- [Source: frontend/packages/api-client/src/index.ts] — import `commands/auth` commenté, à décommenter

## Dev Agent Record

### Agent Model Used

claude-sonnet-4-6

### Debug Log References

### Completion Notes List

### File List
