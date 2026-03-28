# Bounded Context : Catalog

Ce document décrit le modèle du domaine Catalog de passion-market.
Il explique chaque choix de conception et les termes Rust utilisés.

---

## Pourquoi ce Bounded Context ?

Le BC Catalog porte la promesse fondamentale de la plateforme :
**un produit sur passion-market est forcément créé par un maker identifié, jamais importé d'un catalogue dropshipping.**

C'est ici que vivent les règles métier autour des boutiques et des produits.

---

## Concepts Rust utilisés dans ce document

Avant de lire la suite, voici un lexique rapide pour un développeur PHP.

### `struct` — l'équivalent d'une classe sans héritage

```rust
pub struct Product {
    id: ProductId,
    name: ProductName,
    price: Price,
}
```

En PHP tu écrirais `class Product { ... }`. En Rust, une `struct` contient les données.
Les méthodes sont définies séparément dans un bloc `impl`.

```rust
impl Product {
    pub fn publish(&mut self) -> Result<(), CatalogError> {
        // ...
    }
}
```

> Rust n'a pas d'héritage. On compose des types, on ne les étend pas.

---

### `enum` — bien plus puissant qu'en PHP

En PHP, un enum est une liste de constantes. En Rust, chaque variante peut porter des données :

```rust
pub enum MakerLabel {
    Uncertified,
    Certified { since: Date, review_count: u32 },
}
```

`Uncertified` ne porte rien. `Certified` porte une date et un compteur.
C'est comme une union typée — impossible d'avoir un label `Certified` sans sa date.

---

### `Option<T>` — le remplacement de `null`

En PHP, une valeur peut être `null`. En Rust, `null` n'existe pas.
À la place, on utilise `Option<T>` :

```rust
pub fabrication_video: Option<FabricationVideoUrl>,
```

- `Some(url)` → il y a une vidéo
- `None`      → pas de vidéo

Le compilateur t'oblige à gérer les deux cas. Plus de `null pointer exception`.

---

### `Result<T, E>` — le remplacement des exceptions

En PHP tu lèves une exception avec `throw`. En Rust, les fonctions retournent un `Result` :

```rust
pub fn publish(&mut self) -> Result<(), CatalogError> {
    if self.stock.is_depleted() {
        return Err(CatalogError::CannotPublishWithoutStock);
    }
    // ...
    Ok(())
}
```

- `Ok(valeur)` → succès
- `Err(erreur)` → échec

`()` (prononcé "unit") signifie "aucune valeur de retour en cas de succès" — l'équivalent de `void`.
L'appelant est forcé par le compilateur à gérer le cas d'erreur.

---

### `&self` et `&mut self` — accès à l'objet courant

En PHP, `$this` est toujours modifiable. En Rust, Rust distingue :

