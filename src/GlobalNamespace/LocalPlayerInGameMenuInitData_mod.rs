#[cfg(feature = "LocalPlayerInGameMenuInitData")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalPlayerInGameMenuInitData {
    __cordl_parent: crate::System::Object,
    pub beatmapKey: BeatmapKey,
    pub hasSong: bool,
}
#[cfg(feature = "LocalPlayerInGameMenuInitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LocalPlayerInGameMenuInitData => ""
    ."LocalPlayerInGameMenuInitData"
);
#[cfg(feature = "LocalPlayerInGameMenuInitData")]
impl std::ops::Deref for LocalPlayerInGameMenuInitData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalPlayerInGameMenuInitData")]
impl std::ops::DerefMut for LocalPlayerInGameMenuInitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalPlayerInGameMenuInitData")]
impl LocalPlayerInGameMenuInitData {
    pub fn New(
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        hasSong: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapKey, hasSong))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        hasSong: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapKey, hasSong))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LocalPlayerInGameMenuInitData")]
impl quest_hook::libil2cpp::ObjectType for LocalPlayerInGameMenuInitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}