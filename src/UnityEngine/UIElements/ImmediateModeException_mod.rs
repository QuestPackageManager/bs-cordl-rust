#[cfg(feature = "UnityEngine+UIElements+ImmediateModeException")]
#[repr(C)]
#[derive(Debug)]
pub struct ImmediateModeException {
    __cordl_parent: crate::System::Exception,
}
#[cfg(feature = "UnityEngine+UIElements+ImmediateModeException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ImmediateModeException
    => "UnityEngine.UIElements"."ImmediateModeException"
);
#[cfg(feature = "UnityEngine+UIElements+ImmediateModeException")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ImmediateModeException {
    type Target = crate::System::Exception;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ImmediateModeException")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ImmediateModeException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ImmediateModeException")]
impl crate::UnityEngine::UIElements::ImmediateModeException {
    pub fn New(
        inner: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (inner))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        inner: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (inner))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+ImmediateModeException")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ImmediateModeException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
