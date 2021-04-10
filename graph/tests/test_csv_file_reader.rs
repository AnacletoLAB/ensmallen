extern crate graph;
use graph::CSVFileReader;

#[test]
/// Testing if a proper exception is raised when given path does not exists.
fn test_illegal_path() {
    assert!(CSVFileReader::new("non_existant_file", "".to_string(), "".to_string()).is_err());
}

#[test]
/// Testing if a proper exceptions are raised when given path is empty.
fn test_empty_file() -> Result<(), String> {
    assert!(
        CSVFileReader::new("tests/data/empty_file.tsv", "".to_string(), "".to_string())?
            .get_header()
            .is_err()
    );
    assert!(
        CSVFileReader::new("tests/data/empty_file.tsv", "".to_string(), "".to_string())?
            .get_elements_per_line()
            .is_err()
    );
    assert!(
        CSVFileReader::new("tests/data/empty_file.tsv", "".to_string(), "".to_string())?
            .get_column_number("kebab".to_owned())
            .is_err()
    );
    Ok(())
}

#[test]
/// Testing if a proper exception is raised when header is wrong.
fn test_wrong_header() -> Result<(), String> {
    assert!(
        CSVFileReader::new("tests/data/macaque.tsv", "".to_string(), "".to_string())?
            .get_column_number("kebab".to_owned())
            .is_err()
    );
    Ok(())
}
