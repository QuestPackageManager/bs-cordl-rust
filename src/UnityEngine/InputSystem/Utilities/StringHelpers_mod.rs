#[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct StringHelpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::Utilities::StringHelpers =>
    "UnityEngine.InputSystem.Utilities"."StringHelpers"
);
#[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::StringHelpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::StringHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::StringHelpers {
    pub fn CharacterSeparatedListsHaveAtLeastOneCommonElement(
        firstList: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        secondList: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        separator: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CharacterSeparatedListsHaveAtLeastOneCommonElement",
                (firstList, secondList, separator),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Gc_StringComparison1(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        comparison: crate::System::StringComparison,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (str, text, comparison))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains__cordl_char0(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (str, ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn CountOccurrences(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CountOccurrences", (str, ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn Escape(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        replacements: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Escape", (str, chars, replacements))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpandTemplateString(
        _cordl_template: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mapFunc: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpandTemplateString", (_cordl_template, mapFunc))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromNicifiedMemorySize(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<i64>,
        defaultMultiplier: i64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromNicifiedMemorySize", (text, result, defaultMultiplier))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlural(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetPlural", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvariantEqualsIgnoreCase(
        left: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        right: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvariantEqualsIgnoreCase", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrintable(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPrintable", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn Join_Gc_Gc0<TValue>(
        separator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TValue>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Join", (separator, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn Join_Gc_Gc1<TValue>(
        values: quest_hook::libil2cpp::Gc<TValue>,
        separator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Join", (values, separator))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeUniqueName<TExisting>(
        baseName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        existingSet: quest_hook::libil2cpp::Gc<TExisting>,
        getNameFunc: quest_hook::libil2cpp::Gc<
            TExisting,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TExisting: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeUniqueName", (baseName, existingSet, getNameFunc))?;
        Ok(__cordl_ret.into())
    }
    pub fn NicifyMemorySize(
        numBytes: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NicifyMemorySize", (numBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseInt(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pos: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseInt", (str, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadStringFromBuffer_ByRefMut1(
        buffer: crate::System::IntPtr,
        bufferSize: i32,
        offset: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadStringFromBuffer", (buffer, bufferSize, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadStringFromBuffer_IntPtr_i32_0(
        buffer: crate::System::IntPtr,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadStringFromBuffer", (buffer, bufferSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn Split(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        predicate: quest_hook::libil2cpp::Gc<char, bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Split", (str, predicate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Tokenize(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::Utilities::Substring>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::Utilities::Substring,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Tokenize", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unescape(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        replacements: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Unescape", (str, chars, replacements))?;
        Ok(__cordl_ret.into())
    }
    pub fn WithAllWhitespaceStripped(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WithAllWhitespaceStripped", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStringToBuffer_ByRefMut1(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: crate::System::IntPtr,
        bufferSizeInCharacters: i32,
        offset: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "WriteStringToBuffer",
                (text, buffer, bufferSizeInCharacters, offset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStringToBuffer_Gc_IntPtr_i32_0(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        buffer: crate::System::IntPtr,
        bufferSizeInCharacters: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteStringToBuffer", (text, buffer, bufferSizeInCharacters))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+StringHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::StringHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
