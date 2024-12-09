#[cfg(feature = "ModestTree+Log")]
#[repr(C)]
#[derive(Debug)]
pub struct Log {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ModestTree+Log")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::Log => "ModestTree"."Log"
);
#[cfg(feature = "ModestTree+Log")]
impl std::ops::Deref for crate::ModestTree::Log {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Log")]
impl std::ops::DerefMut for crate::ModestTree::Log {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Log")]
impl crate::ModestTree::Log {}
#[cfg(feature = "ModestTree+Log")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::Log {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
