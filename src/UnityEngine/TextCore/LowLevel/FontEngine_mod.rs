#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct FontEngine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::TextCore::LowLevel::FontEngine {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.LowLevel";
    const CLASS_NAME: &'static str = "FontEngine";
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
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
impl std::ops::Deref for crate::UnityEngine::TextCore::LowLevel::FontEngine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::LowLevel::FontEngine {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
impl crate::UnityEngine::TextCore::LowLevel::FontEngine {
    pub fn GenericListToMarshallingArray<T>(
        srcList: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        >,
        dstArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::List_1<T>,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<T>,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("GenericListToMarshallingArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenericListToMarshallingArray", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (srcList, dstArray))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceInfo() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::FaceInfo,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::TextCore::FaceInfo,
                        0usize,
                    >("GetFaceInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFaceInfo", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::FaceInfo = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceInfo_Internal(
        faceInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::TextCore::FaceInfo>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::TextCore::FaceInfo,
                        >),
                        i32,
                        1usize,
                    >("GetFaceInfo_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFaceInfo_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (faceInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlyphIndex(unicode: u32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32), u32, 1usize>("GetGlyphIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGlyphIndex", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (unicode))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlyphPairAdjustmentRecords(
        glyphIndexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u32>,
        >,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<u32>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
                            >,
                        >,
                        2usize,
                    >("GetGlyphPairAdjustmentRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGlyphPairAdjustmentRecords", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        > = unsafe { method.invoke_unchecked((), (glyphIndexes, recordCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlyphPairAdjustmentTable(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u32>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
                            >,
                        >,
                        1usize,
                    >("GetGlyphPairAdjustmentTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGlyphPairAdjustmentTable", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        > = unsafe { method.invoke_unchecked((), (glyphIndexes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPairAdjustmentRecordsFromMarshallingArray(
        glyphPairAdjustmentRecords: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
                                >,
                            >,
                        >),
                        i32,
                        1usize,
                    >("GetPairAdjustmentRecordsFromMarshallingArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPairAdjustmentRecordsFromMarshallingArray", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (glyphPairAdjustmentRecords))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeFontEngine() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
                        0usize,
                    >("InitializeFontEngine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitializeFontEngine", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitializeFontEngine_Internal() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        i32,
                        0usize,
                    >("InitializeFontEngine_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitializeFontEngine_Internal", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Font_i32_1(
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        pointSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>, i32),
                        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
                        2usize,
                    >("LoadFontFace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadFontFace", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError = unsafe {
            method.invoke_unchecked((), (font, pointSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Font_i32_i32_2(
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        pointSize: i32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>, i32, i32),
                        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
                        3usize,
                    >("LoadFontFace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadFontFace", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError = unsafe {
            method.invoke_unchecked((), (font, pointSize, faceIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Il2CppString_Il2CppString_i32_3(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pointSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
                        3usize,
                    >("LoadFontFace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadFontFace", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError = unsafe {
            method.invoke_unchecked((), (familyName, styleName, pointSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Il2CppString_i32_i32_0(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pointSize: i32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
                        3usize,
                    >("LoadFontFace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadFontFace", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError = unsafe {
            method.invoke_unchecked((), (filePath, pointSize, faceIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_And_FaceIndex_Internal(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pointSize: i32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("LoadFontFace_With_Size_And_FaceIndex_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadFontFace_With_Size_And_FaceIndex_Internal", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (filePath, pointSize, faceIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_FromFont_Internal(
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        pointSize: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>, i32),
                        i32,
                        2usize,
                    >("LoadFontFace_With_Size_FromFont_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadFontFace_With_Size_FromFont_Internal", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (font, pointSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_and_FaceIndex_FromFont_Internal(
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        pointSize: i32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>, i32, i32),
                        i32,
                        3usize,
                    >("LoadFontFace_With_Size_and_FaceIndex_FromFont_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadFontFace_With_Size_and_FaceIndex_FromFont_Internal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (font, pointSize, faceIndex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_by_FamilyName_and_StyleName_Internal(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pointSize: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                        ),
                        i32,
                        3usize,
                    >("LoadFontFace_With_Size_by_FamilyName_and_StyleName_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadFontFace_With_Size_by_FamilyName_and_StyleName_Internal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (familyName, styleName, pointSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PopulatePairAdjustmentRecordMarshallingArray_from_KernTable(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u32>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        i32,
                        2usize,
                    >("PopulatePairAdjustmentRecordMarshallingArray_from_KernTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PopulatePairAdjustmentRecordMarshallingArray_from_KernTable",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (glyphIndexes, recordCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ResetAtlasTexture(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ResetAtlasTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResetAtlasTexture", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (texture))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetMarshallingArraySize<T>(
        marshallingArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        >,
        recordCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<T>,
                                >,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetMarshallingArraySize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetMarshallingArraySize", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (marshallingArray, recordCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetTextureUploadMode(
        shouldUploadImmediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetTextureUploadMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetTextureUploadMode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (shouldUploadImmediately))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryAddGlyphToTexture(
        glyphIndex: u32,
        padding: i32,
        packingMode: crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
        freeGlyphRects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::GlyphRect,
            >,
        >,
        usedGlyphRects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::GlyphRect,
            >,
        >,
        renderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        glyph: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u32,
                            i32,
                            crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::TextCore::GlyphRect,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::TextCore::GlyphRect,
                                >,
                            >,
                            crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Glyph,
                                >,
                            >,
                        ),
                        bool,
                        8usize,
                    >("TryAddGlyphToTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryAddGlyphToTexture", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        glyphIndex,
                        padding,
                        packingMode,
                        freeGlyphRects,
                        usedGlyphRects,
                        renderMode,
                        texture,
                        glyph,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryAddGlyphToTexture_Internal(
        glyphIndex: u32,
        padding: i32,
        packingMode: crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
        freeGlyphRects: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::GlyphRect,
                >,
            >,
        >,
        freeGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        usedGlyphRects: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::GlyphRect,
                >,
            >,
        >,
        usedGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        renderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        glyph: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u32,
                            i32,
                            crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::GlyphRect,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::GlyphRect,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
                            >,
                        ),
                        bool,
                        10usize,
                    >("TryAddGlyphToTexture_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryAddGlyphToTexture_Internal", 10usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        glyphIndex,
                        padding,
                        packingMode,
                        freeGlyphRects,
                        freeGlyphRectCount,
                        usedGlyphRects,
                        usedGlyphRectCount,
                        renderMode,
                        texture,
                        glyph,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryAddGlyphsToTexture(
        glyphIndexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u32>,
        >,
        padding: i32,
        packingMode: crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
        freeGlyphRects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::GlyphRect,
            >,
        >,
        usedGlyphRects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::TextCore::GlyphRect,
            >,
        >,
        renderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        glyphs: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<u32>,
                            >,
                            i32,
                            crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::TextCore::GlyphRect,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::UnityEngine::TextCore::GlyphRect,
                                >,
                            >,
                            crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        quest_hook::libil2cpp::Gc<
                                            crate::UnityEngine::TextCore::Glyph,
                                        >,
                                    >,
                                >,
                            >,
                        ),
                        bool,
                        8usize,
                    >("TryAddGlyphsToTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryAddGlyphsToTexture", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        glyphIndexes,
                        padding,
                        packingMode,
                        freeGlyphRects,
                        usedGlyphRects,
                        renderMode,
                        texture,
                        glyphs,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryAddGlyphsToTexture_Internal(
        glyphIndex: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        padding: i32,
        packingMode: crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
        freeGlyphRects: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::GlyphRect,
                >,
            >,
        >,
        freeGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        usedGlyphRects: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::GlyphRect,
                >,
            >,
        >,
        usedGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        renderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        glyphs: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
                >,
            >,
        >,
        glyphCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u32>,
                            >,
                            i32,
                            crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::GlyphRect,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::GlyphRect,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppArray<
                                        crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
                                    >,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        11usize,
                    >("TryAddGlyphsToTexture_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryAddGlyphsToTexture_Internal", 11usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        glyphIndex,
                        padding,
                        packingMode,
                        freeGlyphRects,
                        freeGlyphRectCount,
                        usedGlyphRects,
                        usedGlyphRectCount,
                        renderMode,
                        texture,
                        glyphs,
                        glyphCount,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetGlyphWithIndexValue(
        glyphIndex: u32,
        flags: crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
        glyph: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u32,
                            crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Glyph,
                                >,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetGlyphWithIndexValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetGlyphWithIndexValue", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (glyphIndex, flags, glyph))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetGlyphWithIndexValue_Internal(
        glyphIndex: u32,
        loadFlags: crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
        glyphStruct: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u32,
                            crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetGlyphWithIndexValue_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetGlyphWithIndexValue_Internal", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (glyphIndex, loadFlags, glyphStruct))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetGlyphWithUnicodeValue(
        unicode: u32,
        flags: crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
        glyph: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u32,
                            crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::TextCore::Glyph,
                                >,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetGlyphWithUnicodeValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetGlyphWithUnicodeValue", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (unicode, flags, glyph))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetGlyphWithUnicodeValue_Internal(
        unicode: u32,
        loadFlags: crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
        glyphStruct: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            u32,
                            crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetGlyphWithUnicodeValue_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetGlyphWithUnicodeValue_Internal", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (unicode, loadFlags, glyphStruct))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSystemFontReference(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fontRef: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::FontReference,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::TextCore::LowLevel::FontReference,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetSystemFontReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetSystemFontReference", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (familyName, styleName, fontRef))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSystemFontReference_Internal(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fontRef: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::FontReference,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::TextCore::LowLevel::FontReference,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetSystemFontReference_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetSystemFontReference_Internal", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (familyName, styleName, fontRef))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TextCore+LowLevel+FontEngine")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::LowLevel::FontEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
