#[cfg(feature = "NullAllowed+Context")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NullAllowed_Context {
    Everywhere = 0i32,
    Prefab = 1i32,
}
#[cfg(feature = "NullAllowed+Context")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NullAllowed_Context => ""
    ."NullAllowed/Context"
);
#[cfg(feature = "NullAllowed")]
#[repr(C)]
#[derive(Debug)]
pub struct NullAllowed {
    __cordl_parent: crate::UnityEngine::PropertyAttribute,
    pub _context: crate::GlobalNamespace::NullAllowed_Context,
}
#[cfg(feature = "NullAllowed")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for NullAllowed => ""."NullAllowed"
);
#[cfg(feature = "NullAllowed")]
impl std::ops::Deref for NullAllowed {
    type Target = crate::UnityEngine::PropertyAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NullAllowed")]
impl std::ops::DerefMut for NullAllowed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NullAllowed")]
impl NullAllowed {
    #[cfg(feature = "NullAllowed+Context")]
    pub type Context = crate::GlobalNamespace::NullAllowed_Context;
    pub fn IsNullAllowedFor(
        &mut self,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsNullAllowedFor", (context))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (context))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "NullAllowed")]
impl quest_hook::libil2cpp::ObjectType for NullAllowed {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
