#[cfg(feature = "UnityEngine+TextCore+Text+TextStyle")]
#[repr(C)]
#[derive(Debug)]
pub struct TextStyle {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_HashCode: i32,
    pub m_OpeningDefinition: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_ClosingDefinition: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub m_OpeningTagArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u32>,
    >,
    pub m_ClosingTagArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u32>,
    >,
    pub m_OpeningTagUnicodeArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u32>,
    >,
    pub m_ClosingTagUnicodeArray: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u32>,
    >,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextStyle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::Text::TextStyle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.Text";
    const CLASS_NAME: &'static str = "TextStyle";
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
#[cfg(feature = "UnityEngine+TextCore+Text+TextStyle")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextStyle {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleOpeningDefinition: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        styleClosingDefinition: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (styleName, styleOpeningDefinition, styleClosingDefinition),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshStyle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshStyle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleOpeningDefinition: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        styleClosingDefinition: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (styleName, styleOpeningDefinition, styleClosingDefinition),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_hashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_styleClosingTagArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = __cordl_object.invoke("get_styleClosingTagArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_styleOpeningTagArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u32>,
        > = __cordl_object.invoke("get_styleOpeningTagArray", ())?;
        Ok(__cordl_ret.into())
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
