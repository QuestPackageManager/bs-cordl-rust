#[cfg(feature = "NullAllowed")]
#[repr(C)]
#[derive(Debug)]
pub struct NullAllowed {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::PropertyAttribute>,
    pub _context: crate::GlobalNamespace::NullAllowed_Context,
}
#[cfg(feature = "NullAllowed")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NullAllowed => ""."NullAllowed"
);
#[cfg(feature = "NullAllowed")]
impl std::ops::Deref for crate::GlobalNamespace::NullAllowed {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::PropertyAttribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NullAllowed")]
impl std::ops::DerefMut for crate::GlobalNamespace::NullAllowed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NullAllowed")]
impl crate::GlobalNamespace::NullAllowed {
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        context: crate::GlobalNamespace::NullAllowed_Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (context))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NullAllowed")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NullAllowed {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NullAllowed+Context")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NullAllowed_Context {
    #[default]
    Everywhere = 0i32,
    Prefab = 1i32,
}
#[cfg(feature = "NullAllowed+Context")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NullAllowed_Context => ""
    ."NullAllowed/Context"
);
