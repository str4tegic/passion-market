---
stepsCompleted: [1, 2, 3, 4]
status: 'complete'
inputDocuments:
  - "_bmad-output/planning-artifacts/prd.md"
  - "_bmad-output/planning-artifacts/architecture.md"
  - "_bmad-output/planning-artifacts/project-structure.md"
---

# passion-market - Epic Breakdown

## Overview

Ce document fournit le découpage complet en epics et stories pour passion-market, décomposant les exigences du PRD, de l'Architecture et de la Structure Projet en stories implémentables.

## Requirements Inventory

### Functional Requirements

FR1: Un visiteur peut créer un compte maker avec email et mot de passe
FR2: Un visiteur peut créer un compte acheteur avec email et mot de passe
FR3: Un utilisateur peut se connecter et se déconnecter
FR4: Un utilisateur peut réinitialiser son mot de passe par email
FR5: Un maker peut renseigner son profil (nom, biographie, passion, photo atelier, vidéo de fabrication)
FR6: Un maker peut modifier son profil à tout moment
FR7: L'admin peut consulter la file des comptes makers en attente de validation
FR8: L'admin peut approuver un compte maker avec activation de la boutique
FR9: L'admin peut refuser un compte maker avec motif communiqué par email
FR10: Le système notifie le maker par email du résultat de la validation
FR11: Un maker validé peut créer une boutique avec nom et catégorie
FR12: Un maker peut créer un produit en brouillon (nom, description, prix, stock, photos)
FR13: Un maker peut attacher une vidéo de fabrication à un produit
FR14: Un maker peut publier un produit (stock > 0 requis)
FR15: Un maker peut mettre un produit en pause ou le supprimer
FR16: Un maker peut mettre à jour le stock d'un produit
FR17: Le système passe automatiquement un produit en rupture quand le stock atteint zéro
FR18: Le système repasse automatiquement un produit en ligne quand le stock est réapprovisionné
FR19: Un visiteur peut consulter la page publique d'un maker (profil, histoire, vidéo, boutique)
FR20: Un visiteur peut consulter le catalogue de produits d'une boutique
FR21: Un visiteur peut consulter la fiche détaillée d'un produit
FR22: Un acheteur peut ajouter un produit à sa liste de favoris
FR23: Un acheteur peut recevoir une alerte quand un produit favori redevient disponible
FR24: Un visiteur peut ajouter des produits à un panier
FR25: Le panier est persistant entre les sessions
FR26: Un acheteur peut finaliser une commande avec paiement en ligne (Stripe)
FR27: Le système calcule la commission plateforme sur chaque vente
FR28: Le système applique une commission réduite le premier weekend de chaque mois (Weekend Maker)
FR29: L'acheteur reçoit un email de confirmation de commande
FR30: Le maker reçoit une notification SSE en temps réel et un email récapitulatif lors de chaque nouvelle vente
FR31: Un maker reçoit une notification SSE en temps réel lors d'une rupture de stock
FR32: Le système envoie un email de relance à un acheteur ayant abandonné son panier
FR33: Le maker peut renseigner les informations d'expédition d'une commande
FR34: Un utilisateur peut consulter la politique de confidentialité (RGPD)
FR35: Un utilisateur peut demander la suppression de ses données personnelles
FR36: Les informations sur le droit de rétractation (14 jours) sont affichées lors de tout achat
FR37: Le système ne stocke aucune donnée de carte bancaire (délégation Stripe)

### NonFunctional Requirements

NFR1 (Performance): LCP < 2.5s, CLS < 0.1, FID < 100ms — pages maker/produit < 2s (réseau 4G) — actions critiques (ajout panier, commande) < 3s — SSE non bloquant pour le rendu
NFR2 (Sécurité): HTTPS TLS 1.2+, argon2/bcrypt pour mots de passe, PCI-DSS délégué à Stripe, adresses livraison isolées par maker, tokens avec expiration (JWT access 15min, refresh 7j), endpoints admin sur préfixe distinct `/admin/`
NFR3 (Scalabilité): Bounded Contexts indépendants déployables séparément, cible MVP < 100 makers / < 1 000 acheteurs, scalabilité horizontale préparée post-MVP
NFR4 (Accessibilité): WCAG 2.1 AA, navigation clavier complète, NVDA/VoiceOver/TalkBack, contraste ≥ 4.5:1, alt-text sur toutes les images, sous-titres sur vidéos de fabrication
NFR5 (Intégrations): Stripe webhooks validés côté serveur avant confirmation commande, email transactionnel fiabilité ≥ 99% (Resend), stockage médias S3-compatible (MinIO dev / Cloudflare R2 prod)
NFR6 (Maintenabilité): Couverture tests domaine Rust ≥ 80% (TDD), Bounded Contexts indépendants, API versionnée `/api/v1/`, CI enforce fmt + clippy -D warnings

### Additional Requirements

- **Greenfield — setup initial obligatoire** : Epic 1 Story 1 = initialiser le workspace Cargo + Docker Compose (PostgreSQL, RabbitMQ, MinIO) + Next.js 16 + CI GitHub Actions
- **Structure workspace** : crates/{bc}-domain / {bc}-infra / {bc}-api par BC (catalog, identity, order, payment) + shared-kernel + app-server (binaire)
- **Règle d'isolation domaine** : crates `{bc}-domain` zéro dépendance tokio/sqlx/axum — vérifiable à la compilation
- **Migrations SQLx** : globales dans `/migrations/`, exécutées automatiquement au démarrage de `app-server`
- **Schémas PostgreSQL séparés** : `catalog`, `identity`, `order`, `payment` — aucune jointure SQL inter-schéma
- **RabbitMQ** : topic exchange `{bc}.events`, routing key `{bc}.{aggregate}.{verbe_passé}`, enveloppeur `EventEnvelope` standardisé
- **Application Services (use cases)** : dans `{bc}-infra/src/use_cases/` — orchestrent domaine + infra — dispatch events après persistance
- **Consumers (subscribers)** : dans `{bc}-infra/src/consumers/` — enregistrés comme background tasks dans `app-server/messaging.rs`
- **Handlers API** : thin adapters dans `{bc}-api` — parsent HTTP, appellent use case via State<Arc<UseCase>>, retournent réponse
- **Sérialisation JSON** : `#[serde(rename_all = "camelCase")]` sur tous les DTOs API — UUID v7 string — dates ISO 8601 UTC — montants en centimes
- **Chaîne erreurs** : DomainError → ApplicationError → ApiError (impl IntoResponse axum → RFC 7807)
- **Stockage médias** : `aws-sdk-s3` endpoint configurable (MinIO dev, Cloudflare R2 prod) — trait `MediaStorage` dans `catalog-domain/ports.rs`
- **SSE** : endpoint `GET /api/v1/events` dans `catalog-api/sse.rs` — pousse domain events RabbitMQ vers clients Next.js connectés
- **Observabilité** : `tracing` + `tracing-subscriber` (JSON prod, pretty dev) — init dans `app-server/observability.rs`
- **Email transactionnel** : Resend API — service dans `app-server/mailer.rs`
- **CI lint** : `cargo fmt --all -- --check` + `cargo clippy --all-targets -- -D warnings`
- **Next.js** : App Router, ISR (revalidate: 60s) pour pages publiques (home, maker, produit), SSR pour pages transactionnelles (cart, checkout, orders, dashboard)
- **Next.js proxy** : rewrites dans `next.config.ts` — `/api/*` → `http://localhost:3001`
- **Ordre de développement** : Domain Rust → Infra Rust → API Rust → Front Next.js (par BC dans cet ordre)

