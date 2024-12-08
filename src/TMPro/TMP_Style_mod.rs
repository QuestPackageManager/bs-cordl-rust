#[cfg(feature = "TMPro+TMP_Style")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_Style {
    __cordl_parent: crate::System::Object,
    pub m_Name: *mut crate::System::String,
    pub m_HashCode: i32,
    pub m_OpeningDefinition: *mut crate::System::String,
    pub m_ClosingDefinition: *mut crate::System::String,
    pub m_OpeningTagArray: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub m_ClosingTagArray: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub m_OpeningTagUnicodeArray: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
    pub m_ClosingTagUnicodeArray: *mut quest_hook::libil2cpp::Il2CppArray<u32>,
}
#[cfg(feature = "TMPro+TMP_Style")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_Style => "TMPro"."TMP_Style"
);
#[cfg(feature = "TMPro+TMP_Style")]
impl std::ops::Deref for crate::TMPro::TMP_Style {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Style")]
impl std::ops::DerefMut for crate::TMPro::TMP_Style {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_Style")]
impl crate::TMPro::TMP_Style {
    pub fn set_name(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_name", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_hashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_hashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_hashCode(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hashCode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_styleOpeningDefinition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_styleOpeningDefinition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_styleOpeningTagArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("get_styleOpeningTagArray", ())?;
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
    pub fn get_styleClosingDefinition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_styleClosingDefinition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_name", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_styleClosingTagArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("get_styleClosingTagArray", ())?;
        Ok(__cordl_ret)
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
    pub fn New(
        styleName: *mut crate::System::String,
        styleOpeningDefinition: *mut crate::System::String,
        styleClosingDefinition: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (styleName, styleOpeningDefinition, styleClosingDefinition),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TMPro+TMP_Style")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_Style {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
