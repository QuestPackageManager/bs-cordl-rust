#[cfg(feature = "SongPackMasksModelSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SongPackMasksModelSO {
    __cordl_parent: PersistentScriptableObject,
    pub _defaultSongPackMaskItems: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub _customSongPackMaskItems: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut SongPackMasksModelItem,
    >,
}
#[cfg(feature = "SongPackMasksModelSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SongPackMasksModelSO => ""."SongPackMasksModelSO"
);
#[cfg(feature = "SongPackMasksModelSO")]
impl std::ops::Deref for SongPackMasksModelSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongPackMasksModelSO")]
impl std::ops::DerefMut for SongPackMasksModelSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongPackMasksModelSO")]
impl SongPackMasksModelSO {
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
    pub fn get_defaultSongPackMaskItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_defaultSongPackMaskItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_customSongPackMaskItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut SongPackMasksModelItem>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut SongPackMasksModelItem,
        > = __cordl_object.invoke("get_customSongPackMaskItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SongPackMasksModelSO")]
impl quest_hook::libil2cpp::ObjectType for SongPackMasksModelSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