### UX Design Requirements

_Aucun document UX Design disponible. Pas de UX-DR à extraire._

### FR Coverage Map

FR1: Epic 2 — Inscription maker
FR2: Epic 2 — Inscription acheteur
FR3: Epic 2 — Connexion/déconnexion
FR4: Epic 2 — Réinitialisation mot de passe
FR5: Epic 3 — Profil maker (bio, photo, vidéo)
FR6: Epic 3 — Modification profil maker
FR7: Epic 3 — File validation admin
FR8: Epic 3 — Approbation maker
FR9: Epic 3 — Refus maker avec motif
FR10: Epic 3 — Email résultat validation
FR11: Epic 4 — Création boutique
FR12: Epic 4 — Création produit brouillon
FR13: Epic 4 — Vidéo de fabrication produit
FR14: Epic 4 — Publication produit
FR15: Epic 4 — Pause/suppression produit
FR16: Epic 4 — Mise à jour stock
FR17: Epic 4 — Rupture stock automatique
FR18: Epic 4 — Remise en ligne automatique
FR19: Epic 5 — Page publique maker (ISR)
FR20: Epic 5 — Catalogue produits boutique (ISR)
FR21: Epic 5 — Fiche produit détaillée (ISR)
FR22: Epic 7 — Favoris acheteur
FR23: Epic 7 — Alerte disponibilité favori
FR24: Epic 6 — Ajout panier
FR25: Epic 6 — Panier persistant
FR26: Epic 6 — Commande + paiement Stripe
FR27: Epic 6 — Calcul commission
FR28: Epic 6 — Commission réduite Weekend Maker
FR29: Epic 6 — Email confirmation acheteur
FR30: Epic 6 — Notification SSE + email maker (nouvelle vente)
FR31: Epic 7 — Notification SSE rupture stock maker
FR32: Epic 7 — Email relance panier abandonné
FR33: Epic 7 — Infos expédition maker
FR34: Epic 8 — Politique de confidentialité RGPD
FR35: Epic 8 — Suppression données (droit à l'oubli)
FR36: Epic 8 — Affichage droit de rétractation 14j
FR37: Epic 6 — Zéro stockage données carte (délégation Stripe)

## Epic List

### Epic 1 — Socle Projet & Environnement
L'équipe peut développer, tester et déployer la plateforme dans un environnement stable et reproductible.
**FRs couverts :** aucun FR direct — prérequis technique pour tous les epics suivants

### Epic 2 — Identité & Authentification
Les visiteurs peuvent créer un compte et se connecter en tant que maker ou acheteur.
**FRs couverts :** FR1, FR2, FR3, FR4

### Epic 3 — Profil Maker & Validation Admin
Les makers peuvent présenter leur passion et être validés par l'admin pour accéder à la vente.
**FRs couverts :** FR5, FR6, FR7, FR8, FR9, FR10

### Epic 4 — Boutique & Gestion Catalogue
Les makers validés peuvent créer leur boutique, gérer leurs produits et leur stock.
**FRs couverts :** FR11, FR12, FR13, FR14, FR15, FR16, FR17, FR18

### Epic 5 — Storefront & Découverte Publique
Les visiteurs peuvent découvrir les makers et leurs produits via des pages SEO optimisées.
**FRs couverts :** FR19, FR20, FR21

### Epic 6 — Panier, Commande & Paiement
Les acheteurs peuvent acheter des produits en ligne via Stripe, avec commission et mécanique Weekend Maker.
**FRs couverts :** FR24, FR25, FR26, FR27, FR28, FR29, FR30, FR37

### Epic 7 — Favoris, Notifications & Communication
Les acheteurs reçoivent des alertes sur leurs favoris, les makers sont notifiés en temps réel et peuvent gérer les expéditions.
**FRs couverts :** FR22, FR23, FR31, FR32, FR33

### Epic 8 — Conformité RGPD & Réglementation
La plateforme respecte les obligations légales (RGPD, droit de rétractation, PCI-DSS).
**FRs couverts :** FR34, FR35, FR36

## Epic 1 — Socle Projet & Environnement

L'équipe peut développer, tester et déployer la plateforme dans un environnement stable et reproductible.

### Story 1.1 — Workspace Cargo & Environnement Docker Compose

En tant que développeur,
Je veux un workspace Cargo configuré avec toutes les crates et un Docker Compose fonctionnel,
Afin de pouvoir développer et tester tous les BCs localement sans friction.

**Critères d'Acceptation :**

**Given** le répertoire racine du projet,
**When** `cargo build --workspace` est exécuté,
**Then** toutes les crates (shared-kernel, {catalog,identity,order,payment}-{domain,infra,api}, app-server) compilent sans erreur
**And** `cargo clippy --all-targets -- -D warnings` ne produit aucun avertissement
**And** `cargo fmt --all -- --check` passe sans modification

**Given** le répertoire racine,
**When** `docker compose up -d` est exécuté,
**Then** PostgreSQL est accessible sur le port 5432
**And** RabbitMQ est accessible sur le port 5672 (UI management sur 15672)
**And** MinIO est accessible sur le port 9000

**Given** `app-server` est démarré avec `DATABASE_URL` configuré,
**When** le binaire s'exécute,
**Then** les migrations SQLx dans `/migrations/` sont appliquées automatiquement
**And** les 4 schémas PostgreSQL (`catalog`, `identity`, `order`, `payment`) existent

**Given** les crates `{bc}-domain`,
**When** `cargo test -p catalog-domain` (ou tout autre bc-domain) est exécuté,
**Then** les tests passent sans dépendance tokio/sqlx/axum

---

### Story 1.2 — Application Next.js 16 & Proxy vers l'API Rust

En tant que développeur,
Je veux une application Next.js 16 (App Router, TypeScript, Tailwind) configurée pour proxifier vers l'API Rust,
Afin que le frontend puisse communiquer avec le backend dès la première feature.

**Critères d'Acceptation :**

**Given** le répertoire `frontend/`,
**When** `npm run dev` est exécuté,
**Then** Next.js démarre sur le port 3000 sans erreur de compilation TypeScript

**Given** `next.config.ts` avec les rewrites configurés,
**When** une requête `GET /api/v1/health` est faite depuis le browser,
**Then** elle est proxifiée vers `http://localhost:3001/api/v1/health`

**Given** l'App Router,
**When** la page `/` est visitée,
**Then** une page placeholder "passion-market — bientôt" est affichée (SSG)
**And** aucune erreur 404 ou runtime n'est visible dans la console

**Given** `npm run build`,
**When** le build de production est lancé,
**Then** il se complète sans erreur TypeScript ni erreur ESLint

---

### Story 1.3 — CI GitHub Actions & Déploiement Railway

En tant que développeur,
Je veux une CI automatisée qui enforce la qualité du code et déploie sur Railway à chaque merge sur main,
Afin que chaque merge soit déployable et que la qualité soit garantie en continu.

**Critères d'Acceptation :**

**Given** une pull request ouverte,
**When** la CI s'exécute,
**Then** `cargo fmt --all -- --check` passe
**And** `cargo clippy --all-targets -- -D warnings` passe
**And** `cargo test --workspace` passe

**Given** un merge sur `main` avec CI verte,
**When** le workflow de déploiement se déclenche,
**Then** Railway reçoit le déploiement et le service redémarre avec les nouvelles variables d'environnement

**Given** une CI qui échoue,
**When** le développeur consulte GitHub,
**Then** le check en erreur est clairement identifié avec le message d'erreur complet
**And** le merge vers main est bloqué

## Epic 2 — Identité & Authentification

Les visiteurs peuvent créer un compte et se connecter en tant que maker ou acheteur.
**FRs couverts :** FR1, FR2, FR3, FR4

### Story 2.1 — Domaine Identity : User, Role & Credentials (TDD)

En tant que développeur,
Je veux le modèle de domaine Identity implémenté en TDD pur,
Afin que les invariants métier de l'inscription et de l'authentification soient encodés dans des tests avant tout code d'infrastructure.

**Critères d'Acceptation :**

**Given** la crate `identity-domain`,
**When** `cargo test -p identity-domain` est exécuté,
**Then** les tests passent sans aucune dépendance tokio/sqlx/axum

**Given** l'aggregate `User` avec `Role(Maker | Buyer | Admin)`,
**When** un `User` est créé avec un email déjà pris (simulé via le trait `UserRepository`),
**Then** le domaine retourne `DomainError::Conflict("email already exists")`

**Given** le value object `PasswordHash`,
**When** un mot de passe de moins de 8 caractères est fourni,
**Then** le domaine retourne `DomainError::ValidationError("password too short")`

**Given** les traits dans `ports.rs` (`UserRepository`, `EventPublisher`),
**When** le domaine appelle `event_publisher.publish(UserRegistered { ... })`,
**Then** c'est via le trait — aucune implémentation concrète dans le domaine

---

### Story 2.2 — Inscription Maker (FR1)

En tant que visiteur,
Je veux créer un compte maker avec mon email et un mot de passe,
Afin de pouvoir commencer à configurer ma boutique sur passion-market.

**Critères d'Acceptation :**

**Given** `POST /api/v1/auth/register/maker` avec `{ email, password, name }`,
**When** l'email n'existe pas encore en base,
**Then** le compte est créé avec le rôle `Maker`, statut `PendingValidation`
**And** le mot de passe est hashé via argon2 (jamais stocké en clair)
**And** la réponse HTTP est `201 Created` avec `{ userId, email, role: "maker" }`

**Given** `POST /api/v1/auth/register/maker` avec un email déjà utilisé,
**When** la requête est traitée,
**Then** la réponse est `409 Conflict` au format RFC 7807

**Given** la page d'inscription maker sur `/auth/register/maker`,
**When** le formulaire est soumis avec des données valides,
**Then** l'utilisateur est redirigé vers une page de confirmation "Compte créé — en attente de validation"

**Given** le formulaire d'inscription,
**When** l'email ou le mot de passe est invalide (format ou longueur),
**Then** un message d'erreur inline est affiché sous le champ concerné (sans rechargement de page)

---

### Story 2.3 — Inscription Acheteur (FR2)

En tant que visiteur,
Je veux créer un compte acheteur avec mon email et un mot de passe,
Afin de pouvoir acheter des produits auprès des makers.

**Critères d'Acceptation :**

**Given** `POST /api/v1/auth/register/buyer` avec `{ email, password }`,
**When** l'email n'existe pas encore en base,
**Then** le compte est créé avec le rôle `Buyer`, statut `Active`
**And** la réponse HTTP est `201 Created`

**Given** `POST /api/v1/auth/register/buyer` avec un email déjà utilisé,
**When** la requête est traitée,
**Then** la réponse est `409 Conflict` au format RFC 7807

**Given** la page `/auth/register/buyer`,
**When** le formulaire est soumis avec des données valides,
**Then** l'utilisateur est connecté automatiquement et redirigé vers la page d'accueil avec session active

---

### Story 2.4 — Connexion et Déconnexion (FR3)

En tant qu'utilisateur,
Je veux me connecter et me déconnecter de mon compte,
Afin que ma session soit sécurisée et persiste entre les visites.

**Critères d'Acceptation :**

**Given** `POST /api/v1/auth/login` avec `{ email, password }` valides,
**When** les credentials sont vérifiés via argon2,
**Then** un access token JWT (15min) et un refresh token (7j) sont émis en cookies httpOnly
**And** la réponse HTTP est `200 OK` avec `{ role, makerId? }`

**Given** `POST /api/v1/auth/login` avec un mot de passe incorrect,
**When** la requête est traitée,
**Then** la réponse est `401 Unauthorized` au format RFC 7807 (sans préciser si email ou password est incorrect)

**Given** `POST /api/v1/auth/logout`,
**When** la requête est traitée,
**Then** les cookies JWT sont supprimés (Set-Cookie avec `Max-Age=0`)
**And** la réponse HTTP est `204 No Content`

**Given** un access token expiré et un refresh token valide,
**When** `POST /api/v1/auth/refresh` est appelé,
**Then** un nouvel access token est émis
**And** l'ancien refresh token est invalidé (rotation)

**Given** la page `/auth/login`,
**When** la connexion réussit,
**Then** l'utilisateur est redirigé vers son dashboard (maker → `/dashboard`, acheteur → `/`)

---

### Story 2.5 — Réinitialisation du Mot de Passe (FR4)

En tant qu'utilisateur,
Je veux réinitialiser mon mot de passe via email,
Afin de récupérer l'accès à mon compte si je l'ai oublié.

**Critères d'Acceptation :**

**Given** `POST /api/v1/auth/password-reset/request` avec un email enregistré,
**When** la requête est traitée,
**Then** un token de réinitialisation (UUID v7, expiration 1h) est généré en base
**And** un email est envoyé via Resend avec un lien `/auth/password-reset?token=...`
**And** la réponse HTTP est `202 Accepted` (même si l'email n'existe pas — pour éviter l'énumération)

**Given** `POST /api/v1/auth/password-reset/confirm` avec un token valide et `{ newPassword }`,
**When** le token n'est pas expiré et n'a pas encore été utilisé,
**Then** le mot de passe est mis à jour (hashé argon2)
**And** le token est invalidé
**And** tous les refresh tokens existants sont révoqués
**And** la réponse HTTP est `200 OK`

**Given** un token expiré ou déjà utilisé,
**When** `POST /api/v1/auth/password-reset/confirm` est appelé,
**Then** la réponse est `410 Gone` au format RFC 7807

**Given** la page `/auth/password-reset`,
**When** le nouveau mot de passe est soumis avec succès,
**Then** l'utilisateur est redirigé vers `/auth/login` avec un message de confirmation

## Epic 3 — Profil Maker & Validation Admin

Les makers peuvent présenter leur passion et être validés par l'admin pour accéder à la vente.
**FRs couverts :** FR5, FR6, FR7, FR8, FR9, FR10

### Story 3.1 — Domaine Identity : MakerProfile & Événements de Validation (TDD)

En tant que développeur,
Je veux le modèle de domaine `MakerProfile` et les événements de validation implémentés en TDD,
Afin que les règles métier de l'approbation maker soient vérifiables sans infra.

**Critères d'Acceptation :**

**Given** la crate `identity-domain`,
**When** `approve_maker(maker_id)` est appelé sur un maker en statut `PendingValidation`,
**Then** le statut passe à `Approved`
**And** l'événement `MakerApproved { maker_id, approved_at }` est émis

**Given** `reject_maker(maker_id, reason)` appelé sur un maker `PendingValidation`,
**When** la raison est vide,
**Then** le domaine retourne `DomainError::ValidationError("reason required")`

**Given** un maker déjà `Approved`,
**When** `approve_maker` est appelé à nouveau,
**Then** le domaine retourne `DomainError::Conflict("already approved")`

---

### Story 3.2 — Profil Maker : Renseigner et Modifier (FR5, FR6)

En tant que maker,
Je veux renseigner et modifier mon profil (bio, passion, photo atelier, vidéo de fabrication),
Afin que les acheteurs découvrent qui je suis avant même de voir mes produits.

**Critères d'Acceptation :**

**Given** `PATCH /api/v1/makers/me` avec `{ biography, passion, workshopPhotoUrl }` et un JWT maker valide,
**When** les données sont valides,
**Then** le profil est mis à jour en base
**And** la réponse HTTP est `200 OK` avec le profil mis à jour

**Given** `POST /api/v1/makers/me/video` avec un fichier vidéo (multipart),
**When** le fichier est reçu,
**Then** la vidéo est uploadée dans MinIO/R2 via `s3_media_storage`
**And** l'URL CDN est stockée dans `maker_profile.fabrication_video_url`
**And** la réponse est `200 OK` avec `{ videoUrl }`

**Given** `PATCH /api/v1/makers/me` avec une biography de plus de 1000 caractères,
**When** la requête est traitée,
**Then** la réponse est `422 Unprocessable Entity` au format RFC 7807

**Given** la page `/dashboard/profile`,
**When** le maker soumet le formulaire avec des données valides,
**Then** un message de confirmation "Profil mis à jour" est affiché
**And** les modifications sont visibles immédiatement sur la page

---

### Story 3.3 — Dashboard Admin : File de Validation Makers (FR7)

En tant qu'admin,
Je veux consulter la file des comptes makers en attente de validation,
Afin de traiter les demandes d'inscription dans les meilleurs délais.

**Critères d'Acceptation :**

**Given** `GET /admin/makers?status=pending` avec un JWT rôle `Admin`,
**When** la requête est traitée,
**Then** la réponse est `200 OK` avec la liste paginée des makers `PendingValidation`
**And** chaque entrée contient `{ makerId, name, email, biography, workshopPhotoUrl, createdAt }`

**Given** `GET /admin/makers?status=pending` sans JWT Admin,
**When** la requête est traitée,
**Then** la réponse est `403 Forbidden` au format RFC 7807

**Given** la page `/admin/makers`,
**When** l'admin consulte la file,
**Then** chaque maker est affiché avec son nom, email, bio, photo atelier et date d'inscription
**And** les boutons "Approuver" et "Refuser" sont visibles pour chaque maker en attente

---

### Story 3.4 — Approbation et Refus Maker (FR8, FR9, FR10)

En tant qu'admin,
Je veux approuver ou refuser un compte maker avec notification par email,
Afin que les makers soient informés du résultat de leur demande.

**Critères d'Acceptation :**

**Given** `POST /admin/makers/:id/approve` avec un JWT Admin,
**When** le maker est en statut `PendingValidation`,
**Then** son statut passe à `Approved`
**And** l'événement `MakerApproved` est publié sur RabbitMQ (`identity.events`)
**And** un email "Votre boutique est activée" est envoyé via Resend
**And** la réponse HTTP est `200 OK`

**Given** `POST /admin/makers/:id/reject` avec `{ reason }` et un JWT Admin,
**When** la raison est fournie,
**Then** le statut du maker passe à `Rejected`
**And** l'événement `MakerRejected { maker_id, reason }` est publié sur RabbitMQ
**And** un email "Votre demande a été refusée" avec le motif est envoyé via Resend
**And** la réponse HTTP est `200 OK`

**Given** `POST /admin/makers/:id/reject` sans raison (`reason` vide ou absent),
**When** la requête est traitée,
**Then** la réponse est `422 Unprocessable Entity` au format RFC 7807

**Given** la page `/admin/makers`,
**When** l'admin clique "Approuver" puis confirme,
**Then** le maker disparaît de la file "En attente"
**And** une notification toast "Maker approuvé — email envoyé" est affichée

## Epic 4 — Boutique & Gestion Catalogue

Les makers validés peuvent créer leur boutique, gérer leurs produits et leur stock.
**FRs couverts :** FR11, FR12, FR13, FR14, FR15, FR16, FR17, FR18

### Story 4.1 — Domaine Catalog : Product, Shop, ProductStatus & Stock (TDD)

En tant que développeur,
Je veux le modèle de domaine Catalog complet implémenté en TDD,
Afin que le cycle de vie produit et les règles de gestion stock soient encodés en tests avant l'infra.

**Critères d'Acceptation :**

**Given** la crate `catalog-domain`,
**When** `cargo test -p catalog-domain` est exécuté,
**Then** tous les tests passent sans dépendance tokio/sqlx/axum

**Given** `product.publish()` appelé avec `stock = 0`,
**When** l'invariant est vérifié,
**Then** le domaine retourne `DomainError::ValidationError("stock must be > 0 to publish")`

**Given** un produit `Published` avec `stock = 1`,
**When** `product.decrement_stock()` est appelé,
**Then** le stock passe à `0` et le statut passe automatiquement à `OutOfStock`
**And** l'événement `StockDepleted { product_id }` est émis

**Given** un produit `OutOfStock`,
**When** `product.restock(quantity)` est appelé avec `quantity > 0`,
**Then** le statut repasse à `Published`
**And** l'événement `ProductRestocked { product_id, new_stock }` est émis

**Given** `product.pause()` appelé sur un produit `Published`,
**When** l'action est exécutée,
**Then** le statut passe à `Paused`
**And** l'événement `ProductPaused { product_id }` est émis

---

### Story 4.2 — Création de Boutique Maker (FR11)

En tant que maker validé,
Je veux créer ma boutique avec un nom et une catégorie,
Afin d'avoir un espace dédié pour présenter et vendre mes créations.

**Critères d'Acceptation :**

**Given** `POST /api/v1/shops` avec `{ name, category }` et un JWT maker `Approved`,
**When** le maker n'a pas encore de boutique,
**Then** la boutique est créée en base dans le schéma `catalog`
**And** la réponse HTTP est `201 Created` avec `{ shopId, name, category, makerId }`

**Given** `POST /api/v1/shops` avec un JWT maker `PendingValidation` ou `Rejected`,
**When** la requête est traitée,
**Then** la réponse est `403 Forbidden` au format RFC 7807

**Given** un maker essayant de créer une seconde boutique,
**When** la requête `POST /api/v1/shops` est traitée,
**Then** la réponse est `409 Conflict` (un maker = une boutique)

**Given** la page `/dashboard`,
**When** un maker approuvé se connecte pour la première fois,
**Then** il voit un formulaire de création de boutique
**And** après soumission, il est redirigé vers `/dashboard/products`

---

### Story 4.3 — Créer et Enrichir des Produits (FR12, FR13)

En tant que maker,
Je veux créer des produits en brouillon avec photos et vidéo de fabrication,
Afin de préparer mon catalogue avant de le publier.

**Critères d'Acceptation :**

**Given** `POST /api/v1/products` avec `{ title, description, priceInCents, stock, photos[] }` et un JWT maker,
**When** les données sont valides,
**Then** le produit est créé en statut `Draft` dans le schéma `catalog`
**And** les photos sont uploadées dans MinIO/R2 et leurs URLs CDN stockées
**And** la réponse HTTP est `201 Created` avec `{ productId, status: "draft" }`

**Given** `POST /api/v1/products/:id/video` avec un fichier vidéo,
**When** le produit appartient au maker authentifié,
**Then** la vidéo est uploadée dans MinIO/R2
**And** `product.fabrication_video_url` est mis à jour
**And** la réponse est `200 OK` avec `{ videoUrl }`

**Given** `POST /api/v1/products` avec `priceInCents <= 0`,
**When** la requête est traitée,
**Then** la réponse est `422 Unprocessable Entity` au format RFC 7807

**Given** la page `/dashboard/products/new`,
**When** le maker remplit le formulaire et uploade ses photos,
**Then** le produit apparaît dans la liste avec le badge "Brouillon"

---

### Story 4.4 — Cycle de Vie Produit : Publication, Pause, Suppression (FR14, FR15)

En tant que maker,
Je veux publier, mettre en pause ou supprimer mes produits,
Afin de contrôler ce qui est visible dans ma boutique à tout moment.

**Critères d'Acceptation :**

**Given** `POST /api/v1/products/:id/publish` avec un JWT maker propriétaire,
**When** le produit est en statut `Draft` et `stock > 0`,
**Then** le statut passe à `Published`
**And** l'événement `ProductPublished` est publié sur RabbitMQ (`catalog.events`)
**And** la réponse HTTP est `200 OK`

**Given** `POST /api/v1/products/:id/publish` avec `stock = 0`,
**When** la requête est traitée,
**Then** la réponse est `422 Unprocessable Entity` avec le détail "stock must be > 0 to publish"

**Given** `POST /api/v1/products/:id/pause`,
**When** le produit est `Published`,
**Then** le statut passe à `Paused`
**And** le produit n'est plus visible dans le catalogue public

**Given** `DELETE /api/v1/products/:id`,
**When** le produit est en statut `Draft` ou `Paused`,
**Then** le produit est supprimé de la base
**And** la réponse HTTP est `204 No Content`

**Given** `DELETE /api/v1/products/:id` sur un produit `Published`,
**When** la requête est traitée,
**Then** la réponse est `409 Conflict` (dépublier d'abord)

**Given** la page `/dashboard/products`,
**When** le maker clique "Publier" sur un brouillon,
**Then** le badge passe à "Publié" sans rechargement de page

---

### Story 4.5 — Gestion du Stock et Transitions Automatiques (FR16, FR17, FR18)

En tant que maker,
Je veux mettre à jour le stock de mes produits manuellement et voir les transitions automatiques,
Afin de ne jamais vendre ce que je n'ai plus en stock.

**Critères d'Acceptation :**

**Given** `PATCH /api/v1/products/:id/stock` avec `{ quantity: 5 }` et un JWT maker propriétaire,
**When** la requête est traitée,
**Then** le stock du produit est mis à jour en base
**And** si le produit était `OutOfStock` et `quantity > 0`, le statut repasse à `Published`
**And** l'événement `ProductRestocked` est publié sur RabbitMQ
**And** la réponse HTTP est `200 OK` avec le stock mis à jour

**Given** une commande confirmée qui décrémente le stock d'un produit `Published` à `0`,
**When** l'événement `StockDepleted` est traité par `catalog-infra`,
**Then** le statut du produit passe automatiquement à `OutOfStock` en base
**And** l'événement `ProductOutOfStock { product_id }` est publié sur RabbitMQ

**Given** `PATCH /api/v1/products/:id/stock` avec `{ quantity: -1 }`,
**When** la requête est traitée,
**Then** la réponse est `422 Unprocessable Entity`

**Given** la page `/dashboard/products`,
**When** un produit passe automatiquement à "Rupture de stock",
**Then** le badge change en "Rupture" et un indicateur visuel alerte le maker

---

### Story 4.6 — Dashboard Maker : Interface de Gestion Catalogue

En tant que maker,
Je veux une interface claire pour gérer l'ensemble de mon catalogue depuis mon dashboard,
Afin d'avoir une vue d'ensemble de mes produits et de leurs statuts en un coup d'œil.

**Critères d'Acceptation :**

**Given** la page `/dashboard/products` avec un JWT maker valide,
**When** la page est chargée (SSR),
**Then** tous les produits du maker sont listés avec leur titre, prix, stock, statut et photo principale

**Given** la liste des produits,
**When** un filtre de statut est sélectionné (Brouillon / Publié / Pause / Rupture),
**Then** la liste est filtrée côté client sans rechargement

**Given** un maker sans produits,
**When** la page `/dashboard/products` est chargée,
**Then** un état vide "Créez votre premier produit" avec un CTA est affiché

**Given** un produit dans la liste,
**When** le maker clique "Modifier",
**Then** il est redirigé vers `/dashboard/products/:id/edit` avec le formulaire prérempli

## Epic 5 — Storefront & Découverte Publique

Les visiteurs peuvent découvrir les makers et leurs produits via des pages SEO optimisées.
**FRs couverts :** FR19, FR20, FR21

### Story 5.1 — Page Publique Maker (FR19)

En tant que visiteur,
Je veux consulter la page publique d'un maker avec son profil, son histoire, sa vidéo et sa boutique,
Afin de découvrir qui fabrique les produits avant même de les regarder.

**Critères d'Acceptation :**

**Given** `GET /api/v1/makers/:slug` (endpoint public, sans auth),
**When** le slug correspond à un maker `Approved`,
**Then** la réponse est `200 OK` avec `{ name, biography, passion, workshopPhotoUrl, fabricationVideoUrl, shop: { name, category } }`

**Given** un slug inexistant,
**When** la requête est traitée,
**Then** la réponse est `404 Not Found` au format RFC 7807

**Given** la page `/makers/[slug]` rendue en ISR (revalidate: 60s),
**When** un visiteur arrive sur la page,
**Then** le profil maker est affiché avec : nom, photo atelier, biographie, passion, vidéo de fabrication (si présente)
**And** les balises Open Graph (`og:title`, `og:image`, `og:description`) et JSON-LD `Person` sont correctement générées
**And** le LCP est < 2.5s sur connexion 4G simulée

**Given** une vidéo de fabrication présente,
**When** la page est affichée,
**Then** la vidéo est jouable directement depuis la page
**And** une transcription ou des sous-titres sont disponibles (WCAG 2.1 AA)

---

### Story 5.2 — Catalogue Produits d'une Boutique (FR20)

En tant que visiteur,
Je veux consulter le catalogue de produits d'une boutique maker,
Afin de découvrir l'ensemble des créations disponibles à l'achat.

**Critères d'Acceptation :**

**Given** `GET /api/v1/shops/:shopId/products?status=published` (endpoint public),
**When** la boutique existe et contient des produits `Published`,
**Then** la réponse est `200 OK` avec la liste paginée `{ items: [{ productId, title, priceInCents, stock, photoUrl }], total, page }`

**Given** une boutique sans produits publiés,
**When** `GET /api/v1/shops/:shopId/products` est appelé,
**Then** la réponse est `200 OK` avec `{ items: [], total: 0 }`

**Given** la section catalogue sur `/makers/[slug]` rendue en ISR,
**When** un visiteur consulte la page,
**Then** les produits `Published` sont affichés en grille avec photo, titre et prix
**And** les produits `OutOfStock` affichent un badge "Rupture de stock" et ne sont pas cliquables pour l'achat

---

### Story 5.3 — Fiche Produit Détaillée (FR21)

En tant que visiteur,
Je veux consulter la fiche détaillée d'un produit avec toutes ses informations,
Afin de décider si je souhaite l'acheter en connaissance de cause.

**Critères d'Acceptation :**

**Given** `GET /api/v1/products/:id` (endpoint public),
**When** le produit est `Published`,
**Then** la réponse est `200 OK` avec `{ productId, title, description, priceInCents, stock, photos[], fabricationVideoUrl, maker: { name, slug } }`
**And** la réponse inclut les métadonnées JSON-LD `Product` pour le SEO

**Given** un produit `Draft` ou `Paused`,
**When** `GET /api/v1/products/:id` est appelé sans auth maker propriétaire,
**Then** la réponse est `404 Not Found`

**Given** la page `/products/[id]` rendue en ISR,
**When** un visiteur arrive sur la fiche,
**Then** toutes les photos sont affichées dans une galerie navigable
**And** la vidéo de fabrication est présente si disponible (avec sous-titres WCAG)
**And** le prix est affiché formaté (ex: "89,00 €")
**And** un bouton "Ajouter au panier" est visible et actif si `stock > 0`
**And** le bouton est remplacé par "Rupture de stock" si `stock = 0`
**And** les balises `og:*` et `twitter:card` sont correctement renseignées

**Given** un visiteur sur la fiche produit,
**When** il clique sur le nom du maker,
**Then** il est redirigé vers `/makers/[slug]` du maker concerné

## Epic 6 — Panier, Commande & Paiement

Les acheteurs peuvent acheter des produits en ligne via Stripe, avec commission et mécanique Weekend Maker.
**FRs couverts :** FR24, FR25, FR26, FR27, FR28, FR29, FR30, FR37

### Story 6.1 — Domaine Order : Cart, Order, Commission & Weekend Maker (TDD)

En tant que développeur,
Je veux le modèle de domaine Order complet implémenté en TDD,
Afin que les invariants du panier, de la commande et de la mécanique Weekend Maker soient vérifiables sans infra.

**Critères d'Acceptation :**

**Given** la crate `order-domain`,
**When** `cargo test -p order-domain` est exécuté,
**Then** tous les tests passent sans dépendance tokio/sqlx/axum

**Given** `cart.add_item(product_id, quantity, price)`,
**When** le même produit est ajouté deux fois,
**Then** la quantité est cumulée (pas deux lignes distinctes)

**Given** `order.place()` depuis un panier vide,
**When** l'invariant est vérifié,
**Then** le domaine retourne `DomainError::ValidationError("cart is empty")`

**Given** `commission.calculate(amount, date)` avec une date en premier weekend du mois,
**When** le calcul est effectué,
**Then** le taux réduit "Weekend Maker" est appliqué

**Given** `commission.calculate(amount, date)` avec une date hors weekend Maker,
**When** le calcul est effectué,
**Then** le taux standard est appliqué

**Given** des tests de dates limites pour le Weekend Maker,
**When** les tests couvrent vendredi 23h59, samedi 00h00, dimanche 23h59, lundi 00h00 du premier weekend,
**Then** tous les cas limites retournent le résultat attendu

---

### Story 6.2 — Panier Persistant (FR24, FR25)

En tant que visiteur ou acheteur,
Je veux ajouter des produits à un panier qui persiste entre mes sessions,
Afin de pouvoir reprendre mes achats là où je les ai laissés.

**Critères d'Acceptation :**

**Given** `POST /api/v1/carts/items` avec `{ productId, quantity }`,
**When** le produit est `Published` et a du stock suffisant,
**Then** l'item est ajouté au panier (créé si inexistant)
**And** la réponse est `200 OK` avec le panier complet `{ cartId, items[], totalInCents }`

**Given** un visiteur non connecté ajoutant un produit au panier,
**When** il se connecte ensuite,
**Then** le panier anonyme (stocké en localStorage ou cookie) est fusionné avec son panier serveur

**Given** `DELETE /api/v1/carts/items/:itemId`,
**When** l'item appartient au panier de l'utilisateur,
**Then** l'item est retiré
**And** la réponse est `200 OK` avec le panier mis à jour

**Given** `POST /api/v1/carts/items` avec une quantité supérieure au stock disponible,
**When** la requête est traitée,
**Then** la réponse est `409 Conflict` avec le détail "insufficient stock"

**Given** la page `/cart` en SSR,
**When** un acheteur connecté la consulte,
**Then** tous ses items sont affichés avec photo, titre, prix unitaire, quantité et sous-total
**And** le total du panier est visible en bas de page

---

### Story 6.3 — Checkout & Paiement Stripe (FR26, FR37)

En tant qu'acheteur,
Je veux finaliser ma commande avec un paiement sécurisé via Stripe,
Afin d'acheter les produits de makers sans que mes données bancaires ne soient jamais stockées sur la plateforme.

**Critères d'Acceptation :**

**Given** `POST /api/v1/orders` avec un JWT acheteur et un panier non vide,
**When** le stock est suffisant pour tous les items,
**Then** un `PaymentIntent` Stripe est créé côté serveur
**And** le `client_secret` est retourné au frontend (jamais les données carte)
**And** la réponse est `201 Created` avec `{ orderId, clientSecret, totalInCents }`

**Given** le webhook Stripe `payment_intent.succeeded` reçu sur `POST /api/v1/webhooks/stripe`,
**When** la signature HMAC est valide,
**Then** la commande passe au statut `Confirmed`
**And** l'événement `PaymentConfirmed { order_id }` est publié sur RabbitMQ (`payment.events`)
**And** le stock des produits concernés est décrémenté via le consumer `catalog-infra`

**Given** le webhook Stripe `payment_intent.payment_failed`,
**When** la signature est valide,
**Then** la commande passe au statut `Failed`
**And** le stock réservé est libéré

**Given** un webhook avec une signature invalide,
**When** `POST /api/v1/webhooks/stripe` est appelé,
**Then** la réponse est `400 Bad Request` et le webhook est ignoré

**Given** la page `/checkout` en SSR,
**When** l'acheteur arrive sur la page,
**Then** le récapitulatif de commande est affiché (items, total, infos livraison)
**And** le formulaire Stripe Elements est chargé (zéro donnée carte ne transite par nos serveurs)
**And** le droit de rétractation de 14 jours est affiché de manière visible avant confirmation

---

### Story 6.4 — Commission & Mécanique Weekend Maker (FR27, FR28)

En tant que plateforme,
Je veux calculer automatiquement la commission sur chaque vente avec un taux réduit le premier weekend du mois,
Afin de rémunérer la plateforme tout en créant un rendez-vous mensuel pour les makers.

**Critères d'Acceptation :**

**Given** une commande confirmée hors premier weekend du mois,
**When** la commission est calculée,
**Then** le taux standard est appliqué sur le montant total
**And** `commission.rate_applied` et `commission.amount_in_cents` sont stockés avec la commande

**Given** une commande confirmée durant le premier weekend du mois (samedi ou dimanche),
**When** la commission est calculée,
**Then** le taux réduit Weekend Maker est appliqué
**And** `commission.is_weekend_maker = true` est tracé sur la commande

**Given** `GET /api/v1/orders/:id` avec un JWT maker propriétaire,
**When** la commande est confirmée,
**Then** la réponse inclut `{ totalInCents, commissionInCents, commissionRate, isWeekendMaker, netAmountInCents }`

---

### Story 6.5 — Emails de Confirmation & Notification Maker (FR29, FR30)

En tant qu'acheteur et maker,
Je veux recevoir les confirmations de commande par email et notification temps réel,
Afin d'être informé immédiatement de chaque transaction.

**Critères d'Acceptation :**

**Given** une commande passée au statut `Confirmed`,
**When** le consumer `order-infra` traite l'événement `PaymentConfirmed`,
**Then** un email de confirmation est envoyé à l'acheteur via Resend avec `{ orderRef, items[], totalInCents }`
**And** l'email est délivré en moins de 30 secondes

**Given** le même événement `PaymentConfirmed`,
**When** il est traité,
**Then** un email récapitulatif est envoyé au maker avec `{ orderRef, buyerFirstName, items[], netAmountInCents }`
**And** un événement SSE `order.new_sale` est émis vers le stream du maker connecté

**Given** un maker connecté sur son dashboard,
**When** une nouvelle vente se produit,
**Then** une notification toast "Nouvelle vente !" apparaît en temps réel via SSE
**And** le tableau des commandes se met à jour sans rechargement de page

**Given** une défaillance temporaire de Resend,
**When** l'envoi email échoue,
**Then** une retry est effectuée (3 tentatives max avec backoff exponentiel)
**And** l'échec est loggé via `tracing` avec le niveau `ERROR`

---

### Story 6.6 — Dashboard Maker : Commandes Reçues

En tant que maker,
Je veux consulter mes commandes reçues depuis mon dashboard,
Afin de suivre mes ventes et préparer les expéditions.

**Critères d'Acceptation :**

**Given** `GET /api/v1/orders?role=maker` avec un JWT maker,
**When** la requête est traitée,
**Then** la réponse est `200 OK` avec la liste paginée `{ orderId, orderRef, buyerName, items[], totalInCents, netAmountInCents, status, createdAt }`

**Given** la page `/dashboard/orders` en SSR,
**When** le maker la consulte,
**Then** ses commandes sont listées par date décroissante avec statut, montant net et référence

**Given** un maker sans commandes,
**When** la page est chargée,
**Then** un état vide "Aucune commande pour l'instant" est affiché

## Epic 7 — Favoris, Notifications & Communication

Les acheteurs reçoivent des alertes sur leurs favoris, les makers sont notifiés en temps réel et peuvent gérer les expéditions.
**FRs couverts :** FR22, FR23, FR31, FR32, FR33

### Story 7.1 — Domaine Catalog : Favorite & Alerte Disponibilité (TDD)

En tant que développeur,
Je veux le modèle `Favorite` implémenté en TDD dans le domaine Catalog,
Afin que la logique d'alerte de disponibilité soit vérifiable sans infra.

**Critères d'Acceptation :**

**Given** la crate `catalog-domain`,
**When** `favorite.create(buyer_id, product_id)` est appelé deux fois avec les mêmes paramètres,
**Then** le domaine retourne `DomainError::Conflict("already in favorites")`

**Given** un événement `ProductRestocked { product_id }` traité par le service d'alerte,
**When** des favoris existent pour ce produit,
**Then** un événement `FavoriteAlertTriggered { buyer_ids: [...], product_id }` est émis
**And** les tests couvrent le cas où aucun favori n'existe (pas d'événement émis)

---

### Story 7.2 — Favoris Acheteur (FR22)

En tant qu'acheteur,
Je veux ajouter et retirer des produits de ma liste de favoris,
Afin de retrouver facilement les créations qui m'intéressent.

**Critères d'Acceptation :**

**Given** `POST /api/v1/favorites` avec `{ productId }` et un JWT acheteur,
**When** le produit existe et n'est pas déjà en favoris,
**Then** le favori est créé en base dans le schéma `catalog`
**And** la réponse est `201 Created` avec `{ favoriteId, productId }`

**Given** `POST /api/v1/favorites` avec un produit déjà en favoris,
**When** la requête est traitée,
**Then** la réponse est `409 Conflict` au format RFC 7807

**Given** `DELETE /api/v1/favorites/:productId` avec un JWT acheteur,
**When** le favori appartient à l'acheteur,
**Then** le favori est supprimé
**And** la réponse est `204 No Content`

**Given** `GET /api/v1/favorites` avec un JWT acheteur,
**When** la requête est traitée,
**Then** la réponse est `200 OK` avec la liste `{ items: [{ productId, title, priceInCents, status, photoUrl }] }`

**Given** la fiche produit `/products/[id]`,
**When** un acheteur connecté la consulte,
**Then** un bouton cœur indique si le produit est en favoris
**And** un clic ajoute ou retire le produit sans rechargement de page

---

### Story 7.3 — Alerte Disponibilité Favori (FR23)

En tant qu'acheteur,
Je veux recevoir une alerte email quand un produit de mes favoris redevient disponible,
Afin de ne pas manquer un réapprovisionnement.

**Critères d'Acceptation :**

**Given** le consumer `catalog-infra` traitant l'événement `ProductRestocked`,
**When** des acheteurs ont ce produit en favoris,
**Then** un email "Le produit X est de nouveau disponible" est envoyé à chaque acheteur concerné via Resend
**And** l'email contient un lien direct vers la fiche produit

**Given** un produit repassant en `Published` sans favoris enregistrés,
**When** l'événement `ProductRestocked` est traité,
**Then** aucun email n'est envoyé (pas d'erreur)

**Given** `GET /api/v1/favorites` après qu'un produit favori soit repassé en `Published`,
**When** l'acheteur consulte ses favoris,
**Then** le produit affiche le statut `Published` mis à jour

---

### Story 7.4 — Infrastructure SSE & Notification Rupture Stock Maker (FR31)

En tant que maker,
Je veux recevoir une notification en temps réel quand un de mes produits est en rupture de stock,
Afin de réagir immédiatement et réapprovisionner si nécessaire.

**Critères d'Acceptation :**

**Given** `GET /api/v1/events` avec un JWT valide,
**When** la connexion SSE est établie,
**Then** le serveur maintient le stream ouvert avec des heartbeats toutes les 30 secondes
**And** la réponse a le Content-Type `text/event-stream`

**Given** un produit maker passant au statut `OutOfStock`,
**When** l'événement `ProductOutOfStock` est publié sur RabbitMQ,
**Then** le consumer `app-server/messaging.rs` l'émet dans le stream SSE du maker connecté
**And** le format SSE est `data: { "type": "stock.depleted", "productId": "...", "title": "..." }`

**Given** le composant `SseListener` dans Next.js,
**When** un événement `stock.depleted` est reçu,
**Then** une notification toast "Rupture de stock : [titre produit]" apparaît sur le dashboard
**And** la liste des produits se met à jour visuellement sans rechargement

**Given** un maker non connecté au SSE,
**When** son produit passe en rupture,
**Then** l'événement est ignoré silencieusement — il verra l'état au prochain chargement

---

### Story 7.5 — Email de Relance Panier Abandonné (FR32)

En tant que plateforme,
Je veux envoyer automatiquement un email de relance aux acheteurs ayant abandonné leur panier,
Afin de récupérer des ventes manquées et aider les makers.

**Critères d'Acceptation :**

**Given** un panier non vide dont le dernier update est > 24h sans commande associée,
**When** le job de détection s'exécute (toutes les heures dans `app-server`),
**Then** l'acheteur reçoit un email de relance via Resend avec les items et un lien vers `/cart`
**And** le panier est marqué `abandoned_email_sent = true` pour éviter les doublons

**Given** un acheteur ayant passé commande depuis l'abandon,
**When** le job s'exécute,
**Then** aucun email de relance n'est envoyé

**Given** un panier anonyme (non connecté),
**When** le job s'exécute,
**Then** aucun email n'est envoyé (pas d'email connu)

