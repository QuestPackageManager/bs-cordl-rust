#[cfg(feature = "SelectLevelPackDestination")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectLevelPackDestination {
    __cordl_parent: MenuDestination,
    pub beatmapLevelPack: *mut BeatmapLevelPack,
}
#[cfg(feature = "SelectLevelPackDestination")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SelectLevelPackDestination => ""
    ."SelectLevelPackDestination"
);
#[cfg(feature = "SelectLevelPackDestination")]
impl std::ops::Deref for SelectLevelPackDestination {
    type Target = MenuDestination;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SelectLevelPackDestination")]
impl std::ops::DerefMut for SelectLevelPackDestination {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SelectLevelPackDestination")]
impl SelectLevelPackDestination {
    pub fn _ctor(
        &mut self,
        beatmapLevelPack: *mut BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapLevelPack))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beatmapLevelPack: *mut BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapLevelPack))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SelectLevelPackDestination")]
impl quest_hook::libil2cpp::ObjectType for SelectLevelPackDestination {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
