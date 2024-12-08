#[cfg(feature = "INoteMovementProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct INoteMovementProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "INoteMovementProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::INoteMovementProvider => ""
    ."INoteMovementProvider"
);
#[cfg(feature = "INoteMovementProvider")]
impl std::ops::Deref for crate::GlobalNamespace::INoteMovementProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "INoteMovementProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::INoteMovementProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "INoteMovementProvider")]
impl crate::GlobalNamespace::INoteMovementProvider {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_noteMovement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::NoteMovement> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::NoteMovement = __cordl_object
            .invoke("get_noteMovement", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "INoteMovementProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::INoteMovementProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
