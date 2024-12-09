#[cfg(feature = "ModestTree+Util+ValuePair")]
#[repr(C)]
#[derive(Debug)]
pub struct ValuePair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ModestTree+Util+ValuePair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::Util::ValuePair => "ModestTree.Util"
    ."ValuePair"
);
#[cfg(feature = "ModestTree+Util+ValuePair")]
impl std::ops::Deref for crate::ModestTree::Util::ValuePair {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Util+ValuePair")]
impl std::ops::DerefMut for crate::ModestTree::Util::ValuePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Util+ValuePair")]
impl crate::ModestTree::Util::ValuePair {}
#[cfg(feature = "ModestTree+Util+ValuePair")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::Util::ValuePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
