#[cfg(feature = "TMPro+TMP_StyleSheet")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_StyleSheet {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_StyleList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::TMPro::TMP_Style>,
    >,
    pub m_StyleLookupDictionary: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            *mut crate::TMPro::TMP_Style,
        >,
    >,
}
#[cfg(feature = "TMPro+TMP_StyleSheet")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_StyleSheet => "TMPro"
    ."TMP_StyleSheet"
);
#[cfg(feature = "TMPro+TMP_StyleSheet")]
impl std::ops::Deref for crate::TMPro::TMP_StyleSheet {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_StyleSheet")]
impl std::ops::DerefMut for crate::TMPro::TMP_StyleSheet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_StyleSheet")]
impl crate::TMPro::TMP_StyleSheet {
    pub fn GetStyle_Il2CppString1(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Style>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Style> = __cordl_object
            .invoke("GetStyle", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetStyle_i32_0(
        &mut self,
        hashCode: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Style>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Style> = __cordl_object
            .invoke("GetStyle", (hashCode))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadStyleDictionaryInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadStyleDictionaryInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshStyles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshStyles", ())?;
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
    pub fn get_styles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::TMPro::TMP_Style>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::TMPro::TMP_Style>,
        > = __cordl_object.invoke("get_styles", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_StyleSheet")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_StyleSheet {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
