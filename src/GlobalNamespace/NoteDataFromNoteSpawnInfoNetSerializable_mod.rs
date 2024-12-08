#[cfg(feature = "NoteDataFromNoteSpawnInfoNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct NoteDataFromNoteSpawnInfoNetSerializable {
    __cordl_parent: crate::GlobalNamespace::NoteData,
}
#[cfg(feature = "NoteDataFromNoteSpawnInfoNetSerializable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NoteDataFromNoteSpawnInfoNetSerializable => ""
    ."NoteDataFromNoteSpawnInfoNetSerializable"
);
#[cfg(feature = "NoteDataFromNoteSpawnInfoNetSerializable")]
impl std::ops::Deref
for crate::GlobalNamespace::NoteDataFromNoteSpawnInfoNetSerializable {
    type Target = crate::GlobalNamespace::NoteData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDataFromNoteSpawnInfoNetSerializable")]
impl std::ops::DerefMut
for crate::GlobalNamespace::NoteDataFromNoteSpawnInfoNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoteDataFromNoteSpawnInfoNetSerializable")]
impl crate::GlobalNamespace::NoteDataFromNoteSpawnInfoNetSerializable {
    pub fn New(
        noteSpawnInfo: *mut crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (noteSpawnInfo))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        noteSpawnInfo: *mut crate::GlobalNamespace::NoteSpawnInfoNetSerializable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (noteSpawnInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "NoteDataFromNoteSpawnInfoNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoteDataFromNoteSpawnInfoNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
