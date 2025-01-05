#[cfg(feature = "LinkedListExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct LinkedListExtension {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LinkedListExtension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LinkedListExtension => ""
    ."LinkedListExtension"
);
#[cfg(feature = "LinkedListExtension")]
impl std::ops::Deref for crate::GlobalNamespace::LinkedListExtension {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LinkedListExtension")]
impl std::ops::DerefMut for crate::GlobalNamespace::LinkedListExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LinkedListExtension")]
impl crate::GlobalNamespace::LinkedListExtension {
    pub fn Index<T>(
        searchNode: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Index", (searchNode))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LinkedListExtension")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LinkedListExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
