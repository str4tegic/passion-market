---
stepsCompleted: ["step-01-init", "step-02-discovery", "step-02b-vision", "step-02c-executive-summary", "step-03-success", "step-04-journeys", "step-05-domain", "step-06-innovation", "step-07-project-type", "step-08-scoping", "step-09-functional", "step-10-nonfunctional", "step-11-polish", "step-12-complete"]
completedAt: "2026-03-28"
inputDocuments: ["docs/catalog-domain.md"]
documentCounts:
  briefCount: 0
  researchCount: 0
  brainstormingCount: 0
  projectDocsCount: 1
workflowType: 'prd'
classification:
  projectType: web_app_marketplace
  domain: e-commerce / passion economy
  complexity: low-medium
  projectContext: brownfield
  mvpScope: true
  differentiator: présentation maker soignée (storytelling, vidéo fabrication, identité artisanale)
---

# Product Requirements Document - passion-market

**Auteur :** Francois
**Date :** 2026-03-28

## Résumé Exécutif

passion-market est une plateforme e-commerce MVP dédiée aux makers passionnés — artisans, créateurs et indépendants qui fabriquent eux-mêmes leurs produits. Elle répond à un double problème : les makers choisissent aujourd'hui entre visibilité zéro (grandes plateformes) ou complexité technique et financière (site personnel avec SEO, maintenance, e-réputation). Les acheteurs, eux, sont épuisés par l'anonymat du dropshipping et cherchent à acheter à de vraies personnes.

passion-market supprime la barrière à l'entrée maker : aucun site à créer, aucune maintenance, aucune gestion de la visibilité. Le maker arrive, présente sa passion, et vend. Il conserve la maîtrise de l'expédition et du SAV. L'acheteur sait exactement à qui il achète — un passionné identifié avec son histoire et son processus de fabrication.

### Ce qui rend passion-market unique

Le différenciateur central du MVP est la **présentation du maker** : l'humain et sa passion au premier plan, là où toutes les autres plateformes mettent les produits. Storytelling personnel, vidéo de fabrication, identité artisanale — l'acheteur découvre qui lui vend avant même de regarder le produit.

Cette approche est anti-industrielle par nature : il n'y a pas de question d'échelle. Un maker qui produit 10 objets par mois a autant sa place qu'un autre. passion-market est une plateforme d'émancipation — liberté par le travail, reconnaissance par la création.

**Positionnement :** anti-dropshipping, zéro friction maker, authenticité garantie acheteur. Pas d'avis/notes en MVP — la confiance passe par l'histoire du maker, pas par un algorithme de réputation.

## Classification du Projet

- **Type :** Web App — Marketplace e-commerce (maker-centric, C2C light)
- **Domaine :** E-commerce / Passion economy / Artisanat
- **Complexité :** Faible à Moyenne (MVP volontairement réduit)
- **Contexte :** Brownfield — Bounded Context Catalog existant (Rust/DDD), autres BCs à construire

## Critères de Succès

### Succès Utilisateur

**Maker :**
- Configure sa boutique et met un premier produit en vente en moins de 15 minutes
- Comprend immédiatement le modèle : zéro coût fixe, commission uniquement à la vente
- Aucune action technique requise après l'onboarding

**Acheteur :**
- Perçoit l'authenticité des produits dès la page maker
- Finalise un achat en sachant exactement qui fabrique ce qu'il achète
- Revient ou recommande la plateforme suite à la qualité de l'expérience de découverte

### Succès Business

**À 3 mois :** ≥ 5 makers actifs, ≥ 10 ventes réalisées.

**À 12 mois :** Communauté en croissance visible, mentions spontanées sur les réseaux sociaux, bouche-à-oreille établi.

**Modèle économique :** Commission à la vente uniquement — pas d'abonnement, pas de coût fixe. La plateforme ne gagne que si le maker gagne.

**Mécanique "Weekend Maker" :** Commission réduite le premier weekend de chaque mois — rendez-vous régulier qui crée un rythme communautaire et stimule l'activité.

### Succès Technique

- Architecture DDD correctement appliquée, chaque Bounded Context isolé et cohérent
- Couverture de tests domaine Rust ≥ 80% (TDD)
- Code Rust idiomatique, évolutif, prêt pour BCs additionnels post-MVP
- Paiement Stripe intégré dès le MVP

### Résultats Mesurables

