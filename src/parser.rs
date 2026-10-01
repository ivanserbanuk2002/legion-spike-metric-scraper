use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
}

pub fn parse_metrics(json: &str) -> serde_json::Result<Vec<Metric>> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::parse_metrics;

    #[test]
    fn parses_named_numeric_metrics_and_rejects_invalid_shapes() {
        let metrics = parse_metrics(r#"[{"name":"debt","value":-2.5},{"name":"load","value":3}]"#)
            .expect("valid metric array");

        assert_eq!(metrics.len(), 2);
        assert_eq!(metrics[0].name, "debt");
        assert_eq!(metrics[0].value, -2.5);
        assert_eq!(metrics[1].name, "load");
        assert_eq!(metrics[1].value, 3.0);
        assert!(parse_metrics(r#"[{"name":"missing"}]"#).is_err());
        assert!(parse_metrics(r#"[{"name":"text","value":"3"}]"#).is_err());
        assert!(parse_metrics(r#"[{"name":"huge","value":1e400}]"#).is_err());
        assert!(parse_metrics("not JSON").is_err());
    }
}