**Given** l'email de relance reçu par l'acheteur,
**When** il clique sur le lien "Reprendre mon panier",
**Then** il est redirigé vers `/cart` avec ses items toujours présents

---

### Story 7.6 — Infos Expédition Commande (FR33)

En tant que maker,
Je veux renseigner les informations d'expédition d'une commande,
Afin de tenir mon acheteur informé de l'avancement de sa livraison.

**Critères d'Acceptation :**

**Given** `PATCH /api/v1/orders/:id/shipping` avec `{ trackingNumber, carrier }` et un JWT maker propriétaire,
**When** la commande est en statut `Confirmed`,
**Then** les informations d'expédition sont sauvegardées
**And** le statut de la commande passe à `Shipped`
**And** la réponse est `200 OK`

**Given** `PATCH /api/v1/orders/:id/shipping` sur une commande non `Confirmed`,
**When** la requête est traitée,
**Then** la réponse est `409 Conflict` au format RFC 7807

**Given** la page `/dashboard/orders`,
**When** le maker clique "Marquer comme expédié" sur une commande confirmée,
**Then** un formulaire inline apparaît pour saisir numéro de suivi et transporteur
**And** après soumission, la commande affiche le statut "Expédiée" avec les infos de suivi

## Epic 8 — Conformité RGPD & Réglementation

