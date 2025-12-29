use serde::{Deserialize, Serialize};

use crate::error::ValidationError;
use crate::validation::{Validate, validate_slug, validate_year};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SlugName {
    pub slug: String,
    pub name: String,
}

impl SlugName {
    pub fn new(slug: impl Into<String>, name: impl Into<String>) -> Result<Self, ValidationError> {
        let slug = slug.into();
        let name = name.into();

        validate_slug(&slug)?;

        if name.is_empty() {
            return Err(ValidationError::empty_value("name"));
        }

        Ok(Self { slug, name })
    }

    #[must_use]
    pub fn slug(&self) -> &str {
        &self.slug
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Validate for SlugName {
    fn validate(&self) -> Result<(), ValidationError> {
        validate_slug(&self.slug)?;
        if self.name.is_empty() {
            return Err(ValidationError::empty_value("name"));
        }
        Ok(())
    }
}

impl std::fmt::Display for SlugName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Year(u16);

impl Year {
    pub fn new(value: u16) -> Result<Self, ValidationError> {
        validate_year(value)?;
        Ok(Self(value))
    }

    #[must_use]
    pub const fn value(&self) -> u16 {
        self.0
    }
}

impl Validate for Year {
    fn validate(&self) -> Result<(), ValidationError> {
        validate_year(self.0)
    }
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Year> for u16 {
    fn from(year: Year) -> Self {
        year.0
    }
}

impl TryFrom<u16> for Year {
    type Error = ValidationError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VehicleId {
    pub make_slug: String,
    pub model_slug: String,
    pub year: u16,
    pub trim_slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_slug: Option<String>,
}

impl VehicleId {
    pub fn new(
        make_slug: impl Into<String>,
        model_slug: impl Into<String>,
        year: u16,
        trim_slug: impl Into<String>,
        variant_slug: Option<String>,
    ) -> Result<Self, ValidationError> {
        let make_slug = make_slug.into();
        let model_slug = model_slug.into();
        let trim_slug = trim_slug.into();

        validate_slug(&make_slug)?;
        validate_slug(&model_slug)?;
        validate_year(year)?;
        validate_slug(&trim_slug)?;

        if let Some(ref variant) = variant_slug {
            validate_slug(variant)?;
        }

        Ok(Self {
            make_slug,
            model_slug,
            year,
            trim_slug,
            variant_slug,
        })
    }
}

impl std::fmt::Display for VehicleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.variant_slug {
            Some(variant) => write!(
                f,
                "{}:{}:{}:{}:{}",
                self.make_slug, self.model_slug, self.year, self.trim_slug, variant
            ),
            None => write!(
                f,
                "{}:{}:{}:{}",
                self.make_slug, self.model_slug, self.year, self.trim_slug
            ),
        }
    }
}
