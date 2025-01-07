#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputFeatureUsage {
    pub m_Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_InternalType: crate::UnityEngine::XR::InputFeatureType,
}
#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::XR::InputFeatureUsage {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.XR";
    const CLASS_NAME: &'static str = "InputFeatureUsage";
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
#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::XR::InputFeatureUsage {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::XR::InputFeatureUsage {
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
#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::XR::InputFeatureUsage {
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
#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::XR::InputFeatureUsage {
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
#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::XR::InputFeatureUsage {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
impl crate::UnityEngine::XR::InputFeatureUsage {
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_InputFeatureUsage1(
        &mut self,
        other: crate::UnityEngine::XR::InputFeatureUsage,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_internalType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::XR::InputFeatureType> {
        let __cordl_ret: crate::UnityEngine::XR::InputFeatureType = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_internalType",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_name", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::XR::InputFeatureUsage>>
for crate::UnityEngine::XR::InputFeatureUsage {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::XR::InputFeatureUsage> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+XR+InputFeatureUsage")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::XR::InputFeatureUsage>>
for crate::UnityEngine::XR::InputFeatureUsage {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::XR::InputFeatureUsage> {
        todo!()
    }
}