| Indicateur | Cible MVP (3 mois) |
|---|---|
| Makers avec boutique active | ≥ 5 |
| Ventes réalisées | ≥ 10 |
| Temps d'onboarding maker | ≤ 15 min |
| Couverture tests domaine | ≥ 80% |

## Périmètre Produit & Stratégie

### Philosophie MVP

**Approche :** Experience MVP — valider que le maker-first crée une confiance acheteur absente des autres plateformes. Succès mesuré à la qualité de l'expérience, pas au volume.

**Ressources :** 1 développeur solo + IA (Claude Code).

**Ordre de développement :**
1. Domaine Rust (BCs : Catalog déjà avancé, puis Order, Identity, Payment)
2. API Rust (axum/actix-web, endpoints par BC)
3. Front-end Next.js (SSR, pages maker, tunnel achat)

### MVP — Phase 1

- Inscription & profil maker (storytelling, photo atelier, vidéo fabrication optionnelle)
- Validation manuelle maker par l'admin
- Création boutique, gestion produits (draft → publié), gestion stock
- Page profil maker publique SEO-optimisée (SSR)
- Catalogue & fiche produit (SSR/ISR)
- Panier persistant + tunnel de paiement Stripe
- Commission à la vente + mécanique "Weekend Maker"
- Emails transactionnels (confirmation commande, validation boutique, relance panier abandonné)
- Notifications SSE maker (vente, rupture stock) + alerte acheteur "produit de nouveau disponible"
- Dashboard admin : file de validation makers
- RGPD minimal + affichage droit de rétractation 14 jours

**Hors périmètre MVP :** avis/notes, messagerie intégrée, logistique déléguée, SAV délégué, programme de fidélité.

### Growth — Phase 2

- Système d'avis et de notation acheteurs
- Messagerie maker ↔ acheteur
- Gestion stock & expédition déléguées à la plateforme
- Gestion SAV déléguée à la plateforme
- Mode découverte vidéo (style TikTok) — fil de vidéos de fabrication comme surface de découverte produit
- Scoring maker + alertes automatiques (remplace validation 100% manuelle)
- Moteur de recherche & filtres avancés
- Programme de certification maker (label "Certified" déjà modélisé dans Catalog)
- Dashboard analytics makers

### Vision — Phase 3

- Application mobile native
- Communauté & réseau entre makers
- Événements virtuels / marchés en ligne
- Internationalisation

### Mitigation des Risques

**Techniques :** Courbe Rust/DDD/TDD atténuée par l'IA et l'ordre de dev progressif. Contrat d'API Rust → Next.js défini dès les premiers BCs.

**Marché (oeuf/poule) :** Bootstrap makers via démarchage direct et réseaux sociaux. Bootstrap acheteurs via SEO organique + partage des makers sur leurs propres réseaux. Avantage : chaque maker est un canal d'acquisition.

**Ressources :** Si une fonctionnalité dépasse le budget temps, relance panier et alertes "de nouveau disponible" peuvent glisser en Phase 2 sans bloquer le lancement.

## Parcours Utilisateurs

### Parcours 1 — Thomas, le Maker (chemin réussi)

Thomas a 47 ans. Il est technicien de maintenance le jour, menuisier le soir dans son garage. Depuis deux ans, il offre ses créations à sa famille. Il a regardé Etsy — l'interface l'a découragé. Il a pensé à un site WordPress — les tarifs d'hébergement l'ont stoppé.

Un mardi soir, il tombe sur passion-market. En dix minutes, il crée son compte, écrit trois lignes sur sa passion pour le bois, uploade une photo de son atelier et une vidéo de ponçage. Il crée sa première fiche produit : étagère en chêne massif, 89€. Il lit "0€ tant que tu ne vends pas." Il comprend immédiatement. Il publie.

Le vendredi du premier weekend du mois, notification : commission réduite. Il baisse son prix de 5€. Le samedi matin, première vente. Il emballe, expédie via Colissimo. Il reçoit un message de l'acheteur : "voir ta vidéo m'a convaincu." Thomas relit le message trois fois.

**Capacités révélées :** onboarding maker, création boutique & produit, commission, mécanique weekend, notifications.

---

### Parcours 2 — Thomas, le Maker (rupture de stock)

