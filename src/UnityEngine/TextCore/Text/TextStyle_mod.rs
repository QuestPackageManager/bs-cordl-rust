#[cfg(feature = "UnityEngine+TextCore+Text+TextStyle")]
#[repr(C)]
#[derive(Debug)]
pub struct TextStyle {
    __cordl_parent: crate::System::Object,
    pub m_Name: *mut crate::System::String,
    pub m_HashCode: i32,
    pub m_OpeningDefinition: *mut crate::System::String,
    pub m_ClosingDefinition: *mut crate::System::String,
    pub m_OpeningTagArray: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub m_ClosingTagArray: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub m_OpeningTagUnicodeArray: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub m_ClosingTagUnicodeArray: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextStyle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextStyle =>
    "UnityEngine.TextCore.Text"."TextStyle"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextStyle")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextStyle {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextStyle")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextStyle")]
impl crate::UnityEngine::TextCore::Text::TextStyle {
    pub fn New(
        styleName: *mut crate::System::String,
        styleOpeningDefinition: *mut crate::System::String,
        styleClosingDefinition: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (styleName, styleOpeningDefinition, styleClosingDefinition),
            )?;
        Ok(__cordl_object)
    }
    pub fn RefreshStyle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshStyle", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        styleName: *mut crate::System::String,
        styleOpeningDefinition: *mut crate::System::String,
        styleClosingDefinition: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (styleName, styleOpeningDefinition, styleClosingDefinition),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_hashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_hashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_styleClosingTagArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u32> = __cordl_object
            .invoke("get_styleClosingTagArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_styleOpeningTagArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u32> = __cordl_object
            .invoke("get_styleOpeningTagArray", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextStyle")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::TextStyle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}