- `&self`     → lecture seule (comme `$this` dans un getter)
- `&mut self` → lecture + écriture (comme `$this` dans un setter ou une méthode qui modifie l'état)

Si une méthode prend `&self`, le compilateur garantit qu'elle ne modifie rien.

---

### `trait` — l'équivalent d'une interface PHP

```rust
pub trait ProductRepository: Send + Sync {
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, CatalogError>;
    async fn save(&self, product: &Product) -> Result<(), CatalogError>;
}
```

En PHP : `interface ProductRepository { ... }`.
La couche infrastructure implémente ce trait avec Postgres. Le domaine ne connaît pas Postgres.

`Send + Sync` sont des marqueurs techniques qui disent "ce type peut être utilisé en contexte async multi-thread". À ne pas y penser pour l'instant.

---

### `async fn` / `await` — asynchrone

Similaire à PHP avec ReactPHP ou Swoole, ou à JavaScript. Une fonction `async` retourne une promesse. On `await` pour attendre son résultat. Ici utilisé uniquement dans les repositories et les application services, jamais dans le cœur du domaine.

---

### Newtypes — des IDs typés

En PHP on passe souvent des `string` ou `int` pour les IDs. En Rust on crée un type dédié :

```rust
pub struct ProductId(Uuid);
```

C'est un "newtype" : une `struct` qui enveloppe un seul type.
Avantage : le compilateur refuse de confondre un `ProductId` avec un `ShopId`.
En PHP tu pourrais passer un `$shopId` là où tu voulais un `$productId` — Rust l'interdit.

---

## Les Aggregates

Un **Aggregate** est un groupe de données traité comme une unité cohérente.
Toutes les modifications passent par sa racine (l'**Aggregate Root**).
C'est comme une classe PHP qui protège ses données internes et n'expose que des méthodes métier.

### `Shop` — la boutique

Une boutique appartient à un maker et est spécialisée sur une catégorie.

```rust
pub struct Shop {
    id: ShopId,
    maker_id: MakerId,       // référence vers le BC Maker — on ne stocke pas l'objet Maker ici
    name: ShopName,
    category: Category,
    status: ShopStatus,
    label: MakerLabel,
    events: Vec<CatalogEvent>,
}
```

**Invariants** (règles que la boutique garantit toujours) :
- Une shop ne change jamais de catégorie (elle serait une autre shop)
- Une shop désactivée ne peut plus recevoir de nouveaux produits
- Le label `Certified` ne peut être attribué qu'à une shop active

**`Vec<CatalogEvent>`** : liste des événements domaine générés pendant l'opération.
`Vec<T>` est l'équivalent d'un `array` PHP, mais typé : ici uniquement des `CatalogEvent`.

#### Méthodes métier de Shop

```rust
impl Shop {
    /// Crée une nouvelle boutique — constructeur validant
    pub fn create(
        maker_id: MakerId,
        name: ShopName,
        category: Category,
    ) -> Result<Self, CatalogError> {
        let id = ShopId::new();
        Ok(Self {
            id: id.clone(),
            maker_id,
            name: name.clone(),
            category: category.clone(),
            status: ShopStatus::Active,
            label: MakerLabel::Uncertified,
            events: vec![CatalogEvent::ShopCreated {
                shop_id: id,
                name,
                category,
            }],
        })
    }

    /// Attribue le label certifié — appelé suite aux avis du BC Community
    pub fn grant_label(&mut self, since: Date, review_count: u32) -> Result<(), CatalogError> {
        if self.status == ShopStatus::Deactivated {
            return Err(CatalogError::ShopDeactivated(self.id.clone()));
        }
        self.label = MakerLabel::Certified { since, review_count };
        self.events.push(CatalogEvent::MakerLabelGranted {
            shop_id: self.id.clone(),
            since,
            review_count,
        });
        Ok(())
    }

    /// Désactivation de la boutique
    pub fn deactivate(&mut self) -> Result<(), CatalogError> {
        if self.status == ShopStatus::Deactivated {
            return Err(CatalogError::ShopAlreadyDeactivated);
        }
        self.status = ShopStatus::Deactivated;
        self.events.push(CatalogEvent::ShopDeactivated {
            shop_id: self.id.clone(),
        });
        Ok(())
    }

    /// Vide et retourne les événements accumulés — appelé après chaque opération
    pub fn drain_events(&mut self) -> Vec<CatalogEvent> {
        std::mem::take(&mut self.events)
    }
}
```

> **`std::mem::take`** : remplace le contenu du `Vec` par un `Vec` vide et retourne l'ancien contenu.
> En PHP ce serait : `$events = $this->events; $this->events = []; return $events;`

---

### `Product` — le produit

Un produit appartient à une boutique. Il gère son propre stock.
C'est un Aggregate Root séparé de Shop pour une raison technique : le stock est modifié
par des commandes concurrentes. Si Product était dans Shop, toute commande
lockerait la boutique entière. Séparés, on locke uniquement le produit commandé.

```rust
pub struct Product {
    id: ProductId,
    shop_id: ShopId,
    name: ProductName,
    price: Price,
    status: ProductStatus,
    stock: StockQuantity,
    fabrication_video: Option<FabricationVideoUrl>,  // badge maker, non obligatoire
    version: u64,                                    // pour le verrouillage optimiste
    events: Vec<CatalogEvent>,
}
```

**`version: u64`** : numéro incrémenté à chaque sauvegarde en base.
Si deux processus chargent le même produit simultanément et tentent de le sauvegarder,
la base rejette la deuxième sauvegarde car sa version est obsolète. C'est l'**optimistic locking**.

#### Cycle de vie d'un produit

```
Draft ──publish()──▶ Published ──reserve_stock()──▶ OutOfStock
  │                      │
  │               make_unavailable()
  │                      │
  │                      ▼
  │                 Unavailable ──restore()──▶ Published (si stock > 0)
  │
  └──── delete() ──▶ Deleted  (irréversible, tombstone)
```

**Tombstone** : un produit supprimé n'est pas effacé de la base. On conserve la trace
pour l'historique des commandes passées. Le statut `Deleted` empêche toute opération.

#### Méthodes métier de Product

```rust
impl Product {
    /// Crée un brouillon — point d'entrée unique, pas de constructeur public
    pub fn draft(
        shop_id: ShopId,
        name: ProductName,
        price: Price,
    ) -> Result<Self, CatalogError> {
        let id = ProductId::new();
        Ok(Self {
            id: id.clone(),
            shop_id: shop_id.clone(),
            name: name.clone(),
            price: price.clone(),
            status: ProductStatus::Draft,
            stock: StockQuantity::zero(),
            fabrication_video: None,
            version: 0,
            events: vec![CatalogEvent::ProductDrafted {
                product_id: id,
                shop_id,
                name,
                price,
            }],
        })
    }

    /// Initialise ou réapprovisionne le stock
    pub fn initialize_stock(&mut self, qty: StockQuantity) -> Result<(), CatalogError> {
        if self.status == ProductStatus::Deleted {
            return Err(CatalogError::ProductIsDeleted(self.id.clone()));
        }
        let qty_val = qty.value();
        self.stock = qty;
        // Si le produit était OutOfStock et qu'on réapprovisionne, on le repasse Published
        if self.status == ProductStatus::OutOfStock {
            self.status = ProductStatus::Published;
        }
        self.events.push(CatalogEvent::StockInitialized {
            product_id: self.id.clone(),
            quantity: qty_val,
        });
        Ok(())
    }

    /// Publication — nécessite un stock > 0
    pub fn publish(&mut self) -> Result<(), CatalogError> {
        match self.status {
            ProductStatus::Deleted   => return Err(CatalogError::ProductIsDeleted(self.id.clone())),
            ProductStatus::Published => return Err(CatalogError::AlreadyPublished),
            _ => {}
        }
        if self.stock.is_depleted() {
            return Err(CatalogError::CannotPublishWithoutStock);
        }
        self.status = ProductStatus::Published;
        self.events.push(CatalogEvent::ProductPublished {
            product_id: self.id.clone(),
            shop_id: self.shop_id.clone(),
            has_fabrication_video: self.fabrication_video.is_some(),
        });
        Ok(())
    }

    /// Attache une vidéo de fabrication — enrichit le produit, non obligatoire
    pub fn attach_fabrication_video(&mut self, url: FabricationVideoUrl) -> Result<(), CatalogError> {
        if self.status == ProductStatus::Deleted {
            return Err(CatalogError::ProductIsDeleted(self.id.clone()));
        }
        self.fabrication_video = Some(url);
        self.events.push(CatalogEvent::FabricationVideoAttached {
            product_id: self.id.clone(),
        });
        Ok(())
    }

    /// Réservation de stock — appelée par le BC Order
    pub fn reserve_stock(&mut self, qty: u32) -> Result<(), CatalogError> {
        if self.status != ProductStatus::Published {
            return Err(CatalogError::ProductNotAvailableForOrder(self.status.clone()));
        }
        self.stock = self.stock.reserve(qty)?;  // Le ? propage l'erreur si stock insuffisant
        if self.stock.is_depleted() {
            self.status = ProductStatus::OutOfStock;
            self.events.push(CatalogEvent::ProductWentOutOfStock {
                product_id: self.id.clone(),
            });
        }
        Ok(())
    }

    /// Libération de stock — annulation de commande
    pub fn release_stock(&mut self, qty: u32) -> Result<(), CatalogError> {
        if self.status == ProductStatus::Deleted {
            return Err(CatalogError::ProductIsDeleted(self.id.clone()));
        }
        self.stock = self.stock.release(qty)?;
        if self.status == ProductStatus::OutOfStock && !self.stock.is_depleted() {
            self.status = ProductStatus::Published;
        }
        Ok(())
    }

    /// Mise en pause explicite par le maker
    pub fn make_unavailable(&mut self, reason: Option<String>) -> Result<(), CatalogError> {
        if self.status == ProductStatus::Deleted {
            return Err(CatalogError::ProductIsDeleted(self.id.clone()));
        }
        self.status = ProductStatus::Unavailable;
        self.events.push(CatalogEvent::ProductMadeUnavailable {
            product_id: self.id.clone(),
            reason,
        });
        Ok(())
    }

    /// Restore un produit Unavailable ou OutOfStock
    pub fn restore(&mut self) -> Result<(), CatalogError> {
        match self.status {
            ProductStatus::Deleted => return Err(CatalogError::ProductIsDeleted(self.id.clone())),
            ProductStatus::Published => return Err(CatalogError::AlreadyPublished),
            _ => {}
        }
        if self.stock.is_depleted() {
            return Err(CatalogError::CannotPublishWithoutStock);
        }
        self.status = ProductStatus::Published;
        self.events.push(CatalogEvent::ProductRestored {
            product_id: self.id.clone(),
        });
        Ok(())
    }

    /// Suppression définitive — tombstone
    pub fn delete(&mut self) -> Result<(), CatalogError> {
        if self.status == ProductStatus::Deleted {
            return Err(CatalogError::ProductIsDeleted(self.id.clone()));
        }
        self.status = ProductStatus::Deleted;
        self.events.push(CatalogEvent::ProductDeleted {
            product_id: self.id.clone(),
        });
        Ok(())
    }

    pub fn drain_events(&mut self) -> Vec<CatalogEvent> {
        std::mem::take(&mut self.events)
    }
}
```

> **L'opérateur `?`** : c'est du sucre syntaxique Rust.
> `self.stock.reserve(qty)?` est équivalent en PHP à :
> ```php
> $result = $this->stock->reserve($qty);
> if ($result instanceof Err) { return $result; }
> $value = $result->unwrap();
> ```
> Il retourne l'erreur immédiatement si c'en est une, sinon extrait la valeur.

---

## Les Value Objects

Un **Value Object** n'a pas d'identité. Deux instances avec les mêmes valeurs sont égales.
En PHP tu pourrais les représenter avec des classes `readonly` et `__construct` validant.

### `StockQuantity`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StockQuantity(u32);  // newtype autour d'un entier non signé 32 bits

impl StockQuantity {
    const MAX: u32 = 99_999;

    pub fn new(qty: u32) -> Result<Self, CatalogError> {
        if qty > Self::MAX {
            return Err(CatalogError::StockQuantityTooLarge(qty));
        }
        Ok(Self(qty))
    }

    pub fn zero() -> Self { Self(0) }

    pub fn reserve(&self, qty: u32) -> Result<Self, CatalogError> {
        self.0
            .checked_sub(qty)                   // soustraction sans underflow
            .map(Self)                           // enveloppe le résultat dans StockQuantity
            .ok_or(CatalogError::InsufficientStock {
                available: self.0,
                requested: qty,
            })
    }

    pub fn release(&self, qty: u32) -> Result<Self, CatalogError> {
        let new_qty = self.0.checked_add(qty)
            .ok_or(CatalogError::StockOverflow)?;
        Self::new(new_qty)
    }

    pub fn value(&self) -> u32 { self.0 }
    pub fn is_depleted(&self) -> bool { self.0 == 0 }
}
```

> **`#[derive(Debug, Clone, PartialEq, Eq)]`** : des macros Rust qui génèrent automatiquement
> du code. `Debug` permet d'afficher la valeur avec `{:?}`. `Clone` permet de copier.
> `PartialEq + Eq` permettent les comparaisons avec `==`.
> En PHP, `PartialEq` serait l'équivalent d'implémenter `__eq` ou de comparer les valeurs.

### `Price`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Price {
    amount_cents: u64,  // on stocke en centimes pour éviter les erreurs de virgule flottante
    currency: Currency,
}

impl Price {
    pub fn new(amount_cents: u64, currency: Currency) -> Result<Self, CatalogError> {
        if amount_cents == 0 {
            return Err(CatalogError::PriceMustBePositive);
        }
        Ok(Self { amount_cents, currency })
    }

    pub fn amount_cents(&self) -> u64 { self.amount_cents }
    pub fn currency(&self) -> &Currency { &self.currency }
}
```

> Pourquoi des centimes ? `0.1 + 0.2 == 0.30000000000000004` en virgule flottante.
> On stocke `1099` pour 10,99€. C'est une pratique universelle en e-commerce.

---

## Les Erreurs du domaine

Rust n'a pas d'exceptions. Les erreurs sont des valeurs, représentées par un `enum`.
La crate `thiserror` génère automatiquement les messages d'erreur.

```rust
#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("Le produit {0:?} est supprimé et ne peut plus être modifié")]
    ProductIsDeleted(ProductId),

    #[error("Impossible de publier sans stock disponible")]
    CannotPublishWithoutStock,

    #[error("Le produit est déjà publié")]
    AlreadyPublished,

    #[error("Stock insuffisant : disponible {available}, demandé {requested}")]
    InsufficientStock { available: u32, requested: u32 },

    #[error("Le produit n'est pas disponible à la commande (statut : {0:?})")]
    ProductNotAvailableForOrder(ProductStatus),

    #[error("La quantité de stock dépasse le maximum autorisé ({0})")]
    StockQuantityTooLarge(u32),

    #[error("Le prix doit être supérieur à zéro")]
    PriceMustBePositive,

    #[error("La boutique {0:?} est désactivée")]
    ShopDeactivated(ShopId),

    #[error("La boutique est déjà désactivée")]
    ShopAlreadyDeactivated,

    #[error("Overflow lors du calcul du stock")]
    StockOverflow,
}
```

---

## Les Domain Events

Les événements décrivent ce qui s'est passé, au passé, dans le domaine.
Ils sont émis par les aggregates et consommés par d'autres BCs ou pour mettre à jour
des projections (vues de lecture, index de recherche...).

```rust
#[derive(Debug, Clone)]
pub enum CatalogEvent {
    // Shop
    ShopCreated       { shop_id: ShopId, name: ShopName, category: Category },
    ShopDeactivated   { shop_id: ShopId },
    MakerLabelGranted { shop_id: ShopId, since: Date, review_count: u32 },

