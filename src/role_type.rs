use std::{str::FromStr, fmt::Display};

/// Represents the different user roles in the system.
///
/// This enum defines the different permission levels that users
/// can have within an application.
///
/// # Example
///
/// ```
/// use tenet::role_type::RoleType;
/// use std::str::FromStr;
///
/// let admin_role = RoleType::Administrator;
/// let user_role = RoleType::User;
///
/// // Parsing a role from a string
/// let parsed_role = RoleType::from_str("Administrator").unwrap();
/// assert_eq!(parsed_role, RoleType::Administrator);
/// ```
#[derive(Debug, Clone, Copy, serde_derive::Serialize, serde_derive::Deserialize, PartialEq, PartialOrd)]
pub enum RoleType {
    /// Administrator role with comprehensive permissions
    Administrator = 99,
    /// Standard user role with limited permissions
    User = 50
}

impl FromStr for RoleType {
    type Err = ();

    /// Converts a string to a `RoleType`.
    ///
    /// # Parameters
    ///
    /// * `input` - The string to convert
    ///
    /// # Errors
    ///
    /// Returns an empty error if the string doesn't match a valid `RoleType`.
    fn from_str(input: &str) -> Result<RoleType, Self::Err> {
        match input {
            "Administrator"  => Ok(RoleType::Administrator),
            "User" => Ok(RoleType::User),
            _  => Err(()),
        }
    }
}

impl Display for RoleType {
    /// Implements the display of the `RoleType` as a string.
    ///
    /// # Parameters
    ///
    /// * `f` - The formatter
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_valid() {
        // Test Administrator role
        let admin = RoleType::from_str("Administrator");
        assert!(admin.is_ok());
        assert_eq!(admin.unwrap(), RoleType::Administrator);

        // Test User role
        let user = RoleType::from_str("User");
        assert!(user.is_ok());
        assert_eq!(user.unwrap(), RoleType::User);
    }

    #[test]
    fn test_from_str_invalid() {
        // Test invalid role name
        let invalid = RoleType::from_str("Guest");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_display() {
        // Test display for Administrator
        let admin = RoleType::Administrator;
        assert_eq!(format!("{}", admin), "Administrator");

        // Test display for User
        let user = RoleType::User;
        assert_eq!(format!("{}", user), "User");
    }

    #[test]
    fn test_comparison() {
        // Test ordering/comparison (Administrator should be > User)
        assert!(RoleType::Administrator > RoleType::User);

        // Test equality
        assert_eq!(RoleType::Administrator, RoleType::Administrator);
        assert_eq!(RoleType::User, RoleType::User);
        assert_ne!(RoleType::Administrator, RoleType::User);
    }

    #[test]
    fn test_serialization() {
        // Test serialization
        let admin = RoleType::Administrator;
        let serialized = serde_json::to_string(&admin).unwrap();
        assert_eq!(serialized, "\"Administrator\"");

        // Test deserialization
        let deserialized: RoleType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, RoleType::Administrator);
    }
}