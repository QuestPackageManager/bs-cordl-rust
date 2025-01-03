#[cfg(feature = "LightGroupGridLayouter")]
#[repr(C)]
#[derive(Debug)]
pub struct LightGroupGridLayouter {
    __cordl_parent: crate::GlobalNamespace::LightGroupSubsystem,
    pub _columns: i32,
    pub _columnStep: crate::UnityEngine::Vector3,
    pub _columnsFromCenter: bool,
    pub _rowStep: crate::UnityEngine::Vector3,
    pub _rowsFromCenter: bool,
    pub _transposeOrder: bool,
    pub _alternateOrder: bool,
    pub _defaultRotation: crate::UnityEngine::Vector3,
}
#[cfg(feature = "LightGroupGridLayouter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightGroupGridLayouter => ""
    ."LightGroupGridLayouter"
);
#[cfg(feature = "LightGroupGridLayouter")]
impl std::ops::Deref for crate::GlobalNamespace::LightGroupGridLayouter {
    type Target = crate::GlobalNamespace::LightGroupSubsystem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupGridLayouter")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightGroupGridLayouter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightGroupGridLayouter")]
impl crate::GlobalNamespace::LightGroupGridLayouter {
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
}
#[cfg(feature = "LightGroupGridLayouter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightGroupGridLayouter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
