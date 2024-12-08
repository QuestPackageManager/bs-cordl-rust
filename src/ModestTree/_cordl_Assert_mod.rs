#[cfg(feature = "ModestTree+Assert")]
#[repr(C)]
#[derive(Debug)]
pub struct _cordl_Assert {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ModestTree+Assert")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::_cordl_Assert => "ModestTree"
    ."Assert"
);
#[cfg(feature = "ModestTree+Assert")]
impl std::ops::Deref for crate::ModestTree::_cordl_Assert {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Assert")]
impl std::ops::DerefMut for crate::ModestTree::_cordl_Assert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Assert")]
impl crate::ModestTree::_cordl_Assert {}
#[cfg(feature = "ModestTree+Assert")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::_cordl_Assert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}