
## Deferred from: code review de la story 1-1-workspace-cargo-et-environnement-docker-compose (2026-03-29)

- **EventEnvelope.version toujours 1 hardcodé** — Pas de stratégie de versioning d'events. À adresser quand le bus d'events sera implémenté (Story 6+). [`crates/shared-kernel/src/events.rs:32`]
- **Newtype IDs sans From<Uuid> ni Display** — Utilisabilité limitée. À compléter lors de l'implémentation domaine (Stories 2+). [`crates/shared-kernel/src/ids.rs`]
- **JWT_ACCESS_TTL_SECONDS / JWT_REFRESH_TTL_SECONDS absents de AppConfig** — À ajouter en Story 2 (authentification). [`crates/app-server/src/config.rs`]
- **depends_on entre services Docker absent** — À ajouter quand app-server sera déployé via Docker Compose.
