# Story 2.1 — Domaine Identity : User, Role & Credentials (TDD)

Status: ready-for-dev

## Story

En tant que développeur,
Je veux le modèle de domaine Identity implémenté en TDD pur,
Afin que les invariants métier de l'inscription et de l'authentification soient encodés dans des tests avant tout code d'infrastructure.

## Acceptance Criteria

**CA-1 — Tests domaine purs**
- **Given** la crate `identity-domain`,
- **When** `cargo test -p identity-domain` est exécuté,
- **Then** tous les tests passent sans aucune dépendance tokio/sqlx/axum

**CA-2 — Conflict email**
- **Given** l'aggregate `User` et le trait `UserRepository`,
- **When** un `User` est créé avec un email déjà pris (simulé via mock du trait),
- **Then** le domaine retourne `DomainError::Conflict("email already exists")`

**CA-3 — Validation mot de passe court**
- **Given** le value object `PasswordHash`,
- **When** un mot de passe de moins de 8 caractères est fourni,
- **Then** le domaine retourne `DomainError::ValidationError("password too short")`

**CA-4 — EventPublisher via trait uniquement**
- **Given** les traits dans `ports.rs` (`UserRepository`, `EventPublisher`),
- **When** le domaine appelle `event_publisher.publish(UserRegistered { ... })`,
- **Then** c'est via le trait — aucune implémentation concrète (RabbitMQ, etc.) dans le domaine

## Tasks / Subtasks

- [ ] `DomainError` enum (AC: CA-2, CA-3)
  - [ ] Variantes : `NotFound`, `Conflict(String)`, `ValidationError(String)`, `Unauthorized`, `Forbidden`
  - [ ] `#[derive(Debug, thiserror::Error)]` — thiserror **v2** (déjà dans workspace)
- [ ] `credentials.rs` — value object `PasswordHash` (AC: CA-3)
  - [ ] Struct `PasswordHash(String)` — wraps le hash argon2 (chaîne stockée, pas le mot de passe en clair)
  - [ ] `PasswordHash::validate_password_strength(raw: &str) -> Result<(), DomainError>` — longueur ≥ 8
  - [ ] Note : le **hachage réel argon2** est dans `identity-infra` — le domaine ne connaît que la valeur hashée
- [ ] `user.rs` — aggregate `User` (AC: CA-2, CA-4)
  - [ ] Enum `Role { Maker, Buyer, Admin }` avec `#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]`
  - [ ] Enum `UserStatus { PendingValidation, Active, Rejected }` (Maker commence PendingValidation, Buyer commence Active)
  - [ ] Struct `User { id: IdentityId, email: String, password_hash: PasswordHash, role: Role, status: UserStatus, created_at: String }`
  - [ ] `User::register(id: IdentityId, email: String, password_hash: PasswordHash, role: Role) -> User` — constructeur factory
  - [ ] Pas de `User::new` qui valide l'unicité email — la validation unicité est dans le use case (infra), pas dans l'aggregate
- [ ] `events.rs` — domain events (AC: CA-4)
  - [ ] `UserRegistered { user_id: IdentityId, email: String, role: Role, occurred_at: String }`
  - [ ] Routing key convention : `identity.user.registered`
- [ ] `ports.rs` — traits domaine (AC: CA-2, CA-4)
  - [ ] `trait UserRepository` : `async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>` + `async fn save(&self, user: &User) -> Result<(), DomainError>`
  - [ ] `trait EventPublisher` : `async fn publish<T: Serialize>(&self, event: EventEnvelope<T>) -> Result<(), DomainError>`
  - [ ] **ATTENTION** : les traits utilisent `async fn` — ils doivent être `#[async_trait]` ou utiliser RPIT bounds si nécessaire. En édition 2024 Rust, `async fn in trait` est stable — pas besoin de `async_trait` crate. Vérifier la version stable utilisée (1.82 dans le Dockerfile).
- [ ] `lib.rs` — expose les modules publics
  - [ ] `pub mod credentials; pub mod errors; pub mod events; pub mod ports; pub mod user;`
- [ ] `tests/user_registration_test.rs` — tests TDD des invariants (AC: CA-1, CA-2, CA-3)
  - [ ] Mock manuel `FakeUserRepository` (struct locale dans le fichier test) — implémenter `UserRepository`
  - [ ] Mock manuel `FakeEventPublisher` — stocker les events publiés dans un `Vec`
  - [ ] Test : inscription réussie → User créé + event `UserRegistered` publié
  - [ ] Test : email déjà pris → `DomainError::Conflict`
  - [ ] Test : mot de passe trop court → `DomainError::ValidationError`
  - [ ] Test : rôle Maker → statut `PendingValidation` ; rôle Buyer → statut `Active`

