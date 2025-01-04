#[cfg(feature = "UnityEngineInternal+TypeInferenceRules")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeInferenceRules {
    #[default]
    ArrayOfTypeReferencedByFirstArgument = 2i32,
    TypeOfFirstArgument = 3i32,
    TypeReferencedByFirstArgument = 0i32,
    TypeReferencedBySecondArgument = 1i32,
}
#[cfg(feature = "UnityEngineInternal+TypeInferenceRules")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngineInternal::TypeInferenceRules =>
    "UnityEngineInternal"."TypeInferenceRules"
);
