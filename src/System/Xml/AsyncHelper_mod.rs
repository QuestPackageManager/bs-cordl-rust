#[cfg(feature = "System+Xml+AsyncHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Xml+AsyncHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::AsyncHelper => "System.Xml"
    ."AsyncHelper"
);
#[cfg(feature = "System+Xml+AsyncHelper")]
impl std::ops::Deref for crate::System::Xml::AsyncHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+AsyncHelper")]
impl std::ops::DerefMut for crate::System::Xml::AsyncHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+AsyncHelper")]
impl crate::System::Xml::AsyncHelper {}
#[cfg(feature = "System+Xml+AsyncHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::AsyncHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
