#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+QueryOperator")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryOperator {
    And = 8i32,
    Equals = 1i32,
    Exists = 3i32,
    GreaterThan = 6i32,
    GreaterThanOrEquals = 7i32,
    LessThan = 4i32,
    LessThanOrEquals = 5i32,
    None = 0i32,
    NotEquals = 2i32,
    Or = 9i32,
    RegexEquals = 10i32,
    StrictEquals = 11i32,
    StrictNotEquals = 12i32,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JsonPath+QueryOperator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::JsonPath::QueryOperator
    => "Newtonsoft.Json.Linq.JsonPath"."QueryOperator"
);
