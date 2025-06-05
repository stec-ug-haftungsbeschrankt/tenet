use std::{str::FromStr, fmt::Display};
#[derive(Debug, Clone, Copy, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, PartialOrd)]
pub enum ApplicationType {
    Shop
}

impl FromStr for ApplicationType {
    type Err = ();

    fn from_str(input: &str) -> Result<ApplicationType, Self::Err> {
        match input {
            "Shop"  => Ok(ApplicationType::Shop),
            _  => Err(()),
        }
    }
}

impl Display for ApplicationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        // Test valid conversion
        let app_type = ApplicationType::from_str("Shop");
        assert!(app_type.is_ok());
        assert_eq!(app_type.unwrap(), ApplicationType::Shop);

        // Test invalid conversion
        let invalid = ApplicationType::from_str("Blog");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_display() {
        let app_type = ApplicationType::Shop;
        assert_eq!(format!("{}", app_type), "Shop");
    }

    #[test]
    fn test_serialization() {
        // Test serialization
        let app_type = ApplicationType::Shop;
        let serialized = serde_json::to_string(&app_type).unwrap();
        assert_eq!(serialized, "\"Shop\"");

        // Test deserialization
        let deserialized: ApplicationType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ApplicationType::Shop);
    }

    #[test]
    fn test_clone_and_copy() {
        let original = ApplicationType::Shop;

        // Test Clone
        let cloned = original.clone();
        assert_eq!(original, cloned);

        // Test Copy
        let copied = original;
        assert_eq!(original, copied);
        // Verify original still accessible (Copy trait)
        assert_eq!(original, ApplicationType::Shop);
    }
}
