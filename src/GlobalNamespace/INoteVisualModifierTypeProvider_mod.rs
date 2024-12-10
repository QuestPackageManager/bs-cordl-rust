#[cfg(feature = "INoteVisualModifierTypeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct INoteVisualModifierTypeProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "INoteVisualModifierTypeProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::INoteVisualModifierTypeProvider
    => ""."INoteVisualModifierTypeProvider"
);
#[cfg(feature = "INoteVisualModifierTypeProvider")]
impl std::ops::Deref for crate::GlobalNamespace::INoteVisualModifierTypeProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "INoteVisualModifierTypeProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::INoteVisualModifierTypeProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "INoteVisualModifierTypeProvider")]
impl crate::GlobalNamespace::INoteVisualModifierTypeProvider {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_noteVisualModifierType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::NoteVisualModifierType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::NoteVisualModifierType = __cordl_object
            .invoke("get_noteVisualModifierType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "INoteVisualModifierTypeProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::INoteVisualModifierTypeProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