Après le weekend, Thomas a vendu ses 3 étagères. Le produit passe automatiquement en rupture. Trois semaines plus tard, il réapprovisionne : +2 unités. Le produit repasse en ligne automatiquement. Une acheteur qui l'avait mis en favori reçoit l'alerte "de nouveau disponible". Elle achète.

**Capacités révélées :** gestion stock automatique (OutOfStock → Published), alerte acheteur sur disponibilité.

---

### Parcours 3 — Sophie, l'Acheteur (chemin réussi)

Sophie, 41 ans, refait son salon. Elle cherche une console en bois brut — pas du IKEA, quelque chose d'unique. Etsy ne lui inspire pas confiance : trop de produits venant d'Asie, elle ne sait pas qui fabrique quoi.

Une amie lui partage un lien vers la page de Thomas. Sophie lit son histoire en 30 secondes — technicien le jour, menuisier passionné. Elle voit la photo de l'atelier, regarde la vidéo. Elle voit ses mains travailler le bois. Elle n'a pas encore regardé les prix. Elle fait confiance avant même de voir le produit.

Elle commande et paie en ligne. Elle reçoit l'email de confirmation. Trois jours plus tard, elle poste une photo sur Instagram avec le tag passion-market.

**Capacités révélées :** page profil maker (storytelling, vidéo, atelier), fiche produit, tunnel paiement, email confirmation, partage social organique.

---

### Parcours 4 — Sophie, l'Acheteur (abandon de panier)

Sophie cherche un cadeau de mariage entre 60 et 120€. Elle browse plusieurs makers, ajoute deux produits au panier, hésite, quitte le site. Le lendemain, email de relance panier abandonné. Elle revient, relit le profil du maker, la vidéo la convainc. Elle finalise l'achat.

**Capacités révélées :** panier persistant, email relance abandon, profil maker comme levier de conversion finale.

---

### Parcours 5 — Francois, l'Admin (validation maker)

Julie, 29 ans, créatrice de céramiques, s'inscrit. Profil rempli, photos, courte vidéo. Francois reçoit la notification de validation en attente. Il valide en un clic. Julie reçoit l'email d'activation.

Le lendemain, un deuxième compte : profil vide, description générique. Francois refuse avec motif. Le compte reçoit l'email de refus avec instructions pour corriger.

**Capacités révélées :** dashboard admin (file de validation), aperçu profil, approuver/refuser avec motif, emails automatiques.

---

### Résumé des Capacités par Parcours

| Capacité | Parcours |
|---|---|
| Onboarding maker (profil, vidéo, atelier) | 1, 5 |
| Création boutique & produits | 1 |
| Gestion stock + statuts automatiques | 2 |
| Alerte "de nouveau disponible" | 2 |
| Page profil maker (storytelling, vidéo) | 1, 3, 4 |
| Tunnel d'achat + paiement intégré | 3, 4 |
| Mécanique Weekend Maker | 1 |
| Panier persistant + relance abandon | 4 |
| Emails transactionnels | 3, 4 |
| Dashboard admin + validation maker | 5 |
| Notifications SSE maker | 1, 2 |

## Exigences Domaine

### Conformité & Réglementation

- **RGPD :** collecte minimale de données personnelles, consentement explicite, politique de confidentialité obligatoire, droit à l'oubli implémenté
- **Droit de rétractation :** affichage obligatoire du délai de 14 jours (directive UE e-commerce) lors de tout achat
- **PCI-DSS :** aucune donnée de carte bancaire stockée sur les serveurs passion-market — délégation totale à Stripe ; la plateforme ne manipule que des tokens de paiement

### Sécurité des Données

- Adresses de livraison des acheteurs accessibles uniquement au maker de la commande concernée
- Aucune donnée de carte bancaire en base de données passion-market

### Risques et Mitigations

- **Faux makers / contenu frauduleux :** validation manuelle à l'inscription (MVP) ; scoring automatique en post-MVP
- **Litiges paiement :** Stripe gère les chargebacks ; process d'escalade maker → admin documenté

## Innovation & Positionnement Concurrentiel

### Axes d'Innovation

**1. Maker-first UX (inversion du paradigme marketplace)**
Les marketplaces traditionnelles sont product-first : le produit en avant, le vendeur comme métadonnée. passion-market inverse ce paradigme : le maker et son histoire au centre, le produit comme conséquence de sa passion.

