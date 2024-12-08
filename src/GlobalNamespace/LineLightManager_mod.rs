#[cfg(feature = "LineLightManager")]
#[repr(C)]
#[derive(Debug)]
pub struct LineLightManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _points: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    pub _dirs: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    pub _dirLengths: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub _colors: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
}
#[cfg(feature = "LineLightManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LineLightManager => ""
    ."LineLightManager"
);
#[cfg(feature = "LineLightManager")]
impl std::ops::Deref for crate::GlobalNamespace::LineLightManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LineLightManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::LineLightManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LineLightManager")]
impl crate::GlobalNamespace::LineLightManager {
    pub const kMaxNumberOfLights: i32 = 4i32;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LineLightManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LineLightManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