## Dev Notes

### Périmètre de cette story

**CE QUE CETTE STORY IMPLÉMENTE :**
- `crates/identity-domain/src/` — fichiers `user.rs`, `credentials.rs`, `events.rs`, `ports.rs`, `errors.rs`, `lib.rs`
- `crates/identity-domain/tests/user_registration_test.rs` — tests TDD purs

**CE QUE CETTE STORY NE FAIT PAS :**
- Pas de hachage argon2 réel (→ Story 2.2, dans `identity-infra/argon2_hasher.rs`)
- Pas de migrations SQL (→ Story 2.2)
- Pas d'handlers HTTP (→ Story 2.2)
- Pas de `MakerProfile` (→ Story 3.1, car valeur métier distincte)
- Pas de JWT (→ Story 2.4)

### Règle d'isolation domaine — CRITIQUE

`identity-domain/Cargo.toml` a déjà la liste correcte :
```toml
[dependencies]
shared-kernel = { workspace = true }
serde = { workspace = true }
uuid = { workspace = true }
thiserror = { workspace = true }
# INTERDIT : tokio, sqlx, axum, lapin, aws-sdk-s3
```

**Ne jamais ajouter** tokio, sqlx, axum, lapin dans cette crate. Le CI clippy avec `-D warnings` le détecterait indirectement, mais l'isolation doit être explicite au niveau Cargo.toml.

### Async fn in trait — édition 2024 Rust 1.82+

`async fn` dans les traits est **stable depuis Rust 1.75**. Le Dockerfile utilise `rust:1.82-slim`. Pas besoin de la crate `async-trait`. Exemple :

```rust
// ports.rs — OK en Rust 1.75+
pub trait UserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
}
```

**Mais attention** : dans les tests, les mocks implémentant ces traits doivent utiliser `async fn` aussi. Les tests peuvent rester synchrones en wrappant avec `tokio::test` **uniquement dans les tests** — mais le domaine lui-même ne doit pas dépendre de tokio. Solution : utiliser des futures dans les tests ou rendre les faux repositories synchrones avec `std::future::ready`.

**Alternative plus simple pour les tests** : rendre les traits du domaine non-async et retourner `Result<_, DomainError>` directement — puis dans l'infra, les implémentations async wrappent. À choisir selon la simplicité : si les traits sont synchrones, les tests TDD sont triviaux. Si les traits sont async, utiliser `futures::executor::block_on` ou `tokio::test` dans les tests uniquement.

**Recommandation pour cette story** : traits synchrones dans `identity-domain` (les use cases async dans `identity-infra` font le bridge). C'est plus simple et cohérent avec "zéro dépendance tokio dans le domaine". Le domaine ne se soucie pas de l'async.

### PasswordHash — domaine vs infra

Le domaine stocke un `PasswordHash(String)` qui représente le hash déjà calculé. Le domaine ne fait **pas** le hachage argon2 lui-même — ce serait une dépendance infra (`argon2` crate). Le domaine valide uniquement la **force du mot de passe brut** avant hachage (longueur ≥ 8). Exemple de séparation :

```
Domaine : PasswordHash::validate_password_strength("abc") → Err(ValidationError("password too short"))
Infra    : argon2_hasher.hash("correct_password") → PasswordHash("$argon2id$...")
```

### `IdentityId` depuis shared-kernel

Utiliser `shared-kernel::ids::IdentityId` pour l'ID utilisateur (pas un `Uuid` nu). `IdentityId` est un newtype `IdentityId(pub Uuid)` déjà défini. Pour créer un nouvel ID : `IdentityId(shared_kernel::ids::new_id())`.

### `EventEnvelope<T>` depuis shared-kernel

Les domain events doivent être enveloppés avec `shared_kernel::events::EventEnvelope<T>` avant publication. Le trait `EventPublisher` dans `ports.rs` accepte `EventEnvelope<T>`. Exemple :

```rust
let envelope = EventEnvelope::new(
    "identity.user.registered",
    user.id.0, // Uuid depuis IdentityId
    UserRegistered { user_id: user.id, email: user.email.clone(), role: user.role.clone() },
);
event_publisher.publish(envelope)?;
```

### thiserror v2 — syntaxe mise à jour

