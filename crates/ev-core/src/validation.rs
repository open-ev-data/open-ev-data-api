use crate::error::ValidationError;

pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;

    fn validate_all(&self) -> Result<(), ValidationError> {
        self.validate()
    }
}

pub fn validate_slug(value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Err(ValidationError::empty_value("slug"));
    }

    let is_valid = value
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');

    if !is_valid {
        return Err(ValidationError::invalid_slug(value));
    }

    Ok(())
}

pub fn validate_year(value: u16) -> Result<(), ValidationError> {
    if !(1900..=2100).contains(&value) {
        return Err(ValidationError::InvalidYear { value });
    }
    Ok(())
}

pub fn validate_country_code(code: &str) -> Result<(), ValidationError> {
    if code.len() != 2 || !code.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::InvalidCountryCode {
            code: code.to_string(),
        });
    }
    Ok(())
}

pub fn validate_currency_code(code: &str) -> Result<(), ValidationError> {
    if code.len() != 3 || !code.chars().all(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::InvalidCurrencyCode {
            code: code.to_string(),
        });
    }
    Ok(())
}

pub fn validate_url(url: &str) -> Result<(), ValidationError> {
    if url.is_empty() {
        return Err(ValidationError::empty_value("url"));
    }

    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(ValidationError::InvalidUrl {
            url: url.to_string(),
        });
    }

    Ok(())
}

pub fn collect_errors<T, I>(
    items: I,
    validator: fn(&T) -> Result<(), ValidationError>,
) -> Result<(), ValidationError>
where
    I: IntoIterator<Item = T>,
{
    let errors: Vec<ValidationError> = items
        .into_iter()
        .filter_map(|item| validator(&item).err())
        .collect();

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().expect("checked non-empty"))
    } else {
        Err(ValidationError::Multiple(errors))
    }
}
