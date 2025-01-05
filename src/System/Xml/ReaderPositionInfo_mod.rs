#[cfg(feature = "System+Xml+ReaderPositionInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ReaderPositionInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Xml::PositionInfo>,
    pub lineInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlLineInfo>,
}
#[cfg(feature = "System+Xml+ReaderPositionInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::ReaderPositionInfo => "System.Xml"
    ."ReaderPositionInfo"
);
#[cfg(feature = "System+Xml+ReaderPositionInfo")]
impl std::ops::Deref for crate::System::Xml::ReaderPositionInfo {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Xml::PositionInfo>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+ReaderPositionInfo")]
impl std::ops::DerefMut for crate::System::Xml::ReaderPositionInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+ReaderPositionInfo")]
impl crate::System::Xml::ReaderPositionInfo {
    pub fn HasLineInfo(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasLineInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        lineInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlLineInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lineInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lineInfo: quest_hook::libil2cpp::Gc<crate::System::Xml::IXmlLineInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lineInfo))?;
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
#[cfg(feature = "System+Xml+ReaderPositionInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::ReaderPositionInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
