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
    Administrator,
    /// Standard user role with limited permissions
    User
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