La plateforme respecte les obligations légales (RGPD, droit de rétractation, PCI-DSS).
**FRs couverts :** FR34, FR35, FR36

### Story 8.1 — Politique de Confidentialité & Droit de Rétractation (FR34, FR36)

En tant qu'utilisateur,
Je veux consulter la politique de confidentialité et voir clairement le droit de rétractation lors de mes achats,
Afin d'être informé de mes droits et de l'utilisation de mes données.

**Critères d'Acceptation :**

**Given** la page `/privacy` accessible à tous (SSG),
**When** un visiteur la consulte,
**Then** la politique de confidentialité RGPD est affichée : données collectées, finalités, durée de conservation, droits (accès, rectification, suppression)
**And** un lien vers `/privacy` est présent dans le footer de toutes les pages

**Given** la page `/checkout` lors du récapitulatif de commande,
**When** l'acheteur est sur le point de confirmer,
**Then** le délai de rétractation de 14 jours est affiché de manière visible et non masquable
**And** une case à cocher "J'ai pris connaissance du droit de rétractation" est requise avant de pouvoir payer

**Given** la case à cocher de rétractation non cochée,
**When** l'acheteur tente de soumettre le paiement,
**Then** le formulaire bloque la soumission avec un message explicite

---

### Story 8.2 — Droit à l'Oubli : Suppression des Données Personnelles (FR35)

