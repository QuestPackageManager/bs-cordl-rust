#[cfg(feature = "System+Runtime+Remoting+Metadata+SoapFieldAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SoapFieldAttribute {
    __cordl_parent: crate::System::Runtime::Remoting::Metadata::SoapAttribute,
    pub _elementName: *mut crate::System::String,
    pub _isElement: bool,
}
#[cfg(feature = "System+Runtime+Remoting+Metadata+SoapFieldAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Metadata::SoapFieldAttribute =>
    "System.Runtime.Remoting.Metadata"."SoapFieldAttribute"
);
#[cfg(feature = "System+Runtime+Remoting+Metadata+SoapFieldAttribute")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Metadata::SoapFieldAttribute {
    type Target = crate::System::Runtime::Remoting::Metadata::SoapAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Metadata+SoapFieldAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::Remoting::Metadata::SoapFieldAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Metadata+SoapFieldAttribute")]
impl crate::System::Runtime::Remoting::Metadata::SoapFieldAttribute {
    pub fn IsInteropXmlElement(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsInteropXmlElement", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetReflectionObject(
        &mut self,
        reflectionObject: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetReflectionObject", (reflectionObject))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_XmlElementName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_XmlElementName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Remoting+Metadata+SoapFieldAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Metadata::SoapFieldAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
