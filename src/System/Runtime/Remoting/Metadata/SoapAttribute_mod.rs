#[cfg(feature = "System+Runtime+Remoting+Metadata+SoapAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SoapAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _useAttribute: bool,
    pub ProtXmlNamespace: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub ReflectInfo: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Runtime+Remoting+Metadata+SoapAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Remoting::Metadata::SoapAttribute =>
    "System.Runtime.Remoting.Metadata"."SoapAttribute"
);
#[cfg(feature = "System+Runtime+Remoting+Metadata+SoapAttribute")]
impl std::ops::Deref for crate::System::Runtime::Remoting::Metadata::SoapAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Metadata+SoapAttribute")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::Metadata::SoapAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+Metadata+SoapAttribute")]
impl crate::System::Runtime::Remoting::Metadata::SoapAttribute {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetReflectionObject(
        &mut self,
        reflectionObject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetReflectionObject", (reflectionObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UseAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XmlNamespace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_XmlNamespace", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+Metadata+SoapAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::Metadata::SoapAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
