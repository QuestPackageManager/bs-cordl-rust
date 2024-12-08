#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Context")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SimpleCollator_Context {
    pub Option: crate::System::Globalization::CompareOptions,
    pub NeverMatchFlags: *mut quest_hook::libil2cpp::Il2CppObject,
    pub AlwaysMatchFlags: *mut quest_hook::libil2cpp::Il2CppObject,
    pub Buffer1: *mut quest_hook::libil2cpp::Il2CppObject,
    pub Buffer2: *mut quest_hook::libil2cpp::Il2CppObject,
    pub PrevCode: i32,
    pub PrevSortKey: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Context")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Globalization::Unicode::SimpleCollator_Context =>
    "Mono.Globalization.Unicode"."SimpleCollator/Context"
);
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
        alwaysMatchFlags: *mut quest_hook::libil2cpp::Il2CppObject,
        neverMatchFlags: *mut quest_hook::libil2cpp::Il2CppObject,
        buffer1: *mut quest_hook::libil2cpp::Il2CppObject,
        buffer2: *mut quest_hook::libil2cpp::Il2CppObject,
        prev1: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (opt, alwaysMatchFlags, neverMatchFlags, buffer1, buffer2, prev1),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Escape")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SimpleCollator_Escape {
    pub Source: *mut crate::System::String,
    pub Index: i32,
    pub Start: i32,
    pub End: i32,
    pub Optional: i32,
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+Escape")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Globalization::Unicode::SimpleCollator_Escape => "Mono.Globalization.Unicode"
    ."SimpleCollator/Escape"
);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleCollator_ExtenderType {
    Buggy = 4i32,
    Conditional = 3i32,
    None = 0i32,
    Simple = 1i32,
    Voiced = 2i32,
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+ExtenderType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Globalization::Unicode::SimpleCollator_ExtenderType =>
    "Mono.Globalization.Unicode"."SimpleCollator/ExtenderType"
);
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+PreviousInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SimpleCollator_PreviousInfo {
    pub Code: i32,
    pub SortKey: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator+PreviousInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Globalization::Unicode::SimpleCollator_PreviousInfo =>
    "Mono.Globalization.Unicode"."SimpleCollator/PreviousInfo"
);
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
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (dummy),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator")]
#[repr(C)]
#[derive(Debug)]
pub struct SimpleCollator {
    __cordl_parent: crate::System::Object,
    pub textInfo: *mut crate::System::Globalization::TextInfo,
    pub cjkIndexer: *mut crate::Mono::Globalization::Unicode::CodePointIndexer,
    pub contractions: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Mono::Globalization::Unicode::Contraction,
    >,
    pub level2Maps: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Mono::Globalization::Unicode::Level2Map,
    >,
    pub unsafeFlags: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub cjkCatTable: *mut quest_hook::libil2cpp::Il2CppObject,
    pub cjkLv1Table: *mut quest_hook::libil2cpp::Il2CppObject,
    pub cjkLv2Table: *mut quest_hook::libil2cpp::Il2CppObject,
    pub cjkLv2Indexer: *mut crate::Mono::Globalization::Unicode::CodePointIndexer,
    pub lcid: i32,
    pub frenchSort: bool,
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Globalization::Unicode::SimpleCollator =>
    "Mono.Globalization.Unicode"."SimpleCollator"
);
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::SimpleCollator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+SimpleCollator")]
impl std::ops::DerefMut for crate::Mono::Globalization::Unicode::SimpleCollator {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("Category", (cp))?;
        Ok(__cordl_ret)
    }
    pub fn ClearBuffer(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearBuffer", (buffer, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn Compare(
        &mut self,
        s1: *mut crate::System::String,
        idx1: i32,
        len1: i32,
        s2: *mut crate::System::String,
        idx2: i32,
        len2: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Compare", (s1, idx1, len1, s2, idx2, len2, options))?;
        Ok(__cordl_ret)
    }
    pub fn CompareFlagPair(
        &mut self,
        b1: bool,
        b2: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareFlagPair", (b1, b2))?;
        Ok(__cordl_ret)
    }
    pub fn CompareInternal(
        &mut self,
        s1: *mut crate::System::String,
        idx1: i32,
        len1: i32,
        s2: *mut crate::System::String,
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "CompareInternal",
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
            )?;
        Ok(__cordl_ret)
    }
    pub fn FillSortKeyRaw(
        &mut self,
        i: i32,
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
        buf: *mut crate::Mono::Globalization::Unicode::SortKeyBuffer,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillSortKeyRaw", (i, ext, buf, opt))?;
        Ok(__cordl_ret)
    }
    pub fn FillSurrogateSortKeyRaw(
        &mut self,
        i: i32,
        buf: *mut crate::Mono::Globalization::Unicode::SortKeyBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillSurrogateSortKeyRaw", (i, buf))?;
        Ok(__cordl_ret)
    }
    pub fn FilterExtender(
        &mut self,
        i: i32,
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FilterExtender", (i, ext, opt))?;
        Ok(__cordl_ret)
    }
    pub fn FilterOptions(
        &mut self,
        i: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FilterOptions", (i, opt))?;
        Ok(__cordl_ret)
    }
    pub fn GetContraction_Il2CppArray1(
        &mut self,
        s: *mut crate::System::String,
        start: i32,
        end: i32,
        clist: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Mono::Globalization::Unicode::Contraction,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Globalization::Unicode::Contraction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Globalization::Unicode::Contraction = __cordl_object
            .invoke("GetContraction", (s, start, end, clist))?;
        Ok(__cordl_ret)
    }
    pub fn GetContraction_String_i32_i32_0(
        &mut self,
        s: *mut crate::System::String,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Globalization::Unicode::Contraction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Globalization::Unicode::Contraction = __cordl_object
            .invoke("GetContraction", (s, start, end))?;
        Ok(__cordl_ret)
    }
    pub fn GetExtenderType(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType = __cordl_object
            .invoke("GetExtenderType", (i))?;
        Ok(__cordl_ret)
    }
    pub fn GetSortKey_CompareOptions0(
        &mut self,
        s: *mut crate::System::String,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::SortKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::SortKey = __cordl_object
            .invoke("GetSortKey", (s, options))?;
        Ok(__cordl_ret)
    }
    pub fn GetSortKey_i32_i32_CompareOptions1(
        &mut self,
        s: *mut crate::System::String,
        start: i32,
        length: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::SortKey> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::SortKey = __cordl_object
            .invoke("GetSortKey", (s, start, length, options))?;
        Ok(__cordl_ret)
    }
    pub fn GetSortKey_i32_i32_SortKeyBuffer_CompareOptions2(
        &mut self,
        s: *mut crate::System::String,
        start: i32,
        end: i32,
        buf: *mut crate::Mono::Globalization::Unicode::SortKeyBuffer,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetSortKey", (s, start, end, buf, opt))?;
        Ok(__cordl_ret)
    }
    pub fn GetTailContraction_Il2CppArray1(
        &mut self,
        s: *mut crate::System::String,
        start: i32,
        end: i32,
        clist: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Mono::Globalization::Unicode::Contraction,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Globalization::Unicode::Contraction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Globalization::Unicode::Contraction = __cordl_object
            .invoke("GetTailContraction", (s, start, end, clist))?;
        Ok(__cordl_ret)
    }
    pub fn GetTailContraction_String_i32_i32_0(
        &mut self,
        s: *mut crate::System::String,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Globalization::Unicode::Contraction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Globalization::Unicode::Contraction = __cordl_object
            .invoke("GetTailContraction", (s, start, end))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOfOrdinal_String0(
        &mut self,
        s: *mut crate::System::String,
        target: *mut crate::System::String,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOfOrdinal", (s, target, start, length))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOfOrdinal__cordl_char1(
        &mut self,
        s: *mut crate::System::String,
        target: char,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOfOrdinal", (s, target, start, length))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOfSortKey(
        &mut self,
        s: *mut crate::System::String,
        start: i32,
        length: i32,
        sortkey: *mut quest_hook::libil2cpp::Il2CppObject,
        target: char,
        ti: i32,
        noLv4: bool,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "IndexOfSortKey",
                (s, start, length, sortkey, target, ti, noLv4, ctx),
            )?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf_CompareOptions0(
        &mut self,
        s: *mut crate::System::String,
        target: *mut crate::System::String,
        start: i32,
        length: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOf", (s, target, start, length, opt))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf_Il2CppObject_ByRefMut1(
        &mut self,
        s: *mut crate::System::String,
        target: *mut crate::System::String,
        start: i32,
        length: i32,
        targetSortKey: *mut quest_hook::libil2cpp::Il2CppObject,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("IndexOf", (s, target, start, length, targetSortKey, ctx))?;
        Ok(__cordl_ret)
    }
    pub fn IsPrefix_CompareOptions0(
        &mut self,
        src: *mut crate::System::String,
        target: *mut crate::System::String,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsPrefix", (src, target, opt))?;
        Ok(__cordl_ret)
    }
    pub fn IsPrefix_i32_i32_CompareOptions1(
        &mut self,
        s: *mut crate::System::String,
        target: *mut crate::System::String,
        start: i32,
        length: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsPrefix", (s, target, start, length, opt))?;
        Ok(__cordl_ret)
    }
    pub fn IsPrefix_i32_i32__cordl_bool_ByRefMut2(
        &mut self,
        s: *mut crate::System::String,
        target: *mut crate::System::String,
        start: i32,
        length: i32,
        skipHeadingExtenders: bool,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsPrefix", (s, target, start, length, skipHeadingExtenders, ctx))?;
        Ok(__cordl_ret)
    }
    pub fn IsSafe(&mut self, i: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSafe", (i))?;
        Ok(__cordl_ret)
    }
    pub fn IsSuffix_CompareOptions0(
        &mut self,
        src: *mut crate::System::String,
        target: *mut crate::System::String,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSuffix", (src, target, opt))?;
        Ok(__cordl_ret)
    }
    pub fn IsSuffix_i32_i32_CompareOptions1(
        &mut self,
        s: *mut crate::System::String,
        target: *mut crate::System::String,
        start: i32,
        length: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSuffix", (s, target, start, length, opt))?;
        Ok(__cordl_ret)
    }
    pub fn LastIndexOfOrdinal(
        &mut self,
        s: *mut crate::System::String,
        target: *mut crate::System::String,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LastIndexOfOrdinal", (s, target, start, length))?;
        Ok(__cordl_ret)
    }
    pub fn LastIndexOfSortKey(
        &mut self,
        s: *mut crate::System::String,
        start: i32,
        orgStart: i32,
        length: i32,
        sortkey: *mut quest_hook::libil2cpp::Il2CppObject,
        ti: i32,
        noLv4: bool,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "LastIndexOfSortKey",
                (s, start, orgStart, length, sortkey, ti, noLv4, ctx),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LastIndexOf_CompareOptions0(
        &mut self,
        s: *mut crate::System::String,
        target: *mut crate::System::String,
        start: i32,
        length: i32,
        opt: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LastIndexOf", (s, target, start, length, opt))?;
        Ok(__cordl_ret)
    }
    pub fn LastIndexOf_Il2CppObject_ByRefMut1(
        &mut self,
        s: *mut crate::System::String,
        target: *mut crate::System::String,
        start: i32,
        length: i32,
        targetSortKey: *mut quest_hook::libil2cpp::Il2CppObject,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LastIndexOf", (s, target, start, length, targetSortKey, ctx))?;
        Ok(__cordl_ret)
    }
    pub fn Level1(&mut self, cp: i32) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("Level1", (cp))?;
        Ok(__cordl_ret)
    }
    pub fn Level2(
        &mut self,
        cp: i32,
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("Level2", (cp, ext))?;
        Ok(__cordl_ret)
    }
    pub fn MatchesBackward(
        &mut self,
        s: *mut crate::System::String,
        idx: quest_hook::libil2cpp::ByRefMut<i32>,
        end: i32,
        orgStart: i32,
        ti: i32,
        sortkey: *mut quest_hook::libil2cpp::Il2CppObject,
        noLv4: bool,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "MatchesBackward",
                (s, idx, end, orgStart, ti, sortkey, noLv4, ctx),
            )?;
        Ok(__cordl_ret)
    }
    pub fn MatchesBackwardCore(
        &mut self,
        s: *mut crate::System::String,
        idx: quest_hook::libil2cpp::ByRefMut<i32>,
        end: i32,
        orgStart: i32,
        ti: i32,
        sortkey: *mut quest_hook::libil2cpp::Il2CppObject,
        noLv4: bool,
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
        ct: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Mono::Globalization::Unicode::Contraction,
        >,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "MatchesBackwardCore",
                (s, idx, end, orgStart, ti, sortkey, noLv4, ext, ct, ctx),
            )?;
        Ok(__cordl_ret)
    }
    pub fn MatchesForward(
        &mut self,
        s: *mut crate::System::String,
        idx: quest_hook::libil2cpp::ByRefMut<i32>,
        end: i32,
        ti: i32,
        sortkey: *mut quest_hook::libil2cpp::Il2CppObject,
        noLv4: bool,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchesForward", (s, idx, end, ti, sortkey, noLv4, ctx))?;
        Ok(__cordl_ret)
    }
    pub fn MatchesForwardCore(
        &mut self,
        s: *mut crate::System::String,
        idx: quest_hook::libil2cpp::ByRefMut<i32>,
        end: i32,
        ti: i32,
        sortkey: *mut quest_hook::libil2cpp::Il2CppObject,
        noLv4: bool,
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
        ct: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Mono::Globalization::Unicode::Contraction,
        >,
        ctx: quest_hook::libil2cpp::ByRefMut<
            crate::Mono::Globalization::Unicode::SimpleCollator_Context,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "MatchesForwardCore",
                (s, idx, end, ti, sortkey, noLv4, ext, ct, ctx),
            )?;
        Ok(__cordl_ret)
    }
    pub fn MatchesPrimitive(
        &mut self,
        opt: crate::System::Globalization::CompareOptions,
        source: *mut quest_hook::libil2cpp::Il2CppObject,
        si: i32,
        ext: crate::Mono::Globalization::Unicode::SimpleCollator_ExtenderType,
        target: *mut quest_hook::libil2cpp::Il2CppObject,
        ti: i32,
        noLv4: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MatchesPrimitive", (opt, source, si, ext, target, ti, noLv4))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (culture))?;
        Ok(__cordl_object)
    }
    pub fn QuickIndexOf(
        &mut self,
        s: *mut crate::System::String,
        target: *mut crate::System::String,
        start: i32,
        length: i32,
        testWasUnable: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("QuickIndexOf", (s, target, start, length, testWasUnable))?;
        Ok(__cordl_ret)
    }
    pub fn SetCJKTable(
        &mut self,
        culture: *mut crate::System::Globalization::CultureInfo,
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetCJKTable",
                (culture, cjkIndexer, catTable, lv1Table, lv2Indexer, lv2Table),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Globalization_ISimpleCollator_Compare(
        &mut self,
        s1: *mut crate::System::String,
        idx1: i32,
        len1: i32,
        s2: *mut crate::System::String,
        idx2: i32,
        len2: i32,
        options: crate::System::Globalization::CompareOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "System.Globalization.ISimpleCollator.Compare",
                (s1, idx1, len1, s2, idx2, len2, options),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        culture: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (culture))?;
        Ok(__cordl_ret)
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
