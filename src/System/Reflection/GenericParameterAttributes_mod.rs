#[cfg(feature = "System+Reflection+GenericParameterAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GenericParameterAttributes {
    #[default]
    Contravariant = 2i32,
    Covariant = 1i32,
    DefaultConstructorConstraint = 16i32,
    None = 0i32,
    NotNullableValueTypeConstraint = 8i32,
    ReferenceTypeConstraint = 4i32,
    SpecialConstraintMask = 28i32,
    VarianceMask = 3i32,
}
#[cfg(feature = "System+Reflection+GenericParameterAttributes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::GenericParameterAttributes
    => "System.Reflection"."GenericParameterAttributes"
);
