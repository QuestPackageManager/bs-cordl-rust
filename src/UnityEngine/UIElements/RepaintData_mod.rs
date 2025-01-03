#[cfg(feature = "UnityEngine+UIElements+RepaintData")]
#[repr(C)]
#[derive(Debug)]
pub struct RepaintData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _currentOffset_k__BackingField: crate::UnityEngine::Matrix4x4,
    pub _mousePosition_k__BackingField: crate::UnityEngine::Vector2,
    pub _currentWorldClip_k__BackingField: crate::UnityEngine::Rect,
    pub _repaintEvent_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Event,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+RepaintData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::RepaintData =>
    "UnityEngine.UIElements"."RepaintData"
);
#[cfg(feature = "UnityEngine+UIElements+RepaintData")]
impl std::ops::Deref for crate::UnityEngine::UIElements::RepaintData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RepaintData")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::RepaintData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+RepaintData")]
impl crate::UnityEngine::UIElements::RepaintData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentOffset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("get_currentOffset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentWorldClip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("get_currentWorldClip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_repaintEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event> = __cordl_object
            .invoke("get_repaintEvent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_repaintEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Event>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_repaintEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+RepaintData")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::RepaintData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