**2. Vidéo de fabrication comme surface de découverte (post-MVP)**
Transformer les vidéos de fabrication en fil de découverte (style TikTok) crée une interaction inédite en e-commerce : on achète ce qu'on a vu naître.

**3. Rythme communautaire via événement récurrent**
Le "Weekend Maker" introduit un rythme de marché physique dans le digital — un rendez-vous attendu, pas une promotion permanente qui banalise les prix.

### Positionnement Concurrentiel

| Plateforme | Problème |
|---|---|
| Etsy | Product-first, saturé, maker noyé dans la masse, frais complexes |
| Amazon Handmade | Artisanat industrialisé — paradoxe de la plateforme |
| Instagram/TikTok Shop | Discovery forte, pas de marketplace structurée |
| **passion-market** | **Maker-first + zéro friction entrée + modèle anti-risque** |

### Validation des Hypothèses

- **MVP valide :** la page profil maker convertit-elle mieux qu'une fiche produit standard ? Métrique : taux de conversion profil → achat vs. catalogue → achat.
- **Post-MVP valide :** le fil vidéo génère-t-il plus d'engagement qu'un catalogue classique ?
- **Fallback :** si maker-first ne convertit pas, la fiche produit reste fonctionnelle. Si le weekend ne crée pas de rendez-vous, la commission réduite peut devenir permanente.

## Exigences Web App

### Architecture & Rendu

- **Front-end :** Next.js (SSR + ISR selon les pages)
  - Pages ISR : catalogue, profil maker, fiche produit (SEO-critical, revalidation périodique)
  - Pages SSR : panier, commande, dashboard maker, admin
- **Back-end :** API Rust (axum ou actix-web), découpage par Bounded Context
- **Événementiel :** Server-Sent Events (SSE) — les domain events Rust streamés vers Next.js ; WebSocket hors périmètre MVP (unidirectionnel suffisant)

### Compatibilité Navigateurs & Appareils

| Cible | Support |
|---|---|
| Chrome / Edge (dernières 2 versions) | ✅ |
| Firefox (dernières 2 versions) | ✅ |
| Safari mobile (iOS) | ✅ |
| Chrome mobile (Android) | ✅ |
| IE / navigateurs anciens | ❌ Hors périmètre |

Design responsive mobile-first. Application mobile native : Phase 3.

### Stratégie SEO

- Pages maker et fiches produit : SSR/ISR avec métadonnées complètes (Open Graph, JSON-LD Schema.org `Product` + `Person`)
- URLs sémantiques : `/makers/[slug]`, `/produits/[slug]`
- Sitemap généré dynamiquement
- Social sharing : cards riches (Twitter/X, Facebook, WhatsApp) depuis les pages maker

### Notifications Temps Réel (SSE)

Endpoint SSE consommant les domain events Rust :
- Maker : nouvelle vente, rupture de stock, validation boutique
- Acheteur : statut commande mis à jour, produit favori de nouveau disponible
- Aligné avec `CatalogEvent` / `OrderEvent` modélisés en DDD

## Exigences Fonctionnelles

### Gestion des Comptes & Identité

- **FR1 :** Un visiteur peut créer un compte maker avec email et mot de passe
- **FR2 :** Un visiteur peut créer un compte acheteur avec email et mot de passe
- **FR3 :** Un utilisateur peut se connecter et se déconnecter
- **FR4 :** Un utilisateur peut réinitialiser son mot de passe par email
- **FR5 :** Un maker peut renseigner son profil (nom, biographie, passion, photo atelier, vidéo de fabrication)
- **FR6 :** Un maker peut modifier son profil à tout moment

### Validation & Administration

- **FR7 :** L'admin peut consulter la file des comptes makers en attente de validation
- **FR8 :** L'admin peut approuver un compte maker avec activation de la boutique
- **FR9 :** L'admin peut refuser un compte maker avec motif communiqué par email
- **FR10 :** Le système notifie le maker par email du résultat de la validation

### Boutique & Catalogue

- **FR11 :** Un maker validé peut créer une boutique avec nom et catégorie
- **FR12 :** Un maker peut créer un produit en brouillon (nom, description, prix, stock, photos)
- **FR13 :** Un maker peut attacher une vidéo de fabrication à un produit
- **FR14 :** Un maker peut publier un produit (stock > 0 requis)
- **FR15 :** Un maker peut mettre un produit en pause ou le supprimer
- **FR16 :** Un maker peut mettre à jour le stock d'un produit
- **FR17 :** Le système passe automatiquement un produit en rupture quand le stock atteint zéro
- **FR18 :** Le système repasse automatiquement un produit en ligne quand le stock est réapprovisionné

