#[cfg(feature = "UnityEngine+Scripting+GarbageCollector")]
#[repr(C)]
#[derive(Debug)]
pub struct GarbageCollector {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Scripting::GarbageCollector {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Scripting";
    const CLASS_NAME: &'static str = "GarbageCollector";
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
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector")]
impl std::ops::Deref for crate::UnityEngine::Scripting::GarbageCollector {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector")]
impl std::ops::DerefMut for crate::UnityEngine::Scripting::GarbageCollector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector")]
impl crate::UnityEngine::Scripting::GarbageCollector {
    #[cfg(feature = "UnityEngine+Scripting+GarbageCollector+Mode")]
    pub type Mode = crate::UnityEngine::Scripting::GarbageCollector_Mode;
    pub fn GetMode() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Scripting::GarbageCollector_Mode,
    > {
        let __cordl_ret: crate::UnityEngine::Scripting::GarbageCollector_Mode = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMode(
        mode: crate::UnityEngine::Scripting::GarbageCollector_Mode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMode", (mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_GCMode(
        value: crate::UnityEngine::Scripting::GarbageCollector_Mode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_GCMode", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Scripting::GarbageCollector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector+Mode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GarbageCollector_Mode {
    #[default]
    Disabled = 0i32,
    Enabled = 1i32,
    Manual = 2i32,
}
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector+Mode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Scripting::GarbageCollector_Mode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Scripting";
    const CLASS_NAME: &'static str = "Mode";
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
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector+Mode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Scripting::GarbageCollector_Mode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector+Mode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Scripting::GarbageCollector_Mode {
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
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector+Mode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Scripting::GarbageCollector_Mode {
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
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector+Mode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Scripting::GarbageCollector_Mode {
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