Le workspace utilise `thiserror = "2"` (v2.x). La syntaxe a légèrement changé par rapport à v1 :
```rust
// v2 — inchangé pour les cas simples :
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("not found")]
    NotFound,
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    ValidationError(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
}
```
Pas de changement breaking pour ces patterns.

### Organisation des tests

Selon l'architecture (`architecture.md` § Structure Tests Rust) :
- Tests unitaires inline dans le fichier source via `#[cfg(test)]` pour des invariants unitaires simples
- Tests d'intégration domaine dans `tests/` — ici `user_registration_test.rs`

Pour Story 2.1, les tests d'intégration domaine (avec mocks) vont dans `tests/user_registration_test.rs`. Les tests unitaires purs (ex: `PasswordHash::validate_password_strength`) peuvent être inline dans `credentials.rs`.

### Conventions de nommage

Depuis `architecture.md` § Naming — Code Rust :
- Types/structs/enums : `PascalCase` → `User`, `Role`, `UserStatus`, `PasswordHash`, `DomainError`
- Fonctions/variables : `snake_case` → `register`, `find_by_email`, `validate_password_strength`
- Fichiers : `snake_case` → `user.rs`, `credentials.rs`, `ports.rs`

### Project Structure Notes

Structure canonique pour `identity-domain` (depuis `project-structure.md`) :
```
crates/identity-domain/
├── Cargo.toml  ← déjà présent, ne pas modifier les dépendances (ajouter seulement si indispensable)
├── src/
│   ├── lib.rs           ← à remplir avec pub mod declarations
│   ├── user.rs          ← User aggregate, Role, UserStatus
│   ├── credentials.rs   ← PasswordHash value object
│   ├── events.rs        ← UserRegistered event struct
│   ├── ports.rs         ← UserRepository, EventPublisher traits
│   └── errors.rs        ← DomainError enum
└── tests/
    └── user_registration_test.rs  ← tests TDD avec mocks manuels
```

Le répertoire `tests/` n'existe pas encore — le créer.

### Patterns établis dans les stories précédentes

- **Édition Rust 2024** — `use` statements groupés par crate (`imports_granularity = "Crate"` dans `rustfmt.toml`)
- **`#[serde(rename_all = "camelCase")]`** — obligatoire sur les structs exposées en API. Ici dans le domaine, les structs events n'ont pas encore besoin de cette annotation (les DTOs API dans `identity-api` le feront) — mais l'ajouter sur les events ne fait pas de mal pour la cohérence avec l'`EventEnvelope`
- **CI `SQLX_OFFLINE=true`** — sans impact sur cette crate (aucun sqlx)
- **`cargo fmt --all -- --check` + `cargo clippy --all-targets -- -D warnings`** — doit passer. Éviter les `use` inutilisés, les variables non utilisées, les warnings clippy

### Chaîne d'erreurs (pour anticiper l'infra)

Cette story établit `DomainError`. En Story 2.2, l'infra ajoutera :
```
DomainError → ApplicationError → ApiError (impl IntoResponse axum → RFC 7807)
```
Les variantes `DomainError` doivent couvrir tous les cas HTTP attendus :
- `NotFound` → 404
- `ValidationError(_)` → 422
- `Unauthorized` → 401
- `Forbidden` → 403
- `Conflict(_)` → 409

### References

- [Source: _bmad-output/planning-artifacts/epics.md#Story-2.1] — critères d'acceptation originaux
- [Source: _bmad-output/planning-artifacts/architecture.md#Règle-disolation-domaine] — zéro dépendance infra dans `*-domain`
- [Source: _bmad-output/planning-artifacts/architecture.md#D3] — JWT httpOnly (contexte auth, pas cette story)
- [Source: _bmad-output/planning-artifacts/architecture.md#Process-Gestion-derreurs] — chaîne DomainError → ApplicationError → ApiError
- [Source: _bmad-output/planning-artifacts/architecture.md#Structure-Tests-Rust] — tests inline + tests/
- [Source: _bmad-output/planning-artifacts/project-structure.md#identity-domain] — arbre canonique des fichiers
- [Source: crates/shared-kernel/src/ids.rs] — IdentityId newtype, new_id()
- [Source: crates/shared-kernel/src/events.rs] — EventEnvelope<T>
- [Source: crates/identity-domain/Cargo.toml] — dépendances autorisées

## Dev Agent Record

### Agent Model Used

claude-sonnet-4-6

### Debug Log References

### Completion Notes List

### File List
