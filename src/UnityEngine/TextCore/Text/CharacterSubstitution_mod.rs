#[cfg(feature = "UnityEngine+TextCore+Text+CharacterSubstitution")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CharacterSubstitution {
    pub index: i32,
    pub unicode: u32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+CharacterSubstitution")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::CharacterSubstitution {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "CharacterSubstitution";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+CharacterSubstitution")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::TextCore::Text::CharacterSubstitution {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+CharacterSubstitution")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::TextCore::Text::CharacterSubstitution {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+CharacterSubstitution")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::TextCore::Text::CharacterSubstitution {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+CharacterSubstitution")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::TextCore::Text::CharacterSubstitution {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+CharacterSubstitution")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TextCore::Text::CharacterSubstitution {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+CharacterSubstitution")]
impl crate::UnityEngine::TextCore::Text::CharacterSubstitution {
    pub fn _ctor(
        &mut self,
        index: i32,
        unicode: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, u32), quest_hook::libil2cpp::Void, 2usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (index, unicode))
        };
        Ok(__cordl_ret.into())
    }
}
