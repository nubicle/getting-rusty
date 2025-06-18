use serde::{de::Visitor, Deserialize};

#[derive(Default)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

impl<'de> Deserialize<'de> for Person {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PersonVisitor {})
    }
}

struct PersonVisitor;

impl<'de> Visitor<'de> for PersonVisitor {
    type Value = Person;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Could not deserialize person")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut name = None;
        let mut age = None;

        for pair in v.split('|') {
            let mut parts = pair.splitn(2, ':');
            match (parts.next(), parts.next()) {
                (Some("Name"), Some(v)) => name = Some(v.to_string()),
                (Some("Age"), Some(v)) => {
                    age = v.parse::<u8>().ok();
                }
                _ => return Err(E::custom("Invalid key or value")),
            }
        }

        let name = name.ok_or_else(|| E::missing_field("Name"))?;
        let age = age.ok_or_else(|| E::missing_field("Age"))?;

        Ok(Person { name, age })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = "\"Name:John Doe|Age:30\"";
        let p: Person = serde_json::from_str(data).expect("failed to deserialize");

        assert_eq!(p.age, 30);
        assert_eq!(p.name, String::from("John Doe"));
    }
}
