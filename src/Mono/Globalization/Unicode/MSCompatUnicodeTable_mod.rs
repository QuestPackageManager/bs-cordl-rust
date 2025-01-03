#[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable")]
#[repr(C)]
#[derive(Debug)]
pub struct MSCompatUnicodeTable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Globalization::Unicode::MSCompatUnicodeTable => "Mono.Globalization.Unicode"
    ."MSCompatUnicodeTable"
);
#[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::MSCompatUnicodeTable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable")]
impl std::ops::DerefMut for crate::Mono::Globalization::Unicode::MSCompatUnicodeTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable")]
impl crate::Mono::Globalization::Unicode::MSCompatUnicodeTable {
    #[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable+__c")]
    pub type __c = crate::Mono::Globalization::Unicode::MSCompatUnicodeTable___c;
    pub fn BuildTailoringTables(
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        t: quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::TailoringInfo>,
        contractions: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Mono::Globalization::Unicode::Contraction,
            >,
        >,
        diacriticals: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Mono::Globalization::Unicode::Level2Map,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuildTailoringTables", (culture, t, contractions, diacriticals))?;
        Ok(__cordl_ret.into())
    }
    pub fn Category(cp: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Category", (cp))?;
        Ok(__cordl_ret.into())
    }
    pub fn FillCJK(
        culture: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cjkIndexer: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Mono::Globalization::Unicode::CodePointIndexer,
        >,
        catTable: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        lv1Table: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        lv2Indexer: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Mono::Globalization::Unicode::CodePointIndexer,
        >,
        lv2Table: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FillCJK",
                (culture, cjkIndexer, catTable, lv1Table, lv2Indexer, lv2Table),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FillCJKCore(
        culture: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cjkIndexer: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Mono::Globalization::Unicode::CodePointIndexer,
        >,
        catTable: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        lv1Table: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        cjkLv2Indexer: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Mono::Globalization::Unicode::CodePointIndexer,
        >,
        lv2Table: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FillCJKCore",
                (culture, cjkIndexer, catTable, lv1Table, cjkLv2Indexer, lv2Table),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResource(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetResource", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTailoringInfo(
        lcid: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::TailoringInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Globalization::Unicode::TailoringInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTailoringInfo", (lcid))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasSpecialWeight(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasSpecialWeight", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHalfWidthKana(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHalfWidthKana", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHiragana(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHiragana", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIgnorable(cp: i32, flag: u8) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsIgnorable", (cp, flag))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIgnorableNonSpacing(cp: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsIgnorableNonSpacing", (cp))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsJapaneseSmallLetter(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsJapaneseSmallLetter", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn Level1(cp: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Level1", (cp))?;
        Ok(__cordl_ret.into())
    }
    pub fn Level2(cp: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Level2", (cp))?;
        Ok(__cordl_ret.into())
    }
    pub fn Level3(cp: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Level3", (cp))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCJKReferences(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cjkIndexer: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Mono::Globalization::Unicode::CodePointIndexer,
        >,
        catTable: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        lv1Table: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
        lv2Indexer: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Mono::Globalization::Unicode::CodePointIndexer,
        >,
        lv2Table: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetCJKReferences",
                (name, cjkIndexer, catTable, lv1Table, lv2Indexer, lv2Table),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToKanaTypeInsensitive(i: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToKanaTypeInsensitive", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToWidthCompat(i: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToWidthCompat", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn UInt32FromBytePtr(
        raw: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        idx: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UInt32FromBytePtr", (raw, idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReady() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_IsReady", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+MSCompatUnicodeTable")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Globalization::Unicode::MSCompatUnicodeTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
