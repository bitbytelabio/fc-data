use super::{SsiDate, ValidationError};

const PAGE_SIZES: &[u16] = &[10, 20, 50, 100, 500, 1000];
const SECURITIES_PAGE_SIZES: &[u16] = &[10, 20, 50, 100, 1000];
const STOCK_PAGE_SIZES: &[u16] = &[10, 20, 50, 100];

pub(super) fn page(page_index: u8, page_size: u16) -> Result<(), ValidationError> {
    if !(1..=10).contains(&page_index) {
        return Err(ValidationError::InvalidPageIndex(page_index));
    }
    if !PAGE_SIZES.contains(&page_size) {
        return Err(ValidationError::InvalidPageSize(page_size));
    }
    Ok(())
}

pub(super) fn stock_page_size(page_size: u16) -> Result<(), ValidationError> {
    if STOCK_PAGE_SIZES.contains(&page_size) {
        Ok(())
    } else {
        Err(ValidationError::InvalidPageSize(page_size))
    }
}

pub(super) fn securities_page_size(page_size: u16) -> Result<(), ValidationError> {
    if SECURITIES_PAGE_SIZES.contains(&page_size) {
        Ok(())
    } else {
        Err(ValidationError::InvalidPageSize(page_size))
    }
}

pub(super) fn required(value: &str, field: &'static str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        Err(ValidationError::Missing(field))
    } else {
        Ok(())
    }
}

pub(super) fn optional(value: Option<&str>, field: &'static str) -> Result<(), ValidationError> {
    value.map_or(Ok(()), |value| required(value, field))
}

pub(super) fn date(value: &str, field: &'static str) -> Result<SsiDate, ValidationError> {
    SsiDate::parse(value).map_err(|_| ValidationError::InvalidDate(field))
}

pub(super) fn optional_date(
    value: &str,
    field: &'static str,
) -> Result<Option<SsiDate>, ValidationError> {
    if value.is_empty() {
        Ok(None)
    } else {
        date(value, field).map(Some)
    }
}

pub(super) fn resolution(value: u16) -> Result<(), ValidationError> {
    if (1..=1440).contains(&value) {
        Ok(())
    } else {
        Err(ValidationError::InvalidResolution(value))
    }
}
