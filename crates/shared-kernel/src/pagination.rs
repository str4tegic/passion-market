use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageParams {
    pub page: u32,
    pub per_page: u32,
}

impl PageParams {
    /// Construit des paramètres de pagination validés.
    /// - `page` doit être >= 1
    /// - `per_page` doit être entre 1 et 100
    pub fn new(page: u32, per_page: u32) -> Result<Self, PaginationError> {
        if page == 0 {
            return Err(PaginationError::InvalidPage(page));
        }
        if per_page == 0 || per_page > 100 {
            return Err(PaginationError::InvalidPerPage(per_page));
        }
        Ok(Self { page, per_page })
    }

    /// Calcul de l'offset SQL (sûr contre le débordement u32).
    pub fn offset(&self) -> u64 {
        (self.page as u64 - 1) * self.per_page as u64
    }
}

impl Default for PageParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PaginationError {
    #[error("page must be >= 1, got {0}")]
    InvalidPage(u32),
    #[error("per_page must be between 1 and 100, got {0}")]
    InvalidPerPage(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}
