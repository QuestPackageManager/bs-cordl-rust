#[cfg(feature = "CreditsData+ChildCreditsItem")]
#[repr(C)]
#[derive(Debug)]
pub struct CreditsData_ChildCreditsItem {
    __cordl_parent: crate::System::Object,
    pub title: *mut crate::GlobalNamespace::CreditsData_Text,
    pub text: *mut crate::GlobalNamespace::CreditsData_Text,
}
#[cfg(feature = "CreditsData+ChildCreditsItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CreditsData_ChildCreditsItem =>
    ""."CreditsData/ChildCreditsItem"
);
#[cfg(feature = "CreditsData+ChildCreditsItem")]
impl std::ops::Deref for crate::GlobalNamespace::CreditsData_ChildCreditsItem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsData+ChildCreditsItem")]
impl std::ops::DerefMut for crate::GlobalNamespace::CreditsData_ChildCreditsItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsData+ChildCreditsItem")]
impl crate::GlobalNamespace::CreditsData_ChildCreditsItem {
    pub fn HasText(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasText", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasTitle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasTitle", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "CreditsData+ChildCreditsItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CreditsData_ChildCreditsItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "CreditsData")]
#[repr(C)]
#[derive(Debug)]
pub struct CreditsData {
    __cordl_parent: crate::System::Object,
    pub creditsItems: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::CreditsData_RootCreditsItem,
    >,
}
#[cfg(feature = "CreditsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for CreditsData => ""."CreditsData"
);
#[cfg(feature = "CreditsData")]
impl std::ops::Deref for CreditsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsData")]
impl std::ops::DerefMut for CreditsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsData")]
impl CreditsData {
    #[cfg(feature = "CreditsData+TextStyle")]
    pub type TextStyle = crate::GlobalNamespace::CreditsData_TextStyle;
    #[cfg(feature = "CreditsData+Text")]
    pub type Text = crate::GlobalNamespace::CreditsData_Text;
    #[cfg(feature = "CreditsData+RootCreditsItem")]
    pub type RootCreditsItem = crate::GlobalNamespace::CreditsData_RootCreditsItem;
    #[cfg(feature = "CreditsData+ChildCreditsItem")]
    pub type ChildCreditsItem = crate::GlobalNamespace::CreditsData_ChildCreditsItem;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "CreditsData")]
impl quest_hook::libil2cpp::ObjectType for CreditsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "CreditsData+RootCreditsItem")]
#[repr(C)]
#[derive(Debug)]
pub struct CreditsData_RootCreditsItem {
    __cordl_parent: crate::System::Object,
    pub title: *mut crate::GlobalNamespace::CreditsData_Text,
    pub text: *mut crate::GlobalNamespace::CreditsData_Text,
    pub rowCountOverride: i32,
    pub creditsItems: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::CreditsData_ChildCreditsItem,
    >,
}
#[cfg(feature = "CreditsData+RootCreditsItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CreditsData_RootCreditsItem =>
    ""."CreditsData/RootCreditsItem"
);
#[cfg(feature = "CreditsData+RootCreditsItem")]
impl std::ops::Deref for crate::GlobalNamespace::CreditsData_RootCreditsItem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsData+RootCreditsItem")]
impl std::ops::DerefMut for crate::GlobalNamespace::CreditsData_RootCreditsItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsData+RootCreditsItem")]
impl crate::GlobalNamespace::CreditsData_RootCreditsItem {
    pub fn HasRowItems(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasRowItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasText(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasText", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasTitle(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasTitle", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
}
#[cfg(feature = "CreditsData+RootCreditsItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::CreditsData_RootCreditsItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "CreditsData+Text")]
#[repr(C)]
#[derive(Debug)]
pub struct CreditsData_Text {
    __cordl_parent: crate::System::Object,
    pub text: *mut crate::System::String,
    pub localized: bool,
    pub style: crate::GlobalNamespace::CreditsData_TextStyle,
}
#[cfg(feature = "CreditsData+Text")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CreditsData_Text => ""
    ."CreditsData/Text"
);
#[cfg(feature = "CreditsData+Text")]
impl std::ops::Deref for crate::GlobalNamespace::CreditsData_Text {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsData+Text")]
impl std::ops::DerefMut for crate::GlobalNamespace::CreditsData_Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CreditsData+Text")]
impl crate::GlobalNamespace::CreditsData_Text {
    pub fn IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsEmpty", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
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
}
#[cfg(feature = "CreditsData+Text")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::CreditsData_Text {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "CreditsData+TextStyle")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditsData_TextStyle {
    Header = 2i32,
    Normal = 0i32,
    Title = 1i32,
}
#[cfg(feature = "CreditsData+TextStyle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::CreditsData_TextStyle => ""
    ."CreditsData/TextStyle"
);
