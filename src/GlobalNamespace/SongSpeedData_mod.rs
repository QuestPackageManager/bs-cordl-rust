#[cfg(feature = "SongSpeedData")]
#[repr(C)]
#[derive(Debug)]
pub struct SongSpeedData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub speedMul: f32,
}
#[cfg(feature = "SongSpeedData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SongSpeedData => ""
    ."SongSpeedData"
);
#[cfg(feature = "SongSpeedData")]
impl std::ops::Deref for crate::GlobalNamespace::SongSpeedData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongSpeedData")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongSpeedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongSpeedData")]
impl crate::GlobalNamespace::SongSpeedData {
    pub fn New(
        speedMul: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (speedMul))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        speedMul: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (speedMul))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SongSpeedData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SongSpeedData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
