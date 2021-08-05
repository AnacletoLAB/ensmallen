use super::*;

#[automatically_generated_function]
/// Returns URL from given STRING node name.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Safety
/// This method assumes that the provided node name is a {library_name} node name and
/// may cause a panic if the aforementioned assumption is not true.
///
pub(crate) unsafe fn format_string_url_from_node_name(node_name: &str) -> String {
    {
        format!("https://string-db.org/network/{}", node_name)
    }
}

/// Returns whether the given node name respects the STRING nodes pattern.
///
/// # Arguments
/// * `node_name`: &str - Node name to check pattern with.
///
/// # Example
/// To validate a node you can use:
/// ```rust
/// # use graph::*;
/// let this_library_node_name1 = "287.DR97_1";
/// let this_library_node_name2 = "287.DR97_999";
/// let not_this_library_node_name = "PizzaQuattroStagioni";
/// assert!(may_be_string_node_name(this_library_node_name1));
/// assert!(may_be_string_node_name(this_library_node_name2));
/// assert!(!may_be_string_node_name(not_this_library_node_name));
/// ```
pub(crate) fn may_be_string_node_name(node_name: &str) -> bool {
    let mut begins_with_number = false;
    let mut contains_a_dot_after_number = false;
    let mut has_characters_after_dot = false;
    for character in node_name.chars(){
        if character == '.' {
            if begins_with_number{
                contains_a_dot_after_number = true;
                continue;
            } else {
                break;
            }
        }
        if contains_a_dot_after_number {
            has_characters_after_dot = true;
            break;
        }
        // Otherwise if the character
        // is a digit, we know that we 
        // are continuing from a list
        // of integers otherwise we
        // would have already broken
        // out of the for loop.
        if character.is_digit(10){
            begins_with_number = true;
        } else {
            begins_with_number = false;
            break;
        }
    }
    begins_with_number && contains_a_dot_after_number && has_characters_after_dot
}