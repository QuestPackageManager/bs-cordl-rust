#[cfg(feature = "ENet+ArrayPool")]
#[repr(C)]
#[derive(Debug)]
pub struct ArrayPool {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ENet+ArrayPool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ENet::ArrayPool => "ENet"."ArrayPool"
);
#[cfg(feature = "ENet+ArrayPool")]
impl std::ops::Deref for crate::ENet::ArrayPool {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+ArrayPool")]
impl std::ops::DerefMut for crate::ENet::ArrayPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ENet+ArrayPool")]
impl crate::ENet::ArrayPool {}
#[cfg(feature = "ENet+ArrayPool")]
impl quest_hook::libil2cpp::ObjectType for crate::ENet::ArrayPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}