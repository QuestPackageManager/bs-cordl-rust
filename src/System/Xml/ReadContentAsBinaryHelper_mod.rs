#[cfg(feature = "System+Xml+ReadContentAsBinaryHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct ReadContentAsBinaryHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub reader: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlReader>,
    pub state: crate::System::Xml::ReadContentAsBinaryHelper_State,
    pub valueOffset: i32,
    pub isEnd: bool,
}
#[cfg(feature = "System+Xml+ReadContentAsBinaryHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::ReadContentAsBinaryHelper =>
    "System.Xml"."ReadContentAsBinaryHelper"
);
#[cfg(feature = "System+Xml+ReadContentAsBinaryHelper")]
impl std::ops::Deref for crate::System::Xml::ReadContentAsBinaryHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+ReadContentAsBinaryHelper")]
impl std::ops::DerefMut for crate::System::Xml::ReadContentAsBinaryHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+ReadContentAsBinaryHelper")]
impl crate::System::Xml::ReadContentAsBinaryHelper {
    #[cfg(feature = "System+Xml+ReadContentAsBinaryHelper+State")]
    pub type State = crate::System::Xml::ReadContentAsBinaryHelper_State;
    pub fn Finish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finish", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToNextContentNode(
        &mut self,
        moveIfOnContentNode: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MoveToNextContentNode", (moveIfOnContentNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+ReadContentAsBinaryHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::ReadContentAsBinaryHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+ReadContentAsBinaryHelper+State")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReadContentAsBinaryHelper_State {
    #[default]
    InReadContent = 1i32,
    InReadElementContent = 2i32,
    None = 0i32,
}
#[cfg(feature = "System+Xml+ReadContentAsBinaryHelper+State")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::ReadContentAsBinaryHelper_State =>
    "System.Xml"."ReadContentAsBinaryHelper/State"
);
