#[cfg(feature = "System+Text+RegularExpressions+Regex")]
#[repr(C)]
#[derive(Debug)]
pub struct Regex {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub internalMatchTimeout: crate::System::TimeSpan,
    pub pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub roptions: crate::System::Text::RegularExpressions::RegexOptions,
    pub factory: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::RegexRunnerFactory,
    >,
    pub caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub capnames: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub capslist: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub capsize: i32,
    pub _runnerref: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::ExclusiveReference,
    >,
    pub _replref: quest_hook::libil2cpp::Gc<
        crate::System::WeakReference_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Text::RegularExpressions::RegexReplacement,
            >,
        >,
    >,
    pub _code: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::RegexCode,
    >,
    pub _refsInitialized: bool,
}
#[cfg(feature = "System+Text+RegularExpressions+Regex")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Text::RegularExpressions::Regex {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Text.RegularExpressions";
    const CLASS_NAME: &'static str = "Regex";
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
#[cfg(feature = "System+Text+RegularExpressions+Regex")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::Regex {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::Regex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex")]
impl crate::System::Text::RegularExpressions::Regex {
    pub const CacheDictionarySwitchLimit: i32 = 10i32;
    pub const DefaultMatchTimeout_ConfigKeyName: &'static str = "REGEX_DEFAULT_MATCH_TIMEOUT";
    pub const MaxOptionShift: i32 = 10i32;
    #[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntry")]
    pub type CachedCodeEntry = crate::System::Text::RegularExpressions::Regex_CachedCodeEntry;
    #[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntryKey")]
    pub type CachedCodeEntryKey = crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey;
    pub fn FillCacheDictionary(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FillCacheDictionary", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedCode(
        &mut self,
        key: crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
        isToAdd: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Regex_CachedCodeEntry,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Regex_CachedCodeEntry,
        > = __cordl_object.invoke("GetCachedCode", (key, isToAdd))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedCodeEntryInternal(
        &mut self,
        key: crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
        isToAdd: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Regex_CachedCodeEntry,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Regex_CachedCodeEntry,
        > = __cordl_object.invoke("GetCachedCodeEntryInternal", (key, isToAdd))?;
        Ok(__cordl_ret.into())
    }
    pub fn GroupNameFromNumber(
        &mut self,
        i: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GroupNameFromNumber", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn GroupNumberFromName(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GroupNumberFromName", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitDefaultMatchTimeout() -> quest_hook::libil2cpp::Result<
        crate::System::TimeSpan,
    > {
        let __cordl_ret: crate::System::TimeSpan = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitDefaultMatchTimeout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeReferences(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeReferences", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMatch_Il2CppString0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMatch", (input, pattern))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMatch_Il2CppString2(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMatch", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMatch_Il2CppString_RegexOptions_TimeSpan1(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        matchTimeout: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMatch", (input, pattern, options, matchTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMatch_i32_3(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startat: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsMatch", (input, startat))?;
        Ok(__cordl_ret.into())
    }
    pub fn LookupCachedAndPromote(
        key: crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Regex_CachedCodeEntry,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Regex_CachedCodeEntry,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LookupCachedAndPromote", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn Match_Il2CppString0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Match>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Match,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Match", (input, pattern))?;
        Ok(__cordl_ret.into())
    }
    pub fn Match_Il2CppString2(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Match>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Match,
        > = __cordl_object.invoke("Match", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn Match_Il2CppString_RegexOptions_TimeSpan1(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        matchTimeout: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Match>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Match,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Match", (input, pattern, options, matchTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn Match_i32_3(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startat: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Match>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Match,
        > = __cordl_object.invoke("Match", (input, startat))?;
        Ok(__cordl_ret.into())
    }
    pub fn Matches(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startat: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::MatchCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::MatchCollection,
        > = __cordl_object.invoke("Matches", (input, startat))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppString0(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pattern))?;
        Ok(__cordl_object.into())
    }
    pub fn New_RegexOptions1(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pattern, options))?;
        Ok(__cordl_object.into())
    }
    pub fn New_RegexOptions_TimeSpan__cordl_bool2(
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        matchTimeout: crate::System::TimeSpan,
        addToCache: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pattern, options, matchTimeout, addToCache))?;
        Ok(__cordl_object.into())
    }
    pub fn Replace_Il2CppString0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        replacement: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Replace", (input, pattern, replacement))?;
        Ok(__cordl_ret.into())
    }
    pub fn Replace_Il2CppString_Il2CppString3(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        replacement: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("Replace", (input, replacement))?;
        Ok(__cordl_ret.into())
    }
    pub fn Replace_Il2CppString_RegexOptions1(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        replacement: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Replace", (input, pattern, replacement, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn Replace_Il2CppString_RegexOptions_TimeSpan2(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        replacement: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        matchTimeout: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Replace", (input, pattern, replacement, options, matchTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn Replace_i32_i32_4(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        replacement: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        count: i32,
        startat: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("Replace", (input, replacement, count, startat))?;
        Ok(__cordl_ret.into())
    }
    pub fn Run(
        &mut self,
        quick: bool,
        prevlen: i32,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        beginning: i32,
        length: i32,
        startat: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Match>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::Match,
        > = __cordl_object
            .invoke("Run", (quick, prevlen, input, beginning, length, startat))?;
        Ok(__cordl_ret.into())
    }
    pub fn Split_Il2CppString0(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = __cordl_object.invoke("Split", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn Split_Il2CppString_i32_i32_1(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        count: i32,
        startat: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = __cordl_object.invoke("Split", (input, count, startat))?;
        Ok(__cordl_ret.into())
    }
    pub fn Split_Regex_Il2CppString_i32_i32_2(
        regex: quest_hook::libil2cpp::Gc<crate::System::Text::RegularExpressions::Regex>,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        count: i32,
        startat: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Split", (regex, input, count, startat))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        si: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (si, context),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetCacheValue(
        key: crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
        entry: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Text::RegularExpressions::Regex_CachedCodeEntry,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetCacheValue", (key, entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetCacheValueSmall(
        key: crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
        entry: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Text::RegularExpressions::Regex_CachedCodeEntry,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetCacheValueSmall", (key, entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn UseOptionInvariant(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UseOptionInvariant", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UseOptionR(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("UseOptionR", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateMatchTimeout(
        matchTimeout: crate::System::TimeSpan,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateMatchTimeout", (matchTimeout))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pattern))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RegexOptions1(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Text::RegularExpressions::RegexOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pattern, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RegexOptions_TimeSpan__cordl_bool2(
        &mut self,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        matchTimeout: crate::System::TimeSpan,
        addToCache: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pattern, options, matchTimeout, addToCache))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Options(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Text::RegularExpressions::RegexOptions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Text::RegularExpressions::RegexOptions = __cordl_object
            .invoke("get_Options", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RightToLeft(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RightToLeft", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::Regex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex")]
impl AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Text::RegularExpressions::Regex {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex")]
impl AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Text::RegularExpressions::Regex {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct Regex_CachedCodeEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Next: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::Regex_CachedCodeEntry,
    >,
    pub Previous: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::Regex_CachedCodeEntry,
    >,
    pub Key: crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
    pub Code: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::RegexCode,
    >,
    pub Caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub Capnames: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
    pub Capslist: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub Capsize: i32,
    pub Runnerref: quest_hook::libil2cpp::Gc<
        crate::System::Text::RegularExpressions::ExclusiveReference,
    >,
    pub ReplRef: quest_hook::libil2cpp::Gc<
        crate::System::WeakReference_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Text::RegularExpressions::RegexReplacement,
            >,
        >,
    >,
}
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Text::RegularExpressions::Regex_CachedCodeEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Text.RegularExpressions";
    const CLASS_NAME: &'static str = "Regex/CachedCodeEntry";
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
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntry")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::Regex_CachedCodeEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntry")]
impl std::ops::DerefMut
for crate::System::Text::RegularExpressions::Regex_CachedCodeEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntry")]
impl crate::System::Text::RegularExpressions::Regex_CachedCodeEntry {
    pub fn New(
        key: crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
        capnames: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capslist: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        code: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCode,
        >,
        caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capsize: i32,
        runner: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::ExclusiveReference,
        >,
        replref: quest_hook::libil2cpp::Gc<
            crate::System::WeakReference_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Text::RegularExpressions::RegexReplacement,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (key, capnames, capslist, code, caps, capsize, runner, replref),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        key: crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
        capnames: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capslist: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        code: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::RegexCode,
        >,
        caps: quest_hook::libil2cpp::Gc<crate::System::Collections::Hashtable>,
        capsize: i32,
        runner: quest_hook::libil2cpp::Gc<
            crate::System::Text::RegularExpressions::ExclusiveReference,
        >,
        replref: quest_hook::libil2cpp::Gc<
            crate::System::WeakReference_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Text::RegularExpressions::RegexReplacement,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (key, capnames, capslist, code, caps, capsize, runner, replref),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::Regex_CachedCodeEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntryKey")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Regex_CachedCodeEntryKey {
    pub _options: crate::System::Text::RegularExpressions::RegexOptions,
    pub _cultureKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntryKey")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Text.RegularExpressions";
    const CLASS_NAME: &'static str = "CachedCodeEntryKey";
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
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntryKey")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntryKey")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey {
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
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntryKey")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey {
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
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntryKey")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey {
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
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntryKey")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntryKey")]
impl crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey {
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Regex_CachedCodeEntryKey1(
        &mut self,
        other: crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        options: crate::System::Text::RegularExpressions::RegexOptions,
        cultureKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pattern: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (options, cultureKey, pattern),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
        right: crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntryKey")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
    >,
> for crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
    > {
        todo!()
    }
}
#[cfg(feature = "System+Text+RegularExpressions+Regex+CachedCodeEntryKey")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
    >,
> for crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::System::Text::RegularExpressions::Regex_CachedCodeEntryKey,
    > {
        todo!()
    }
}