En tant qu'utilisateur,
Je veux pouvoir demander la suppression de toutes mes données personnelles,
Afin d'exercer mon droit à l'oubli conformément au RGPD.

**Critères d'Acceptation :**

**Given** `DELETE /api/v1/users/me` avec un JWT valide,
**When** la requête est traitée,
**Then** les données personnelles identifiantes sont anonymisées dans tous les schémas (`identity`, `order`)
**And** email, nom et biographie sont remplacés par `[supprimé]`
**And** l'historique des commandes est conservé avec les montants (obligation comptable) mais sans données personnelles
**And** les cookies JWT sont supprimés et le compte ne peut plus se connecter
**And** la réponse est `200 OK`

**Given** un maker avec des produits publiés demandant la suppression,
**When** `DELETE /api/v1/users/me` est appelé,
**Then** ses produits passent automatiquement en statut `Paused`
**And** sa boutique est marquée `inactive`

**Given** la page `/dashboard/settings`,
**When** un utilisateur connecté clique "Supprimer mon compte",
**Then** une modale de confirmation avec avertissement d'irréversibilité apparaît
**And** une saisie de confirmation (`"SUPPRIMER"`) est requise avant de procéder

**Given** la suppression confirmée,
**When** la page se recharge,
**Then** l'utilisateur est redirigé vers la page d'accueil non connecté
