use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

// TODO: Add custom error codes with `thiserror` crate
pub trait Creator<T: DeserializeOwned> {
    fn create<P: AsRef<Path>>(path: P) -> serde_json::Result<Vec<T>> {
        let f = File::open(path).unwrap();
        let rdr = BufReader::new(f);
        serde_json::from_reader(rdr)
    }
}

pub trait Set<U, I, T>: Creator<U> + Creator<I> + Creator<T>
where
    U: DeserializeOwned,
    I: DeserializeOwned,
    T: DeserializeOwned,
{
    fn units<P: AsRef<Path>>(path: P) -> Vec<U> {
        Self::create(path).unwrap()
    }
    fn items<P: AsRef<Path>>(path: P) -> Vec<I> {
        Self::create(path).unwrap()
    }
    fn traits<P: AsRef<Path>>(path: P) -> Vec<T> {
        Self::create(path).unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "style")]
pub enum TraitStyle {
    Bronze { min: u8, max: Option<u8> },
    Gold { min: u8, max: Option<u8> },
    Silver { min: u8, max: Option<u8> },
    Chromatic { min: u8, max: Option<u8> },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TraitType {
    Origin,
    Class,
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    struct UnitCreator;

    impl Creator<TestUnit> for UnitCreator {}

    struct ItemCreator;

    impl Creator<TestItem> for ItemCreator {}

    struct TraitCreator;

    impl Creator<TestTrait> for TraitCreator {}

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestUnit {
        name: String,
        cost: u8,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestItem {
        name: String,
        description: String,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestTrait {
        key: String,
        name: String,
        description: String,
    }

    #[test]
    pub fn parse_units_json() {
        let expected: Vec<TestUnit> = serde_json::from_str(
            r#"
            [
                {
                    "name": "Allen",
                    "cost": 22
                },
                {
                    "name": "Gueco",
                    "cost": 33
                }
            ]
        "#,
        )
        .unwrap();

        let actual: Vec<TestUnit> =
            UnitCreator::create(Path::new("src/resources/test_units.json")).unwrap();

        assert!(!actual.is_empty());
        assert_eq!(
            TestUnit {
                name: "Allen".to_owned(),
                cost: 22
            },
            actual[0]
        );
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn parse_items_json() {
        let expected: Vec<TestItem> = serde_json::from_str(
            r#"
            [
                {
                    "name": "B. F. Sword",
                    "description": "Gain Attack Damage."
                },
                {
                    "name": "Recurve Bow",
                    "description": "Gain Attack Speed."
                }
            ]
        "#,
        )
        .unwrap();

        let actual: Vec<TestItem> =
            ItemCreator::create(Path::new("src/resources/test_items.json")).unwrap();

        assert!(!actual.is_empty());
        assert_eq!(
            TestItem {
                name: "B. F. Sword".to_owned(),
                description: "Gain Attack Damage.".to_owned(),
            },
            actual[0]
        );
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn parse_traits_json() {
        let expected: Vec<TestTrait> = serde_json::from_str(
            r#"
            [
                {
                    "key": "Set5_Abomination",
                    "name": "Abomination",
                    "description": "Abom desc."
                },
                {
                    "key": "Set5_Assassin",
                    "name": "Assassin",
                    "description": "Sins desc."
                }
            ]
        "#,
        )
        .unwrap();

        let actual: Vec<TestTrait> =
            TraitCreator::create(Path::new("src/resources/test_items.json")).unwrap();

        assert!(!actual.is_empty());
        assert_eq!(expected, actual);
    }
}
