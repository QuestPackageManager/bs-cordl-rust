#[cfg(feature = "cordl_class_UnityEngine+TextCore+LowLevel+FontEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct FontEngine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+LowLevel+FontEngine")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::TextCore::LowLevel::FontEngine {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.TextCore.LowLevel";
    const CLASS_NAME: &'static str = "FontEngine";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<T>,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "GenericListToMarshallingArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GenericListToMarshallingArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (srcList, dstArray))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllLigatureSubstitutionRecords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
                        >,
                    >, 0usize>("GetAllLigatureSubstitutionRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAllLigatureSubstitutionRecords",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllMarkToBaseAdjustmentRecords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
                        >,
                    >, 0usize>("GetAllMarkToBaseAdjustmentRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAllMarkToBaseAdjustmentRecords",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllMarkToBaseAdjustmentRecords_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::BlittableArrayWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Bindings::BlittableArrayWrapper,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "GetAllMarkToBaseAdjustmentRecords_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAllMarkToBaseAdjustmentRecords_Injected",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllMarkToMarkAdjustmentRecords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
                        >,
                    >, 0usize>("GetAllMarkToMarkAdjustmentRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAllMarkToMarkAdjustmentRecords",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllMarkToMarkAdjustmentRecords_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::BlittableArrayWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Bindings::BlittableArrayWrapper,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "GetAllMarkToMarkAdjustmentRecords_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAllMarkToMarkAdjustmentRecords_Injected",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllPairAdjustmentRecords() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
                        >,
                    >, 0usize>("GetAllPairAdjustmentRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAllPairAdjustmentRecords",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetAllPairAdjustmentRecords_Injected(
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::BlittableArrayWrapper>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Bindings::BlittableArrayWrapper,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "GetAllPairAdjustmentRecords_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetAllPairAdjustmentRecords_Injected",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (ret))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceInfo() -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::FaceInfo> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), crate::UnityEngine::TextCore::FaceInfo, 0usize>(
                        "GetFaceInfo",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFaceInfo",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::FaceInfo =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceInfo_Internal(
        faceInfo: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::TextCore::FaceInfo>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (faceInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFontFaces() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        >,
                    >, 0usize>("GetFontFaces")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFontFaces",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFontFaces_Internal() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        >,
                    >, 0usize>("GetFontFaces_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFontFaces_Internal",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetGlyphIndex(unicode: u32) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32), u32, 1usize>("GetGlyphIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetGlyphIndex",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (unicode))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
        > = unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLigatureSubstitutionRecordsFromMarshallingArray(
        ligatureSubstitutionRecords: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
                            >,
                        >,
                    >), i32, 1usize>(
                        "GetLigatureSubstitutionRecordsFromMarshallingArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetLigatureSubstitutionRecordsFromMarshallingArray",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (ligatureSubstitutionRecords))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLigatureSubstitutionRecords_Il2CppArray2(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u32>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
                            >,
                        >,
                        1usize,
                    >("GetLigatureSubstitutionRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLigatureSubstitutionRecords", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLigatureSubstitutionRecords_List_1_1(
        glyphIndexes: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<u32>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
                            >,
                        >,
                        1usize,
                    >("GetLigatureSubstitutionRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetLigatureSubstitutionRecords", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLigatureSubstitutionRecords_u32_0(
        glyphIndex: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
                        >,
                    >, 1usize>("GetLigatureSubstitutionRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetLigatureSubstitutionRecords",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::LigatureSubstitutionRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (glyphIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkToBaseAdjustmentRecordsFromMarshallingArray(
        adjustmentRecords: crate::System::Span_1<
            crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::System::Span_1<
                        crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
                    >), i32, 1usize>(
                        "GetMarkToBaseAdjustmentRecordsFromMarshallingArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMarkToBaseAdjustmentRecordsFromMarshallingArray",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (adjustmentRecords))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkToBaseAdjustmentRecordsFromMarshallingArray_Injected(
        adjustmentRecords: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Bindings::ManagedSpanWrapper,
                    >), i32, 1usize>(
                        "GetMarkToBaseAdjustmentRecordsFromMarshallingArray_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMarkToBaseAdjustmentRecordsFromMarshallingArray_Injected",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (adjustmentRecords))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkToBaseAdjustmentRecords_Il2CppArray1(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u32>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
                            >,
                        >,
                        1usize,
                    >("GetMarkToBaseAdjustmentRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMarkToBaseAdjustmentRecords", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkToBaseAdjustmentRecords_List_1_0(
        glyphIndexes: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<u32>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
                            >,
                        >,
                        1usize,
                    >("GetMarkToBaseAdjustmentRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMarkToBaseAdjustmentRecords", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::MarkToBaseAdjustmentRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkToMarkAdjustmentRecordsFromMarshallingArray(
        adjustmentRecords: crate::System::Span_1<
            crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::System::Span_1<
                        crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
                    >), i32, 1usize>(
                        "GetMarkToMarkAdjustmentRecordsFromMarshallingArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMarkToMarkAdjustmentRecordsFromMarshallingArray",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (adjustmentRecords))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkToMarkAdjustmentRecordsFromMarshallingArray_Injected(
        adjustmentRecords: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Bindings::ManagedSpanWrapper,
                    >), i32, 1usize>(
                        "GetMarkToMarkAdjustmentRecordsFromMarshallingArray_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetMarkToMarkAdjustmentRecordsFromMarshallingArray_Injected",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (adjustmentRecords))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkToMarkAdjustmentRecords_Il2CppArray1(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u32>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
                            >,
                        >,
                        1usize,
                    >("GetMarkToMarkAdjustmentRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMarkToMarkAdjustmentRecords", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetMarkToMarkAdjustmentRecords_List_1_0(
        glyphIndexes: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<u32>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
                            >,
                        >,
                        1usize,
                    >("GetMarkToMarkAdjustmentRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMarkToMarkAdjustmentRecords", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::MarkToMarkAdjustmentRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPairAdjustmentRecordsFromMarshallingArray(
        glyphPairAdjustmentRecords: crate::System::Span_1<
            crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::System::Span_1<
                        crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
                    >), i32, 1usize>(
                        "GetPairAdjustmentRecordsFromMarshallingArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPairAdjustmentRecordsFromMarshallingArray",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphPairAdjustmentRecords))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPairAdjustmentRecordsFromMarshallingArray_Injected(
        glyphPairAdjustmentRecords: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Bindings::ManagedSpanWrapper,
                    >), i32, 1usize>(
                        "GetPairAdjustmentRecordsFromMarshallingArray_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPairAdjustmentRecordsFromMarshallingArray_Injected",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphPairAdjustmentRecords))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPairAdjustmentRecords_Il2CppArray1(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
                    >("GetPairAdjustmentRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPairAdjustmentRecords", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetPairAdjustmentRecords_List_1_0(
        glyphIndexes: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<u32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<u32>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
                            >,
                        >,
                        1usize,
                    >("GetPairAdjustmentRecords")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPairAdjustmentRecords", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::TextCore::LowLevel::GlyphPairAdjustmentRecord,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetVariantGlyphIndex(
        unicode: u32,
        variantSelectorUnicode: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32, u32), u32, 2usize>("GetVariantGlyphIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetVariantGlyphIndex",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u32 =
            unsafe { cordl_method_info.invoke_unchecked((), (unicode, variantSelectorUnicode))? };
        Ok(__cordl_ret.into())
    }
    pub fn GlyphIndexToMarshallingArray(
        glyphIndex: u32,
        dstArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "GlyphIndexToMarshallingArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GlyphIndexToMarshallingArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphIndex, dstArray))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Font_f32_i32_2(
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        pointSize: f32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::LowLevel::FontEngineError>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
                        f32,
                        i32,
                    ), crate::UnityEngine::TextCore::LowLevel::FontEngineError, 3usize>(
                        "LoadFontFace",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadFontFace",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError =
            unsafe { cordl_method_info.invoke_unchecked((), (font, pointSize, faceIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Il2CppString0(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::LowLevel::FontEngineError>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
                        1usize,
                    >("LoadFontFace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadFontFace", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError =
            unsafe { cordl_method_info.invoke_unchecked((), (filePath))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Il2CppString_Il2CppString_f32_3(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pointSize: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::LowLevel::FontEngineError>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                    ), crate::UnityEngine::TextCore::LowLevel::FontEngineError, 3usize>(
                        "LoadFontFace",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadFontFace",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError =
            unsafe { cordl_method_info.invoke_unchecked((), (familyName, styleName, pointSize))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Il2CppString_f32_i32_1(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pointSize: f32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::LowLevel::FontEngineError>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        f32,
                        i32,
                    ), crate::UnityEngine::TextCore::LowLevel::FontEngineError, 3usize>(
                        "LoadFontFace",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadFontFace",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError =
            unsafe { cordl_method_info.invoke_unchecked((), (filePath, pointSize, faceIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Internal(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        i32,
                        1usize,
                    >("LoadFontFace_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "LoadFontFace_Internal", 1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (filePath))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_Internal_Injected(
        filePath: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Bindings::ManagedSpanWrapper,
                    >), i32, 1usize>("LoadFontFace_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadFontFace_Internal_Injected",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (filePath))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_And_FaceIndex_Internal(
        filePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pointSize: i32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                        i32,
                    ), i32, 3usize>(
                        "LoadFontFace_With_Size_And_FaceIndex_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadFontFace_With_Size_And_FaceIndex_Internal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (filePath, pointSize, faceIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_And_FaceIndex_Internal_Injected(
        filePath: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bindings::ManagedSpanWrapper>,
        pointSize: i32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        i32,
                        i32,
                    ), i32, 3usize>(
                        "LoadFontFace_With_Size_And_FaceIndex_Internal_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadFontFace_With_Size_And_FaceIndex_Internal_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (filePath, pointSize, faceIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_and_FaceIndex_FromFont_Internal(
        font: quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
        pointSize: i32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Font>,
                        i32,
                        i32,
                    ), i32, 3usize>(
                        "LoadFontFace_With_Size_and_FaceIndex_FromFont_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadFontFace_With_Size_and_FaceIndex_FromFont_Internal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (font, pointSize, faceIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_and_FaceIndex_FromFont_Internal_Injected(
        font: crate::System::IntPtr,
        pointSize: i32,
        faceIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::System::IntPtr, i32, i32), i32, 3usize>(
                        "LoadFontFace_With_Size_and_FaceIndex_FromFont_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadFontFace_With_Size_and_FaceIndex_FromFont_Internal_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (font, pointSize, faceIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_by_FamilyName_and_StyleName_Internal(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pointSize: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), i32, 3usize>(
                        "LoadFontFace_With_Size_by_FamilyName_and_StyleName_Internal"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadFontFace_With_Size_by_FamilyName_and_StyleName_Internal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (familyName, styleName, pointSize))? };
        Ok(__cordl_ret.into())
    }
    pub fn LoadFontFace_With_Size_by_FamilyName_and_StyleName_Internal_Injected(
        familyName: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        styleName: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        pointSize: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        i32,
                    ), i32, 3usize>(
                        "LoadFontFace_With_Size_by_FamilyName_and_StyleName_Internal_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadFontFace_With_Size_by_FamilyName_and_StyleName_Internal_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (familyName, styleName, pointSize))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateLigatureSubstitutionRecordMarshallingArray(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), i32, 2usize>(
                        "PopulateLigatureSubstitutionRecordMarshallingArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PopulateLigatureSubstitutionRecordMarshallingArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes, recordCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateLigatureSubstitutionRecordMarshallingArray_Injected(
        glyphIndexes: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), i32, 2usize>(
                        "PopulateLigatureSubstitutionRecordMarshallingArray_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PopulateLigatureSubstitutionRecordMarshallingArray_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes, recordCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateMarkToBaseAdjustmentRecordMarshallingArray(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), i32, 2usize>(
                        "PopulateMarkToBaseAdjustmentRecordMarshallingArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PopulateMarkToBaseAdjustmentRecordMarshallingArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes, recordCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateMarkToBaseAdjustmentRecordMarshallingArray_Injected(
        glyphIndexes: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), i32, 2usize>(
                        "PopulateMarkToBaseAdjustmentRecordMarshallingArray_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PopulateMarkToBaseAdjustmentRecordMarshallingArray_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes, recordCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateMarkToMarkAdjustmentRecordMarshallingArray(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), i32, 2usize>(
                        "PopulateMarkToMarkAdjustmentRecordMarshallingArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PopulateMarkToMarkAdjustmentRecordMarshallingArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes, recordCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateMarkToMarkAdjustmentRecordMarshallingArray_Injected(
        glyphIndexes: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), i32, 2usize>(
                        "PopulateMarkToMarkAdjustmentRecordMarshallingArray_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PopulateMarkToMarkAdjustmentRecordMarshallingArray_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes, recordCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopulatePairAdjustmentRecordMarshallingArray(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), i32, 2usize>(
                        "PopulatePairAdjustmentRecordMarshallingArray"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PopulatePairAdjustmentRecordMarshallingArray",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes, recordCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopulatePairAdjustmentRecordMarshallingArray_Injected(
        glyphIndexes: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), i32, 2usize>(
                        "PopulatePairAdjustmentRecordMarshallingArray_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PopulatePairAdjustmentRecordMarshallingArray_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes, recordCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopulatePairAdjustmentRecordMarshallingArray_from_KernTable(
        glyphIndexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), i32, 2usize>(
                        "PopulatePairAdjustmentRecordMarshallingArray_from_KernTable"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PopulatePairAdjustmentRecordMarshallingArray_from_KernTable",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes, recordCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn PopulatePairAdjustmentRecordMarshallingArray_from_KernTable_Injected(
        glyphIndexes: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        recordCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), i32, 2usize>(
                        "PopulatePairAdjustmentRecordMarshallingArray_from_KernTable_Injected",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "PopulatePairAdjustmentRecordMarshallingArray_from_KernTable_Injected",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphIndexes, recordCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetAtlasTexture(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetAtlasTexture_Injected(
        texture: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ResetAtlasTexture_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResetAtlasTexture_Injected", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (texture))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetMarshallingArraySize<T>(
        marshallingArray: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        >,
        recordCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
                        >,
                        i32,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "SetMarshallingArraySize"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetMarshallingArraySize",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (marshallingArray, recordCount))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetTextureUploadMode(
        shouldUploadImmediately: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "SetTextureUploadMode",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetTextureUploadMode",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (shouldUploadImmediately))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryAddGlyphToTexture(
        glyphIndex: u32,
        padding: i32,
        packingMode: crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
        freeGlyphRects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::TextCore::GlyphRect>,
        >,
        usedGlyphRects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::TextCore::GlyphRect>,
        >,
        renderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        glyph: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
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
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
                        >,
                    ), bool, 8usize>("TryAddGlyphToTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryAddGlyphToTexture",
                            8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
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
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::TextCore::GlyphRect>,
            >,
        >,
        freeGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        usedGlyphRects: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::TextCore::GlyphRect>,
            >,
        >,
        usedGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        renderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        glyph: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
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
                    ), bool, 10usize>("TryAddGlyphToTexture_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryAddGlyphToTexture_Internal",
                            10usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
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
    pub fn TryAddGlyphToTexture_Internal_Injected(
        glyphIndex: u32,
        padding: i32,
        packingMode: crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
        freeGlyphRects: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::BlittableArrayWrapper,
        >,
        freeGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        usedGlyphRects: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::BlittableArrayWrapper,
        >,
        usedGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        renderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        texture: crate::System::IntPtr,
        glyph: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u32,
                        i32,
                        crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::BlittableArrayWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::BlittableArrayWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
                        >,
                    ), bool, 10usize>("TryAddGlyphToTexture_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryAddGlyphToTexture_Internal_Injected",
                            10usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
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
        glyphIndexes: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<u32>>,
        padding: i32,
        packingMode: crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
        freeGlyphRects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::TextCore::GlyphRect>,
        >,
        usedGlyphRects: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::TextCore::GlyphRect>,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<u32>>,
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
                                    quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
                                >,
                            >,
                        >,
                    ), bool, 8usize>("TryAddGlyphsToTexture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryAddGlyphsToTexture",
                            8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
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
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::TextCore::GlyphRect>,
            >,
        >,
        freeGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        usedGlyphRects: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::TextCore::GlyphRect>,
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
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
                    ), bool, 11usize>("TryAddGlyphsToTexture_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryAddGlyphsToTexture_Internal",
                            11usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
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
    pub fn TryAddGlyphsToTexture_Internal_Injected(
        glyphIndex: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        padding: i32,
        packingMode: crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
        freeGlyphRects: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::BlittableArrayWrapper,
        >,
        freeGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        usedGlyphRects: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::BlittableArrayWrapper,
        >,
        usedGlyphRectCount: quest_hook::libil2cpp::ByRefMut<i32>,
        renderMode: crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
        texture: crate::System::IntPtr,
        glyphs: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::BlittableArrayWrapper,
        >,
        glyphCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        i32,
                        crate::UnityEngine::TextCore::LowLevel::GlyphPackingMode,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::BlittableArrayWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::BlittableArrayWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                        crate::UnityEngine::TextCore::LowLevel::GlyphRenderMode,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::BlittableArrayWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<i32>,
                    ), bool, 11usize>("TryAddGlyphsToTexture_Internal_Injected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryAddGlyphsToTexture_Internal_Injected",
                            11usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u32,
                        crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
                        >,
                    ), bool, 3usize>("TryGetGlyphWithIndexValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetGlyphWithIndexValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (glyphIndex, flags, glyph))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetGlyphWithIndexValue_Internal(
        glyphIndex: u32,
        loadFlags: crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
        glyphStruct: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u32,
                        crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
                        >,
                    ), bool, 3usize>("TryGetGlyphWithIndexValue_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetGlyphWithIndexValue_Internal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (glyphIndex, loadFlags, glyphStruct))?
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u32,
                        crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::TextCore::Glyph>,
                        >,
                    ), bool, 3usize>("TryGetGlyphWithUnicodeValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetGlyphWithUnicodeValue",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (unicode, flags, glyph))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetGlyphWithUnicodeValue_Internal(
        unicode: u32,
        loadFlags: crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
        glyphStruct: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        u32,
                        crate::UnityEngine::TextCore::LowLevel::GlyphLoadFlags,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::TextCore::LowLevel::GlyphMarshallingStruct,
                        >,
                    ), bool, 3usize>("TryGetGlyphWithUnicodeValue_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetGlyphWithUnicodeValue_Internal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (unicode, loadFlags, glyphStruct))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSystemFontReference(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fontRef: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::FontReference,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::TextCore::LowLevel::FontReference,
                        >,
                    ), bool, 3usize>("TryGetSystemFontReference")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetSystemFontReference",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (familyName, styleName, fontRef))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSystemFontReference_Internal(
        familyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        styleName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fontRef: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::FontReference,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::TextCore::LowLevel::FontReference,
                        >,
                    ), bool, 3usize>("TryGetSystemFontReference_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetSystemFontReference_Internal",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (familyName, styleName, fontRef))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSystemFontReference_Internal_Injected(
        familyName: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        styleName: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Bindings::ManagedSpanWrapper,
        >,
        fontRef: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::TextCore::LowLevel::FontReference,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Bindings::ManagedSpanWrapper,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::TextCore::LowLevel::FontReference,
                        >,
                    ), bool, 3usize>(
                        "TryGetSystemFontReference_Internal_Injected"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetSystemFontReference_Internal_Injected",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (familyName, styleName, fontRef))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnloadFontFace(
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::TextCore::LowLevel::FontEngineError>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::TextCore::LowLevel::FontEngineError,
                        0usize,
                    >("UnloadFontFace")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnloadFontFace", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::TextCore::LowLevel::FontEngineError =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn UnloadFontFace_Internal() -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), i32, 0usize>("UnloadFontFace_Internal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnloadFontFace_Internal",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextCore+LowLevel+FontEngine")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::TextCore::LowLevel::FontEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