    // Product
    ProductDrafted    { product_id: ProductId, shop_id: ShopId, name: ProductName, price: Price },
    ProductPublished  { product_id: ProductId, shop_id: ShopId, has_fabrication_video: bool },
    ProductMadeUnavailable { product_id: ProductId, reason: Option<String> },
    ProductRestored   { product_id: ProductId },
    ProductDeleted    { product_id: ProductId },
    ProductWentOutOfStock  { product_id: ProductId },

    // Stock
    StockInitialized  { product_id: ProductId, quantity: u32 },

    // Contenu maker
    FabricationVideoAttached { product_id: ProductId },
}
```

---

## Les Repositories (interfaces)

Les repositories sont des **traits** (interfaces) définis dans le domaine.
Le domaine ne sait pas comment les données sont stockées.
L'infrastructure (Postgres, etc.) implémente ces traits.

```rust
pub trait ProductRepository: Send + Sync {
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, CatalogError>;
    async fn find_by_shop(&self, shop_id: &ShopId) -> Result<Vec<Product>, CatalogError>;
    async fn save(&self, product: &Product) -> Result<(), CatalogError>;
}

pub trait ShopRepository: Send + Sync {
    async fn find_by_id(&self, id: &ShopId) -> Result<Option<Shop>, CatalogError>;
    async fn find_by_maker(&self, maker_id: &MakerId) -> Result<Vec<Shop>, CatalogError>;
    async fn save(&self, shop: &Shop) -> Result<(), CatalogError>;
}
```

> `find_by_id` retourne `Option<Product>` : soit `Some(product)` si trouvé, soit `None`.
> Jamais `null`. Le compilateur force l'appelant à gérer le cas "pas trouvé".

---

## Interactions avec les autres BCs

Le Catalog ne connaît pas les autres BCs directement.
La communication se fait via des **Domain Events** consommés par des **Application Services**.

```
BC Community ──[MakerLabelGranted]──▶ Application Service
                                            │
                                            ▼
                                    shop.grant_label(...)
                                    shop_repo.save(&shop)

