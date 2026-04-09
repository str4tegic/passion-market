# Story 2.3 : Inscription Acheteur (FR2)

Status: ready-for-dev

## Story

En tant que visiteur,
Je veux créer un compte acheteur avec mon email et mon mot de passe,
Afin de pouvoir acheter des produits auprès des makers.

## Acceptance Criteria

**CA-1 — Création compte réussie**
**Given** `POST /api/v1/auth/register/buyer` avec `{ email, password }`,
**When** l'email n'existe pas encore en base,
**Then** le compte est créé avec le rôle `Buyer`, statut `Active`
**And** le mot de passe est hashé via argon2id (jamais stocké en clair)
**And** la réponse HTTP est `201 Created` avec `{ userId, email, role: "Buyer" }`

**CA-2 — Email déjà utilisé**
**Given** `POST /api/v1/auth/register/buyer` avec un email déjà enregistré,
**When** la requête est traitée,
**Then** la réponse est `409 Conflict` au format RFC 7807

**CA-3 — Frontend : redirection après inscription**
**Given** la page `/auth/register/buyer`,
**When** le formulaire est soumis avec des données valides,
**Then** l'utilisateur est redirigé vers `/auth/login` avec un message de succès
**Note :** La connexion automatique (session JWT) est story 2.4 — hors scope ici.

**CA-4 — Frontend : validation inline**
**Given** le formulaire d'inscription acheteur,
**When** l'email ou le mot de passe est invalide (format ou longueur),
**Then** un message d'erreur inline est affiché sous le champ concerné, sans rechargement de page

## Tasks / Subtasks

### Backend — identity-api

- [ ] `identity-api/src/handlers/register_buyer.rs` — nouveau handler axum thin (AC: CA-1, CA-2)
  - [ ] `RegisterBuyerRequest` : `{ email: String, password: String }` avec `#[serde(rename_all = "camelCase")]`
  - [ ] `RegisterBuyerResponse` : `{ userId: String, email: String, role: String }` avec `#[serde(rename_all = "camelCase")]`
  - [ ] Handler : `State(uc): State<Arc<dyn RegisterUserPort>>` + `Json(body): Json<RegisterBuyerRequest>`
  - [ ] Construire `RegisterUserCommand::new(body.email.clone(), body.password, "Buyer".into())` → map_err(ApiError::from)
  - [ ] Retourner `201 Created` avec `RegisterBuyerResponse`
  - [ ] **TDD** : test rouge en premier dans `identity-api/tests/register_buyer_handler_test.rs`
    - `inscription_buyer_valide_retourne_201`
    - `email_doublon_retourne_409_rfc7807`

- [ ] `identity-api/src/handlers/mod.rs` — ajouter `pub mod register_buyer` (AC: CA-1)

- [ ] `identity-api/src/router.rs` — ajouter route `POST /api/v1/auth/register/buyer` (AC: CA-1)
  - Réutiliser le même `Arc<dyn RegisterUserPort>` injecté en state
  - Attention : deux routes sur le même router — utiliser `Router::new().route(...).route(...).with_state(uc)`

### Backend — app-server

- [ ] `app-server/src/main.rs` — aucune modification nécessaire
  - Le `register_uc` est déjà injecté dans `identity_router(register_uc)` — le nouveau handler partage le même use case

### Frontend — api-client

- [ ] `frontend/packages/api-client/src/commands/auth.ts` — ajouter (AC: CA-1, CA-4)
  - [ ] `RegisterBuyerInput` : `{ email: string; password: string }`
  - [ ] `RegisterBuyerErrors` : `{ email?: string; password?: string }`
  - [ ] `validateRegisterBuyer(input)` : validation inline (email format + password ≥ 8 chars)
  - [ ] `registerBuyer(input)` : `apiPost<RegisterMakerOutput>('/api/v1/auth/register/buyer', input)`
  - [ ] Exporter depuis `src/index.ts`
  - [ ] Tests dans `auth.test.ts` : validation buyer (email invalide, password court, champs vides)

### Frontend — hooks

- [ ] `frontend/packages/hooks/src/useRegisterBuyer.ts` — hook identique à `useRegisterMaker` (AC: CA-1, CA-2, CA-3)
  - States : `idle | loading | success | error`
  - Exporter depuis `src/index.ts`
  - Test dans `useRegisterBuyer.test.ts` : succès → status success, erreur API → status error

### Frontend Web — Next.js

- [ ] `frontend/apps/web/src/app/auth/register/buyer/page.tsx` — formulaire inscription acheteur (AC: CA-3, CA-4)
  - Champs : email, password
  - Validation inline au blur (pattern de `register/maker/page.tsx`)
  - Succès → `router.push('/auth/login?registered=true')` (message login page story 2.4)
  - Erreur 409 → "Cet email est déjà utilisé."
  - Erreur générique → `error.detail || 'Une erreur est survenue.'`

### Frontend Mobile — Expo

