#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Vector2MagnitudeComparer {}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Utilities";
    const CLASS_NAME: &'static str = "Vector2MagnitudeComparer";
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
impl crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
    pub fn Compare(
        &mut self,
        x: crate::UnityEngine::Vector2,
        y: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector2, crate::UnityEngine::Vector2),
                i32,
                2usize,
            >("Compare")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Compare", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (x, y)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
impl AsRef<crate::System::Collections::Generic::IComparer_1<crate::UnityEngine::Vector2>>
for crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IComparer_1<crate::UnityEngine::Vector2> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+Vector2MagnitudeComparer")]
impl AsMut<crate::System::Collections::Generic::IComparer_1<crate::UnityEngine::Vector2>>
for crate::UnityEngine::InputSystem::Utilities::Vector2MagnitudeComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IComparer_1<
        crate::UnityEngine::Vector2,
    > {
        todo!()
    }
}
