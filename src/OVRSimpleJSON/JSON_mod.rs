#[cfg(feature = "OVRSimpleJSON+JSON")]
#[repr(C)]
#[derive(Debug)]
pub struct JSON {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRSimpleJSON+JSON")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVRSimpleJSON::JSON => "OVRSimpleJSON"."JSON"
);
#[cfg(feature = "OVRSimpleJSON+JSON")]
impl std::ops::Deref for crate::OVRSimpleJSON::JSON {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSimpleJSON+JSON")]
impl std::ops::DerefMut for crate::OVRSimpleJSON::JSON {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRSimpleJSON+JSON")]
impl crate::OVRSimpleJSON::JSON {}
#[cfg(feature = "OVRSimpleJSON+JSON")]
impl quest_hook::libil2cpp::ObjectType for crate::OVRSimpleJSON::JSON {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
