#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlAsyncCheckReaderWithLineInfo {
    __cordl_parent: crate::System::Xml::XmlAsyncCheckReader,
    pub readerAsIXmlLineInfo: quest_hook::libil2cpp::Gc<
        crate::System::Xml::IXmlLineInfo,
    >,
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::XmlAsyncCheckReaderWithLineInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "XmlAsyncCheckReaderWithLineInfo";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfo")]
impl std::ops::Deref for crate::System::Xml::XmlAsyncCheckReaderWithLineInfo {
    type Target = crate::System::Xml::XmlAsyncCheckReader;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfo")]
impl std::ops::DerefMut for crate::System::Xml::XmlAsyncCheckReaderWithLineInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfo")]
impl crate::System::Xml::XmlAsyncCheckReaderWithLineInfo {
    pub fn HasLineInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasLineInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reader))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LineNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNumber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LinePosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LinePosition", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::XmlAsyncCheckReaderWithLineInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfo")]
impl AsRef<crate::System::Xml::IXmlLineInfo>
for crate::System::Xml::XmlAsyncCheckReaderWithLineInfo {
    fn as_ref(&self) -> &crate::System::Xml::IXmlLineInfo {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Xml+XmlAsyncCheckReaderWithLineInfo")]
impl AsMut<crate::System::Xml::IXmlLineInfo>
for crate::System::Xml::XmlAsyncCheckReaderWithLineInfo {
    fn as_mut(&mut self) -> &mut crate::System::Xml::IXmlLineInfo {
        unsafe { std::mem::transmute(self) }
    }
}
