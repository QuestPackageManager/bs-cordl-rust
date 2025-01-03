#[cfg(feature = "UnityEngine+Scripting+GarbageCollector")]
#[repr(C)]
#[derive(Debug)]
pub struct GarbageCollector {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Scripting::GarbageCollector =>
    "UnityEngine.Scripting"."GarbageCollector"
);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GarbageCollector_Mode {
    Disabled = 0i32,
    Enabled = 1i32,
    Manual = 2i32,
}
#[cfg(feature = "UnityEngine+Scripting+GarbageCollector+Mode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Scripting::GarbageCollector_Mode =>
    "UnityEngine.Scripting"."GarbageCollector/Mode"
);