BC Order     ──[OrderConfirmed]────▶ Application Service
                                            │
                                            ▼
                                    product.reserve_stock(qty)
                                    product_repo.save(&product)

BC Order     ──[OrderCancelled]────▶ Application Service
                                            │
                                            ▼
                                    product.release_stock(qty)
                                    product_repo.save(&product)

Catalog      ──[ProductPublished]──▶ BC Storefront (mise à jour index recherche)
Catalog      ──[ProductDeleted]────▶ BC Storefront (dépublication)
```

---

## Structure du crate `catalog-domain`

```
crates/catalog-domain/
├── Cargo.toml
└── src/
    ├── lib.rs              ← point d'entrée, réexporte les types publics
    ├── ids.rs              ← ShopId, ProductId
    ├── shop.rs             ← Aggregate Shop
    ├── product.rs          ← Aggregate Product
    ├── value_objects.rs    ← StockQuantity, Price, Currency, Category...
    ├── events.rs           ← CatalogEvent
    ├── errors.rs           ← CatalogError
    └── repositories.rs     ← traits ShopRepository, ProductRepository
```

```toml
# crates/catalog-domain/Cargo.toml
[package]
name = "catalog-domain"
version = "0.1.0"
edition = "2024"

[dependencies]
shared-kernel = { path = "../shared-kernel" }
uuid.workspace = true
serde.workspace = true
thiserror.workspace = true
# Pas de tokio ici — le domaine est sync, les traits repo sont async via AFIT
```

> **AFIT (Async Functions in Traits)** : depuis Rust 1.75, les traits peuvent avoir des méthodes
> `async fn` directement. Le domaine définit des traits async sans dépendre de Tokio.
> Tokio n'arrive qu'en infrastructure.
