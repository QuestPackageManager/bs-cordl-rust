#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeFieldAttribute {
    __cordl_parent: crate::UnityEngine::PropertyAttribute,
    pub _useEditMode_k__BackingField: crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode,
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::TimeFieldAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "TimeFieldAttribute";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TimeFieldAttribute {
    type Target = crate::UnityEngine::PropertyAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TimeFieldAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute")]
impl crate::UnityEngine::Timeline::TimeFieldAttribute {
    #[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute+UseEditMode")]
    pub type UseEditMode = crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode;
    pub fn New(
        useEditMode: crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (useEditMode))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        useEditMode: crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (useEditMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useEditMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode = __cordl_object
            .invoke("get_useEditMode", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TimeFieldAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute+UseEditMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimeFieldAttribute_UseEditMode {
    #[default]
    ApplyEditMode = 1i32,
    None = 0i32,
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute+UseEditMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Timeline";
    const CLASS_NAME: &'static str = "UseEditMode";
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
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute+UseEditMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute+UseEditMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode {
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
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute+UseEditMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode {
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
#[cfg(feature = "UnityEngine+Timeline+TimeFieldAttribute+UseEditMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Timeline::TimeFieldAttribute_UseEditMode {
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
