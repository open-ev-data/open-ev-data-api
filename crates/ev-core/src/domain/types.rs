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

    #[must_use]
    pub fn canonical_id(&self) -> String {
        match &self.variant_slug {
            Some(variant) => format!(
                "oed:{}:{}:{}:{}:{}",
                self.make_slug, self.model_slug, self.year, self.trim_slug, variant
            ),
            None => format!(
                "oed:{}:{}:{}:{}",
                self.make_slug, self.model_slug, self.year, self.trim_slug
            ),
        }
    }
}

impl std::fmt::Display for VehicleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slug_name_valid() {
        let slug_name = SlugName::new("tesla", "Tesla").unwrap();
        assert_eq!(slug_name.slug(), "tesla");
        assert_eq!(slug_name.name(), "Tesla");
    }

    #[test]
    fn test_slug_name_invalid_slug() {
        assert!(SlugName::new("Tesla", "Tesla").is_err());
        assert!(SlugName::new("", "Tesla").is_err());
    }

    #[test]
    fn test_slug_name_empty_name() {
        assert!(SlugName::new("tesla", "").is_err());
    }

    #[test]
    fn test_year_valid() {
        assert!(Year::new(2024).is_ok());
        assert!(Year::new(1900).is_ok());
        assert!(Year::new(2100).is_ok());
    }

    #[test]
    fn test_year_invalid() {
        assert!(Year::new(1899).is_err());
        assert!(Year::new(2101).is_err());
    }

    #[test]
    fn test_vehicle_id_canonical() {
        let id = VehicleId::new("tesla", "model_3", 2024, "base", None).unwrap();
        assert_eq!(id.canonical_id(), "oed:tesla:model_3:2024:base");

        let id_with_variant = VehicleId::new(
            "tesla",
            "model_3",
            2024,
            "base",
            Some("long_range".to_string()),
        )
        .unwrap();
        assert_eq!(
            id_with_variant.canonical_id(),
            "oed:tesla:model_3:2024:base:long_range"
        );
    }
}
