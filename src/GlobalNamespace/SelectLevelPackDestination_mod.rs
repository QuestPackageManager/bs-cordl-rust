#[cfg(feature = "SelectLevelPackDestination")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectLevelPackDestination {
    __cordl_parent: crate::GlobalNamespace::MenuDestination,
    pub beatmapLevelPack: *mut crate::GlobalNamespace::BeatmapLevelPack,
}
#[cfg(feature = "SelectLevelPackDestination")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SelectLevelPackDestination =>
    ""."SelectLevelPackDestination"
);
#[cfg(feature = "SelectLevelPackDestination")]
impl std::ops::Deref for crate::GlobalNamespace::SelectLevelPackDestination {
    type Target = crate::GlobalNamespace::MenuDestination;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectLevelPackDestination")]
impl std::ops::DerefMut for crate::GlobalNamespace::SelectLevelPackDestination {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectLevelPackDestination")]
impl crate::GlobalNamespace::SelectLevelPackDestination {
    pub fn New(
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapLevelPack))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapLevelPack))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SelectLevelPackDestination")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SelectLevelPackDestination {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
