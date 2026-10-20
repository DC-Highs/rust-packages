use std::collections::HashSet;

use crate::enums::DragonElement;
use crate::settings::elements::element_info;
use crate::tools::validation::{validate_element_name, ValidationError};

pub fn calculate_element_resistances(
    element: DragonElement,
) -> &'static [DragonElement] {
    element_info(element).weaknesses
}

pub fn calculate_element_resistances_by_name(
    element_name: &str,
) -> Result<&'static [DragonElement], ValidationError> {
    let elem = validate_element_name(element_name)?;
    Ok(calculate_element_resistances(elem))
}

pub fn calculate_element_strengths(
    element: DragonElement,
) -> &'static [DragonElement] {
    element_info(element).strengths
}

pub fn calculate_element_strengths_by_name(
    element_name: &str,
) -> Result<&'static [DragonElement], ValidationError> {
    let elem = validate_element_name(element_name)?;
    Ok(calculate_element_strengths(elem))
}

pub fn calculate_element_weaknesses(
    element: DragonElement,
) -> Vec<DragonElement> {
    let mut weaknesses = Vec::new();
    for &candidate in DragonElement::ALL {
        if element_info(candidate).strengths.contains(&element) {
            weaknesses.push(candidate);
        }
    }
    weaknesses
}

pub fn calculate_element_weaknesses_by_name(
    element_name: &str,
) -> Result<Vec<DragonElement>, ValidationError> {
    let elem = validate_element_name(element_name)?;
    Ok(calculate_element_weaknesses(elem))
}

pub fn calculate_elements_strengths(
    elements: &[DragonElement],
) -> Vec<DragonElement> {
    let mut unique = HashSet::new();
    let mut result = Vec::new();

    for &elem in elements {
        for &s in element_info(elem).strengths {
            if unique.insert(s) {
                result.push(s);
            }
        }
    }

    result
}

pub fn calculate_elements_strengths_by_names(
    element_names: &[&str],
) -> Result<Vec<DragonElement>, ValidationError> {
    let mut elements = Vec::with_capacity(element_names.len());
    for name in element_names {
        elements.push(validate_element_name(name)?);
    }
    Ok(calculate_elements_strengths(&elements))
}
