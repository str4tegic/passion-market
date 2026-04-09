
## Deferred from: code review story 2-2-inscription-maker (2026-04-09)

- **Race condition TOCTOU email** — check `find_by_email` + `save` sans transaction. La contrainte UNIQUE DB est le vrai garde-fou. À traiter avec une story dédiée transactions. [`identity-application/src/specifications.rs`, `use_cases.rs`]
- **Champ `name` reçu en API mais ignoré** — `RegisterMakerRequest.name` non forwarded au domaine. À traiter quand `name` intégrera le domaine User. [`identity-api/src/handlers/register_maker.rs`]
- **Pas de validation format email** — Aucune validation regex côté API ni domaine. Décision à prendre en story 2.3 ou story dédiée. [`identity-api/src/handlers/register_maker.rs`]
- **Mot de passe plaintext non zéroïsé** — `body.password` reste en mémoire heap sans zeroize. Hardening sécurité hors scope. [`identity-api/src/handlers/register_maker.rs`]
- **RFC 7807 : `instance` absent, `type` = "about:blank"** — Acceptable pour le moment ; à enrichir quand la doc API sera formalisée. [`identity-api/src/errors.rs`]
- **`UserRegistered` event ignoré** — Handler ignore l'event retourné par le use case. À connecter au bus RabbitMQ dans une story dédiée. [`identity-api/src/handlers/register_maker.rs`]
- **Pas de transaction wrapping check+save** — Lié au TOCTOU. Même story infra. [`identity-application/src/use_cases.rs`]
- **Message d'erreur Argon2 générique** — Hash failure retourne "Failed to hash password" sans cause. Non bloquant pour les users. [`identity-infra/src/argon2_hasher.rs`]

## Deferred from: code review de la story 1-1-workspace-cargo-et-environnement-docker-compose (2026-03-29)

- **EventEnvelope.version toujours 1 hardcodé** — Pas de stratégie de versioning d'events. À adresser quand le bus d'events sera implémenté (Story 6+). [`crates/shared-kernel/src/events.rs:32`]
- **Newtype IDs sans From<Uuid> ni Display** — Utilisabilité limitée. À compléter lors de l'implémentation domaine (Stories 2+). [`crates/shared-kernel/src/ids.rs`]
- **JWT_ACCESS_TTL_SECONDS / JWT_REFRESH_TTL_SECONDS absents de AppConfig** — À ajouter en Story 2 (authentification). [`crates/app-server/src/config.rs`]
- **depends_on entre services Docker absent** — À ajouter quand app-server sera déployé via Docker Compose.