- [ ] `frontend/apps/mobile/app/(auth)/register-buyer.tsx` — écran inscription acheteur (AC: CA-3, CA-4)
  - Champs : email, password (pattern de `register-maker.tsx`)
  - Succès → `router.push('/auth/login')` (ou `/` en attendant story 2.4)
  - Erreur 409 → message doublon email

- [ ] `frontend/apps/mobile/app/(auth)/_layout.tsx` — ajouter `Stack.Screen name="register-buyer"` (AC: CA-3)

## Dev Notes

### Réutilisation maximale — zéro duplication

Le domaine, l'infra et le use case sont **identiques** à la story 2.2. `User::register` détermine automatiquement le statut selon le rôle :
- `Buyer` → `UserStatus::Active` (déjà testé dans `identity-domain/src/user.rs:116`)
- `Maker` → `UserStatus::PendingValidation`

Le handler `register_buyer` est une copie quasi-identique de `register_maker` avec :
1. `"Buyer".into()` au lieu de `"Maker".into()`
2. Champ `name` absent de `RegisterBuyerRequest`
3. Endpoint `/api/v1/auth/register/buyer`

### Router — state partagé

Le même `Arc<dyn RegisterUserPort>` est partagé entre les deux routes. Pattern à suivre dans `router.rs` :

```rust
pub fn identity_router(uc: Arc<dyn RegisterUserPort>) -> Router {
    Router::new()
        .route("/api/v1/auth/register/maker", post(register_maker))
        .route("/api/v1/auth/register/buyer", post(register_buyer))
        .with_state(uc)
}
```

### CA-3 — Connexion automatique hors scope

L'epics mentionne "connexion automatique et redirection vers la page d'accueil avec session active". Les cookies JWT (access + refresh) sont story 2.4. Pour cette story, le frontend redirige vers `/auth/login` après inscription réussie. Le message de succès peut être passé via query param (`?registered=true`) et affiché sur la page login en story 2.4.

### Patterns établis en story 2.2 à respecter

| Zone | Pattern |
|------|---------|
| Test handler | `tower::ServiceExt::oneshot` avec `MockRegisterUserPort` |
| Erreur API | `ApiError::from(DomainError::...)` → RFC 7807 |
| Validation frontend | `validateRegisterBuyer` au `onBlur` + au submit |
| Hook | `useRegisterBuyer` : `idle/loading/success/error` + `input: RegisterBuyerInput` typé |
| api-client | `apiPost<Output>(url, input)` |

### TDD — ordre strict

1. Test rouge `identity-api/tests/register_buyer_handler_test.rs` (copie adaptée de `register_maker_handler_test.rs`)
2. Implémenter `register_buyer` handler
3. Ajouter route dans `router.rs`
4. Tests frontend api-client (jest/vitest) avant implémentation
5. Tests hook avant implémentation

### Fichiers à créer

```
crates/identity-api/src/handlers/register_buyer.rs   ← nouveau
crates/identity-api/tests/register_buyer_handler_test.rs ← nouveau
frontend/packages/api-client/src/commands/auth.ts    ← modifier (ajouter buyer)
frontend/packages/hooks/src/useRegisterBuyer.ts      ← nouveau
frontend/packages/hooks/src/useRegisterBuyer.test.ts ← nouveau
frontend/apps/web/src/app/auth/register/buyer/page.tsx ← nouveau
frontend/apps/mobile/app/(auth)/register-buyer.tsx   ← nouveau
```

### Fichiers à modifier

```
crates/identity-api/src/handlers/mod.rs   ← pub mod register_buyer
crates/identity-api/src/router.rs         ← route /register/buyer
frontend/packages/api-client/src/index.ts ← exports buyer
frontend/packages/hooks/src/index.ts      ← exports useRegisterBuyer
frontend/apps/mobile/app/(auth)/_layout.tsx ← Stack.Screen register-buyer
```

### Project Structure Notes

- Crates : `identity-api`, `identity-application`, `identity-infra` — aucune modification infra nécessaire
- Frontend workspace : `packages/api-client`, `packages/hooks`, `apps/web`, `apps/mobile`
- Schéma DB `identity.users` : aucune modification de migration — la colonne `role` accepte déjà "Buyer"

### References

- [Source: epics.md#Story 2.3] — AC originaux
- [Source: architecture.md#D7] — Use cases pattern (déviation assumée : `identity-application` en story 2.2)
- [Source: architecture.md#D4] — RFC 7807 Problem Details
- [Source: crates/identity-api/src/handlers/register_maker.rs] — pattern à dupliquer
- [Source: crates/identity-api/src/router.rs] — router existant à étendre
- [Source: crates/identity-domain/src/user.rs:49-52] — logique statut Buyer→Active
- [Source: frontend/packages/api-client/src/commands/auth.ts] — pattern registerMaker
- [Source: frontend/packages/hooks/src/useRegisterMaker.ts] — pattern hook

## Dev Agent Record

### Agent Model Used

claude-sonnet-4-6

### Debug Log References

### Completion Notes List

### File List
