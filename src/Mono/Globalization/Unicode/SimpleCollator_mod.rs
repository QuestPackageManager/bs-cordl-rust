#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator")]
#[repr(C)]
#[derive(Debug)]
pub struct SimpleCollator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub textInfo: quest_hook::libil2cpp::Gc<crate::System::Globalization::TextInfo>,
    pub cjkIndexer: quest_hook::libil2cpp::Gc<
        crate::Mono::Globalization::Unicode::CodePointIndexer,
    >,
    pub contractions: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
        >,
    >,
    pub level2Maps: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Level2Map>,
        >,
    >,
    pub unsafeFlags: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub cjkCatTable: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub cjkLv1Table: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub cjkLv2Table: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub cjkLv2Indexer: quest_hook::libil2cpp::Gc<
        crate::Mono::Globalization::Unicode::CodePointIndexer,
    >,
    pub lcid: i32,
    pub frenchSort: bool,
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Globalization::Unicode::SimpleCollator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Globalization.Unicode";
    const CLASS_NAME: &'static str = "SimpleCollator";
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
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::SimpleCollator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator")]
impl std::ops::DerefMut for crate::Mono::Globalization::Unicode::SimpleCollator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator")]
impl crate::Mono::Globalization::Unicode::SimpleCollator {
    #[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Context")]
    pub type Context = crate::Mono::Globalization::Unicode::SimpleCollator_Context;
    #[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Escape")]
    pub type Escape = crate::Mono::Globalization::Unicode::SimpleCollator_Escape;
    #[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+ExtenderType")]
    pub type ExtenderType = crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType;
    #[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+PreviousInfo")]
    pub type PreviousInfo = crate::Mono::Globalization::Unicode::SimpleCollator_PreviousInfo;
    pub fn Category(&mut self, cp: i32) -> quest_hook::libil2cpp::Result<u8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(i32), u8, 1usize>("Category")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Category", 1usize
                        )
                    })
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked(self, (cp))? };
        Ok(__cordl_ret.into())
    }
    pub fn ClearBuffer(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ClearBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ClearBuffer", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (buffer, _cordl_size))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Compare(
        &mut self,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx1: i32,
        len1: i32,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx2: i32,
        len2: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            crate::System::Globalization::CompareOptions,
                        ),
                        i32,
                        7usize,
                    >("Compare")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Compare", 7usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (s1, idx1, len1, s2, idx2, len2, options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CompareFlagPair(
        &mut self,
        b1: bool,
        b2: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(bool, bool), i32, 2usize>("CompareFlagPair")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CompareFlagPair", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (b1, b2))? };
        Ok(__cordl_ret.into())
    }
    pub fn CompareInternal(
        &mut self,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx1: i32,
        len1: i32,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx2: i32,
        len2: i32,
        targetConsumed: quest_hook::libil2cpp::ByRefMut<bool>,
        sourceConsumed: quest_hook::libil2cpp::ByRefMut<bool>,
        skipHeadingExtenders: bool,
        immediateBreakup: bool,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            bool,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Mono::Globalization::Unicode::SimpleCollator_Context,
                            >,
                        ),
                        i32,
                        11usize,
                    >("CompareInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CompareInternal", 11usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        s1,
                        idx1,
                        len1,
                        s2,
                        idx2,
                        len2,
                        targetConsumed,
                        sourceConsumed,
                        skipHeadingExtenders,
                        immediateBreakup,
                        ctx,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillSortKeyRaw(
        &mut self,
        i: i32,
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
        buf: quest_hook::libil2cpp::Gc<
            crate::Mono::Globalization::Unicode::SortKeyBuffer,
        >,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Globalization::Unicode::SortKeyBuffer,
                            >,
                            crate::System::Globalization::CompareOptions,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("FillSortKeyRaw")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FillSortKeyRaw", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (i, ext, buf, opt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FillSurrogateSortKeyRaw(
        &mut self,
        i: i32,
        buf: quest_hook::libil2cpp::Gc<
            crate::Mono::Globalization::Unicode::SortKeyBuffer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Globalization::Unicode::SortKeyBuffer,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("FillSurrogateSortKeyRaw")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FillSurrogateSortKeyRaw", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (i, buf))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FilterExtender(
        &mut self,
        i: i32,
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
                            crate::System::Globalization::CompareOptions,
                        ),
                        i32,
                        3usize,
                    >("FilterExtender")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FilterExtender", 3usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (i, ext, opt))? };
        Ok(__cordl_ret.into())
    }
    pub fn FilterOptions(
        &mut self,
        i: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32, crate::System::Globalization::CompareOptions),
                        i32,
                        2usize,
                    >("FilterOptions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "FilterOptions", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (i, opt))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetContraction_Il2CppArray1(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        end: i32,
        clist: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Mono::Globalization::Unicode::Contraction,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::Mono::Globalization::Unicode::Contraction,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Globalization::Unicode::Contraction,
                        >,
                        4usize,
                    >("GetContraction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetContraction", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Globalization::Unicode::Contraction,
        > = unsafe { method.invoke_unchecked(self, (s, start, end, clist))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetContraction_Il2CppString_i32_i32_0(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Globalization::Unicode::Contraction,
                        >,
                        3usize,
                    >("GetContraction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetContraction", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Globalization::Unicode::Contraction,
        > = unsafe { method.invoke_unchecked(self, (s, start, end))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetExtenderType(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (i32),
                        crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
                        1usize,
                    >("GetExtenderType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetExtenderType", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType = unsafe {
            method.invoke_unchecked(self, (i))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetNeutralCulture(
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Globalization::CultureInfo,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Globalization::CultureInfo,
                        >,
                        1usize,
                    >("GetNeutralCulture")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetNeutralCulture", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::CultureInfo,
        > = unsafe { method.invoke_unchecked((), (info))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSortKey_CompareOptions0(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::SortKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Globalization::CompareOptions,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::SortKey>,
                        2usize,
                    >("GetSortKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSortKey", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::SortKey,
        > = unsafe { method.invoke_unchecked(self, (s, options))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSortKey_i32_i32_CompareOptions1(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Globalization::SortKey>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            crate::System::Globalization::CompareOptions,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Globalization::SortKey>,
                        4usize,
                    >("GetSortKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSortKey", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Globalization::SortKey,
        > = unsafe { method.invoke_unchecked(self, (s, start, length, options))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSortKey_i32_i32_SortKeyBuffer_CompareOptions2(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        end: i32,
        buf: quest_hook::libil2cpp::Gc<
            crate::Mono::Globalization::Unicode::SortKeyBuffer,
        >,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Globalization::Unicode::SortKeyBuffer,
                            >,
                            crate::System::Globalization::CompareOptions,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("GetSortKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetSortKey", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (s, start, end, buf, opt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetTailContraction_Il2CppArray1(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        end: i32,
        clist: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::Mono::Globalization::Unicode::Contraction,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::Mono::Globalization::Unicode::Contraction,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Globalization::Unicode::Contraction,
                        >,
                        4usize,
                    >("GetTailContraction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetTailContraction", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Globalization::Unicode::Contraction,
        > = unsafe { method.invoke_unchecked(self, (s, start, end, clist))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTailContraction_Il2CppString_i32_i32_0(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Globalization::Unicode::Contraction,
                        >,
                        3usize,
                    >("GetTailContraction")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetTailContraction", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Globalization::Unicode::Contraction,
        > = unsafe { method.invoke_unchecked(self, (s, start, end))? };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfOrdinal_Il2CppString0(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("IndexOfOrdinal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IndexOfOrdinal", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (s, target, start, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfOrdinal__cordl_char1(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: char,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            char,
                            i32,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("IndexOfOrdinal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IndexOfOrdinal", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (s, target, start, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOfSortKey(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
        sortkey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        target: char,
        ti: i32,
        noLv4: bool,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            char,
                            i32,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Mono::Globalization::Unicode::SimpleCollator_Context,
                            >,
                        ),
                        i32,
                        8usize,
                    >("IndexOfSortKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IndexOfSortKey", 8usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (s, start, length, sortkey, target, ti, noLv4, ctx),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_CompareOptions0(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            crate::System::Globalization::CompareOptions,
                        ),
                        i32,
                        5usize,
                    >("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IndexOf", 5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (s, target, start, length, opt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_Il2CppObject_ByRefMut1(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
        targetSortKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Mono::Globalization::Unicode::SimpleCollator_Context,
                            >,
                        ),
                        i32,
                        6usize,
                    >("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IndexOf", 6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(self, (s, target, start, length, targetSortKey, ctx))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsHalfKana(
        cp: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::System::Globalization::CompareOptions),
                        bool,
                        2usize,
                    >("IsHalfKana")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsHalfKana", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (cp, opt))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsIgnorable(
        i: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::System::Globalization::CompareOptions),
                        bool,
                        2usize,
                    >("IsIgnorable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsIgnorable", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (i, opt))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsPrefix_CompareOptions0(
        &mut self,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Globalization::CompareOptions,
                        ),
                        bool,
                        3usize,
                    >("IsPrefix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsPrefix", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (src, target, opt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsPrefix_i32_i32_CompareOptions1(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            crate::System::Globalization::CompareOptions,
                        ),
                        bool,
                        5usize,
                    >("IsPrefix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsPrefix", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (s, target, start, length, opt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsPrefix_i32_i32__cordl_bool_ByRefMut2(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
        skipHeadingExtenders: bool,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Mono::Globalization::Unicode::SimpleCollator_Context,
                            >,
                        ),
                        bool,
                        6usize,
                    >("IsPrefix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsPrefix", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (s, target, start, length, skipHeadingExtenders, ctx),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSafe(&mut self, i: i32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(i32), bool, 1usize>("IsSafe")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsSafe", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (i))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsSuffix_CompareOptions0(
        &mut self,
        src: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Globalization::CompareOptions,
                        ),
                        bool,
                        3usize,
                    >("IsSuffix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsSuffix", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (src, target, opt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSuffix_i32_i32_CompareOptions1(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            crate::System::Globalization::CompareOptions,
                        ),
                        bool,
                        5usize,
                    >("IsSuffix")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "IsSuffix", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (s, target, start, length, opt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOfOrdinal(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                        ),
                        i32,
                        4usize,
                    >("LastIndexOfOrdinal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LastIndexOfOrdinal", 4usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (s, target, start, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOfSortKey(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        orgStart: i32,
        length: i32,
        sortkey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ti: i32,
        noLv4: bool,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Mono::Globalization::Unicode::SimpleCollator_Context,
                            >,
                        ),
                        i32,
                        8usize,
                    >("LastIndexOfSortKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LastIndexOfSortKey", 8usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (s, start, orgStart, length, sortkey, ti, noLv4, ctx),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_CompareOptions0(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            crate::System::Globalization::CompareOptions,
                        ),
                        i32,
                        5usize,
                    >("LastIndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LastIndexOf", 5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (s, target, start, length, opt))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LastIndexOf_Il2CppObject_ByRefMut1(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
        targetSortKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Mono::Globalization::Unicode::SimpleCollator_Context,
                            >,
                        ),
                        i32,
                        6usize,
                    >("LastIndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LastIndexOf", 6usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method
                .invoke_unchecked(self, (s, target, start, length, targetSortKey, ctx))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Level1(&mut self, cp: i32) -> quest_hook::libil2cpp::Result<u8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(i32), u8, 1usize>("Level1")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Level1", 1usize
                        )
                    })
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked(self, (cp))? };
        Ok(__cordl_ret.into())
    }
    pub fn Level2(
        &mut self,
        cp: i32,
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            i32,
                            crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
                        ),
                        u8,
                        2usize,
                    >("Level2")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Level2", 2usize
                        )
                    })
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked(self, (cp, ext))? };
        Ok(__cordl_ret.into())
    }
    pub fn MatchesBackward(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx: quest_hook::libil2cpp::ByRefMut<i32>,
        end: i32,
        orgStart: i32,
        ti: i32,
        sortkey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        noLv4: bool,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Mono::Globalization::Unicode::SimpleCollator_Context,
                            >,
                        ),
                        bool,
                        8usize,
                    >("MatchesBackward")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MatchesBackward", 8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (s, idx, end, orgStart, ti, sortkey, noLv4, ctx),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchesBackwardCore(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx: quest_hook::libil2cpp::ByRefMut<i32>,
        end: i32,
        orgStart: i32,
        ti: i32,
        sortkey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        noLv4: bool,
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
        ct: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
        >,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            bool,
                            crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Mono::Globalization::Unicode::Contraction,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Mono::Globalization::Unicode::SimpleCollator_Context,
                            >,
                        ),
                        bool,
                        10usize,
                    >("MatchesBackwardCore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MatchesBackwardCore", 10usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (s, idx, end, orgStart, ti, sortkey, noLv4, ext, ct, ctx),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchesForward(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx: quest_hook::libil2cpp::ByRefMut<i32>,
        end: i32,
        ti: i32,
        sortkey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        noLv4: bool,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Mono::Globalization::Unicode::SimpleCollator_Context,
                            >,
                        ),
                        bool,
                        7usize,
                    >("MatchesForward")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MatchesForward", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (s, idx, end, ti, sortkey, noLv4, ctx))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchesForwardCore(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx: quest_hook::libil2cpp::ByRefMut<i32>,
        end: i32,
        ti: i32,
        sortkey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        noLv4: bool,
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
        ct: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Mono::Globalization::Unicode::Contraction>,
        >,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            bool,
                            crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Mono::Globalization::Unicode::Contraction,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::Mono::Globalization::Unicode::SimpleCollator_Context,
                            >,
                        ),
                        bool,
                        9usize,
                    >("MatchesForwardCore")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MatchesForwardCore", 9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method
                .invoke_unchecked(self, (s, idx, end, ti, sortkey, noLv4, ext, ct, ctx))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MatchesPrimitive(
        &mut self,
        opt: crate::System::Globalization::CompareOptions,
        source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        si: i32,
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ti: i32,
        noLv4: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::System::Globalization::CompareOptions,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            i32,
                            bool,
                        ),
                        bool,
                        7usize,
                    >("MatchesPrimitive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MatchesPrimitive", 7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (opt, source, si, ext, target, ti, noLv4))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (culture))?;
        Ok(__cordl_object.into())
    }
    pub fn QuickIndexOf(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        length: i32,
        testWasUnable: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                        ),
                        i32,
                        5usize,
                    >("QuickIndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "QuickIndexOf", 5usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (s, target, start, length, testWasUnable))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetCJKTable(
        &mut self,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
        cjkIndexer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::Mono::Globalization::Unicode::CodePointIndexer,
            >,
        >,
        catTable: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        lv1Table: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
        lv2Indexer: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::Mono::Globalization::Unicode::CodePointIndexer,
            >,
        >,
        lv2Table: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Globalization::CultureInfo,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Mono::Globalization::Unicode::CodePointIndexer,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Mono::Globalization::Unicode::CodePointIndexer,
                                >,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    quest_hook::libil2cpp::Il2CppObject,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("SetCJKTable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "SetCJKTable", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (culture, cjkIndexer, catTable, lv1Table, lv2Indexer, lv2Table),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Globalization_ISimpleCollator_Compare(
        &mut self,
        s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx1: i32,
        len1: i32,
        s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        idx2: i32,
        len2: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i32,
                            i32,
                            crate::System::Globalization::CompareOptions,
                        ),
                        i32,
                        7usize,
                    >("System.Globalization.ISimpleCollator.Compare")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(),
                            "System.Globalization.ISimpleCollator.Compare", 7usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (s1, idx1, len1, s2, idx2, len2, options))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToDashTypeValue(
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
                            crate::System::Globalization::CompareOptions,
                        ),
                        u8,
                        2usize,
                    >("ToDashTypeValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ToDashTypeValue", 2usize
                        )
                    })
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (ext, opt))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        culture: quest_hook::libil2cpp::Gc<crate::System::Globalization::CultureInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Globalization::CultureInfo,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (culture))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Globalization::Unicode::SimpleCollator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator")]
impl AsRef<crate::System::Globalization::ISimpleCollator>
for crate::Mono::Globalization::Unicode::SimpleCollator {
    fn as_ref(&self) -> &crate::System::Globalization::ISimpleCollator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator")]
impl AsMut<crate::System::Globalization::ISimpleCollator>
for crate::Mono::Globalization::Unicode::SimpleCollator {
    fn as_mut(&mut self) -> &mut crate::System::Globalization::ISimpleCollator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Context")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCollator_Context {
    pub Option: crate::System::Globalization::CompareOptions,
    pub NeverMatchFlags: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub AlwaysMatchFlags: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub Buffer1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub Buffer2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub PrevCode: i32,
    pub PrevSortKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Context")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Globalization::Unicode::SimpleCollator_Context {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Globalization.Unicode";
    const CLASS_NAME: &'static str = "SimpleCollator/Context";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Context")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Globalization::Unicode::SimpleCollator_Context {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Context")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Globalization::Unicode::SimpleCollator_Context {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Context")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Globalization::Unicode::SimpleCollator_Context {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Context")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Globalization::Unicode::SimpleCollator_Context {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Context")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Globalization::Unicode::SimpleCollator_Context {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Context")]
impl crate::Mono::Globalization::Unicode::SimpleCollator_Context {
    pub fn _ctor(
        &mut self,
        opt: crate::System::Globalization::CompareOptions,
        alwaysMatchFlags: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        neverMatchFlags: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        buffer1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        buffer2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        prev1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::System::Globalization::CompareOptions,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (opt, alwaysMatchFlags, neverMatchFlags, buffer1, buffer2, prev1),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Escape")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCollator_Escape {
    pub Source: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Index: i32,
    pub Start: i32,
    pub End: i32,
    pub Optional: i32,
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Escape")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Globalization::Unicode::SimpleCollator_Escape {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Globalization.Unicode";
    const CLASS_NAME: &'static str = "SimpleCollator/Escape";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Escape")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Globalization::Unicode::SimpleCollator_Escape {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Escape")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Globalization::Unicode::SimpleCollator_Escape {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Escape")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Globalization::Unicode::SimpleCollator_Escape {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Escape")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Globalization::Unicode::SimpleCollator_Escape {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Escape")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Globalization::Unicode::SimpleCollator_Escape {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Escape")]
impl crate::Mono::Globalization::Unicode::SimpleCollator_Escape {}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+ExtenderType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SimpleCollator_ExtenderType {
    #[default]
    Buggy = 4i32,
    Conditional = 3i32,
    None = 0i32,
    Simple = 1i32,
    Voiced = 2i32,
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+ExtenderType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Globalization.Unicode";
    const CLASS_NAME: &'static str = "SimpleCollator/ExtenderType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+ExtenderType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+ExtenderType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+ExtenderType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+ExtenderType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+PreviousInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimpleCollator_PreviousInfo {
    pub Code: i32,
    pub SortKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+PreviousInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Globalization::Unicode::SimpleCollator_PreviousInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Globalization.Unicode";
    const CLASS_NAME: &'static str = "SimpleCollator/PreviousInfo";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+PreviousInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Globalization::Unicode::SimpleCollator_PreviousInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+PreviousInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Globalization::Unicode::SimpleCollator_PreviousInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+PreviousInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Globalization::Unicode::SimpleCollator_PreviousInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+PreviousInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Globalization::Unicode::SimpleCollator_PreviousInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+PreviousInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Globalization::Unicode::SimpleCollator_PreviousInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+PreviousInfo")]
impl crate::Mono::Globalization::Unicode::SimpleCollator_PreviousInfo {
    pub fn _ctor(
        &mut self,
        dummy: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (dummy))?
        };
        Ok(__cordl_ret.into())
    }
}
