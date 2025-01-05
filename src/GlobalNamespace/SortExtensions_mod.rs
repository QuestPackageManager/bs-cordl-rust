#[cfg(feature = "SortExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SortExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "SortExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SortExtensions => ""
    ."SortExtensions"
);
#[cfg(feature = "SortExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::SortExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SortExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::SortExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SortExtensions")]
impl crate::GlobalNamespace::SortExtensions {
    pub fn InsertSorted<T>(
        list: quest_hook::libil2cpp::Gc<T>,
        item: T,
        getSortIndex: quest_hook::libil2cpp::Gc<T, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InsertSorted", (list, item, getSortIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort<T>(
        list: quest_hook::libil2cpp::Gc<T>,
        getSortIndex: quest_hook::libil2cpp::Gc<T, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (list, getSortIndex))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SortExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SortExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