### Découverte & Consultation

- **FR19 :** Un visiteur peut consulter la page publique d'un maker (profil, histoire, vidéo, boutique)
- **FR20 :** Un visiteur peut consulter le catalogue de produits d'une boutique
- **FR21 :** Un visiteur peut consulter la fiche détaillée d'un produit
- **FR22 :** Un acheteur peut ajouter un produit à sa liste de favoris
- **FR23 :** Un acheteur peut recevoir une alerte quand un produit favori redevient disponible

### Tunnel d'Achat & Paiement

- **FR24 :** Un visiteur peut ajouter des produits à un panier
- **FR25 :** Le panier est persistant entre les sessions
- **FR26 :** Un acheteur peut finaliser une commande avec paiement en ligne (Stripe)
- **FR27 :** Le système calcule la commission plateforme sur chaque vente
- **FR28 :** Le système applique une commission réduite le premier weekend de chaque mois
- **FR29 :** L'acheteur reçoit un email de confirmation de commande
- **FR30 :** Le maker reçoit une notification SSE en temps réel et un email récapitulatif lors de chaque nouvelle vente

### Notifications & Communication

- **FR31 :** Un maker reçoit une notification SSE en temps réel lors d'une rupture de stock
- **FR32 :** Le système envoie un email de relance à un acheteur ayant abandonné son panier
- **FR33 :** Le maker peut renseigner les informations d'expédition d'une commande (numéro de suivi, transporteur)

### Conformité & Données

- **FR34 :** Un utilisateur peut consulter la politique de confidentialité (RGPD)
- **FR35 :** Un utilisateur peut demander la suppression de ses données personnelles
- **FR36 :** Les informations sur le droit de rétractation (14 jours) sont affichées lors de tout achat
- **FR37 :** Le système ne stocke aucune donnée de carte bancaire (délégation Stripe)

## Exigences Non-Fonctionnelles

### Performance

- Pages maker et fiches produit (SSR/ISR) : affichage < 2 secondes sur connexion 4G/fibre standard
- Core Web Vitals : LCP < 2.5s, CLS < 0.1, FID < 100ms
- Actions critiques (ajout panier, paiement) : réponse < 3 secondes
- Flux SSE : ne bloque pas le chargement de la page

### Sécurité

- Toutes les communications chiffrées via HTTPS (TLS 1.2 minimum)
- Mots de passe hashés avec bcrypt ou argon2
- Aucune donnée de carte bancaire stockée (délégation Stripe, conformité PCI-DSS)
- Adresses de livraison accessibles uniquement au maker de la commande concernée
- Tokens d'authentification avec expiration sur inactivité configurable
- Endpoints admin distincts et protégés des endpoints publics

### Scalabilité

- Architecture DDD par BC supporte une montée en charge progressive sans refactoring majeur
- Cible MVP : < 100 makers, < 1 000 acheteurs — pas de contrainte de scalabilité horizontale immédiate
- Chaque BC peut être déployé et scalé indépendamment en post-MVP

### Accessibilité

- Conformité WCAG 2.1 niveau AA sur toutes les pages publiques
- Navigation clavier complète sans piège de focus
- Compatible NVDA, VoiceOver, TalkBack
- Ratio de contraste ≥ 4.5:1 (texte courant), ≥ 3:1 (grands textes)
- Textes alternatifs sur toutes les images
- Vidéos de fabrication : transcription ou sous-titres disponibles

### Intégrations

- **Stripe :** webhooks côté serveur pour confirmer les paiements avant validation de commande ; aucun stockage de données carte
- **Email transactionnel** (Resend, SendGrid ou équivalent) : fiabilité ≥ 99% sur emails critiques (confirmation commande, validation maker)
- **Stockage médias** (S3 ou équivalent) : photos et vidéos maker hors base de données, diffusion via CDN

### Maintenabilité

- Couverture de tests domaine Rust ≥ 80% (TDD — les tests documentent le comportement métier)
- Chaque Bounded Context indépendant : changement dans un BC sans impact sur les autres
- Contrat d'API versionné dès le début pour garantir la compatibilité avec le front Next.js
