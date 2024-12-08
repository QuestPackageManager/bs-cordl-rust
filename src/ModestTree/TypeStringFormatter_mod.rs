#[cfg(feature = "ModestTree+TypeStringFormatter")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeStringFormatter {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ModestTree+TypeStringFormatter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::TypeStringFormatter => "ModestTree"
    ."TypeStringFormatter"
);
#[cfg(feature = "ModestTree+TypeStringFormatter")]
impl std::ops::Deref for crate::ModestTree::TypeStringFormatter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+TypeStringFormatter")]
impl std::ops::DerefMut for crate::ModestTree::TypeStringFormatter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+TypeStringFormatter")]
impl crate::ModestTree::TypeStringFormatter {
    #[cfg(feature = "ModestTree+TypeStringFormatter+__c")]
    pub type __c = crate::ModestTree::TypeStringFormatter___c;
}
#[cfg(feature = "ModestTree+TypeStringFormatter")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::TypeStringFormatter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
