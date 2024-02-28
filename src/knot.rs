use serde::Deserialize;

#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Knot {
    pub name: String,
    pub youtube: String,
}

impl Knot {
    pub fn slug(&self) -> String {
        return self.name.replace(" ", "-")
            .to_lowercase();
    }
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Knots {
    pub knots: Vec<Knot>,
}

#[cfg(test)]
mod tests {
    use crate::knot::{Knot, Knots};
    use toml;

    #[test]
    pub fn that_knot_can_be_deserialized() {
        // Given some TOML
        let toml = "\n\
                    name = \"Test\"\n\
                    youtube = \"yt-001\"
                    ";

        // When the TOML is deserialized
        let knot: Knot = toml::from_str(toml).unwrap();

        // Then the knot is named Test
        assert_eq!(knot, Knot {
            name: "Test".to_owned(),
            youtube: "yt-001".to_owned()
        });
    }

    #[test]
    pub fn that_slug_is_inferred() {
        // Given some TOML
        let toml = "\n\
                    name = \"Test\"\n\
                    youtube = \"yt-001\"
                    ";

        // When the TOML is deserialized
        let knot: Knot = toml::from_str(toml).unwrap();

        // Then the slug is "test"
        assert_eq!(knot.slug(), "test");
    }

    #[test]
    pub fn that_knots_can_be_deserialized() {
        // Given some TOML
        let toml = "\n\
                    [[knots]]
                    name = \"Eldredge\"\n\
                    youtube = \"yt-001\"\n\
                    \n\
                    [[knots]]
                    name = \"Vidalia\"\n\
                    youtube = \"yt-002\"\n\
                    ";

        // When the TOML is deserialized
        let knots: Knots = toml::from_str(toml).unwrap();

        // Then the knot is named Test
        assert_eq!(knots, Knots {
            knots: vec![
                Knot {
                    name: "Eldredge".to_owned(),
                    youtube: "yt-001".to_owned(),
                },
                Knot {
                    name: "Vidalia".to_owned(),
                    youtube: "yt-002".to_owned(),
                },
            ],
        });
    }
}