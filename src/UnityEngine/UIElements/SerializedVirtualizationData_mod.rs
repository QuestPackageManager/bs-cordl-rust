#[cfg(feature = "UnityEngine+UIElements+SerializedVirtualizationData")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializedVirtualizationData {
    __cordl_parent: crate::System::Object,
    pub scrollOffset: crate::UnityEngine::Vector2,
    pub firstVisibleIndex: i32,
    pub contentPadding: f32,
    pub contentHeight: f32,
    pub anchoredItemIndex: i32,
    pub anchorOffset: f32,
}
#[cfg(feature = "UnityEngine+UIElements+SerializedVirtualizationData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::SerializedVirtualizationData => "UnityEngine.UIElements"
    ."SerializedVirtualizationData"
);
#[cfg(feature = "UnityEngine+UIElements+SerializedVirtualizationData")]
impl std::ops::Deref for crate::UnityEngine::UIElements::SerializedVirtualizationData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SerializedVirtualizationData")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::SerializedVirtualizationData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+SerializedVirtualizationData")]
impl crate::UnityEngine::UIElements::SerializedVirtualizationData {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "UnityEngine+UIElements+SerializedVirtualizationData")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::SerializedVirtualizationData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
