#[cfg(feature = "System+Runtime+Remoting+Messaging+Header")]
#[repr(C)]
#[derive(Debug)]
pub struct Header {
    __cordl_parent: crate::System::Object,
    pub HeaderNamespace: *mut crate::System::String,
    pub MustUnderstand: bool,
    pub Name: *mut crate::System::String,
    pub Value: *mut crate::System::Object,
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+Header")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::Remoting::Messaging::Header =>
    "System.Runtime.Remoting.Messaging"."Header"
);
#[cfg(feature = "System+Runtime+Remoting+Messaging+Header")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Messaging::Header {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+Header")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Messaging::Header {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Messaging+Header")]
impl crate::System::Runtime::Remoting::Messaging::Header {}
#[cfg(feature = "System+Runtime+Remoting+Messaging+Header")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Messaging::Header {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
