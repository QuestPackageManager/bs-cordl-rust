#[cfg(feature = "TMPro+TMP_FontUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TMP_FontUtilities {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "TMPro+TMP_FontUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TMP_FontUtilities => "TMPro"
    ."TMP_FontUtilities"
);
#[cfg(feature = "TMPro+TMP_FontUtilities")]
impl std::ops::Deref for crate::TMPro::TMP_FontUtilities {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontUtilities")]
impl std::ops::DerefMut for crate::TMPro::TMP_FontUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+TMP_FontUtilities")]
impl crate::TMPro::TMP_FontUtilities {
    pub fn SearchForCharacterInternal_Gc_u32_ByRefMut0(
        font: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        unicode: u32,
        character: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SearchForCharacterInternal", (font, unicode, character))?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForCharacterInternal_Gc_u32_ByRefMut1(
        fonts: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        >,
        unicode: u32,
        character: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SearchForCharacterInternal", (fonts, unicode, character))?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForCharacter_Gc_u32_ByRefMut0(
        font: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        unicode: u32,
        character: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SearchForCharacter", (font, unicode, character))?;
        Ok(__cordl_ret.into())
    }
    pub fn SearchForCharacter_Gc_u32_ByRefMut1(
        fonts: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
        >,
        unicode: u32,
        character: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::TMPro::TMP_Character>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::TMPro::TMP_FontAsset> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SearchForCharacter", (fonts, unicode, character))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TMPro+TMP_FontUtilities")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::TMP_FontUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
