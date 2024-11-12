#[cfg(test)]
mod test {

    use serde::{Deserialize, Serialize};
    use serde_infer::serde_infer;
    use serde_json::{from_value, json, to_value};

    #[serde_infer(outgoing = "upper")]
    #[derive(Serialize, Deserialize)]
    pub struct DataSource {
        title: String,
        author: String,
        star_count: f32,
    }

    #[test]
    pub fn incoming() {
        assert!(from_value::<DataSource>(json!({
            "Title": "Love and Limerence: The Experience of Being in Love",
            "Author": "Dorothy Tennov",
            "Star Count": 3.94
        }))
        .is_ok())
    }

    #[test]
    pub fn outgoing() {
        assert_eq!(
            to_value(DataSource {
                title: "Something".to_string(),
                author: "Something".to_string(),
                star_count: 0.0
            })
            .unwrap(),
            json!({
                "TITLE": "Something",
                "AUTHOR": "Something",
                "STAR_   COUNT": 0.0
            })
        );
    }
}
