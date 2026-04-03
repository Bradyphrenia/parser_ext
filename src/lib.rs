use pgrx::prelude::*;

pgrx::pg_module_magic!();

#[pg_extern]
fn parse_diskret_zahl(wert: Option<&str>) -> Option<i32> {
    let wert = wert?;
    parse_diskret_zahl_impl(wert)
}

fn parse_diskret_zahl_impl(wert: &str) -> Option<i32> {
    let mut s: String = wert
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    s = s.replace('?', "");
    s = s.replace('≈', "~");
    s = s.replace('∼', "~");

    if s.is_empty() {
        return None;
    }

    // Entspricht deiner hochgeladenen Funktion:
    // ∞ -> 21
    if s == "∞" {
        return Some(21);
    }

    if let Some(rest) = s.strip_prefix('<') {
        return rest.parse::<i32>().ok().map(|n| n - 1);
    }

    if let Some(rest) = s.strip_prefix('>') {
        return rest.parse::<i32>().ok().map(|n| n + 1);
    }

    if let Some(rest) = s.strip_prefix("0~") {
        return rest.parse::<i32>().ok();
    }

    if let Some(rest) = s.strip_prefix('~') {
        return rest.parse::<i32>().ok();
    }

    s.parse::<i32>().ok()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;

    #[test]
    fn test_parse_diskret_zahl() {
        assert_eq!(parse_diskret_zahl_impl(">10?"), Some(11));
        assert_eq!(parse_diskret_zahl_impl("<5"), Some(4));
        assert_eq!(parse_diskret_zahl_impl("0 ~ 3"), Some(3));
        assert_eq!(parse_diskret_zahl_impl("≈4"), Some(4));
        assert_eq!(parse_diskret_zahl_impl("12"), Some(12));
        assert_eq!(parse_diskret_zahl_impl("?"), None);
        assert_eq!(parse_diskret_zahl_impl("∞"), Some(21));
    }
}