#[cfg(feature = "System+Xml+XmlCachedStream")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlCachedStream {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
    pub uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
}
#[cfg(feature = "System+Xml+XmlCachedStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlCachedStream => "System.Xml"
    ."XmlCachedStream"
);
#[cfg(feature = "System+Xml+XmlCachedStream")]
impl std::ops::Deref for crate::System::Xml::XmlCachedStream {
    type Target = quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlCachedStream")]
impl std::ops::DerefMut for crate::System::Xml::XmlCachedStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlCachedStream")]
impl crate::System::Xml::XmlCachedStream {
    pub fn New(
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (uri, stream))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (uri, stream))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlCachedStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlCachedStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
