#[cfg(feature = "System+UriHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct UriHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+UriHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::UriHelper => "System"."UriHelper"
);
#[cfg(feature = "System+UriHelper")]
impl std::ops::Deref for crate::System::UriHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+UriHelper")]
impl std::ops::DerefMut for crate::System::UriHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+UriHelper")]
impl crate::System::UriHelper {
    pub fn EnsureDestinationSize(
        pStr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        currentInputPos: i32,
        charsToAdd: i16,
        minReallocateChars: i16,
        destPos: quest_hook::libil2cpp::ByRefMut<i32>,
        prevInputPos: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EnsureDestinationSize",
                (
                    pStr,
                    dest,
                    currentInputPos,
                    charsToAdd,
                    minReallocateChars,
                    destPos,
                    prevInputPos,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EscapeAsciiChar(
        ch: char,
        to: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        pos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EscapeAsciiChar", (ch, to, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn EscapeString(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        end: i32,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        destPos: quest_hook::libil2cpp::ByRefMut<i32>,
        isUriString: bool,
        force1: char,
        force2: char,
        rsvd: char,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EscapeString",
                (input, start, end, dest, destPos, isUriString, force1, force2, rsvd),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EscapedAscii(digit: char, next: char) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EscapedAscii", (digit, next))?;
        Ok(__cordl_ret.into())
    }
    pub fn Is3986Unreserved(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Is3986Unreserved", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNotSafeForUnescape(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNotSafeForUnescape", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsReservedUnreservedOrHash(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsReservedUnreservedOrHash", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsUnreserved(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsUnreserved", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchUTF8Sequence(
        pDest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        destOffset: quest_hook::libil2cpp::ByRefMut<i32>,
        unescapedChars: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        >,
        charCount: i32,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        byteCount: i32,
        isQuery: bool,
        iriParsing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "MatchUTF8Sequence",
                (
                    pDest,
                    dest,
                    destOffset,
                    unescapedChars,
                    charCount,
                    bytes,
                    byteCount,
                    isQuery,
                    iriParsing,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TestForSubPath(
        pMe: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        meLength: u16,
        pShe: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        sheLength: u16,
        ignoreCase: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TestForSubPath", (pMe, meLength, pShe, sheLength, ignoreCase))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnescapeString_Il2CppObject1(
        pStr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        start: i32,
        end: i32,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        destPosition: quest_hook::libil2cpp::ByRefMut<i32>,
        rsvd1: char,
        rsvd2: char,
        rsvd3: char,
        unescapeMode: crate::System::UnescapeMode,
        syntax: quest_hook::libil2cpp::Gc<crate::System::UriParser>,
        isQuery: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "UnescapeString",
                (
                    pStr,
                    start,
                    end,
                    dest,
                    destPosition,
                    rsvd1,
                    rsvd2,
                    rsvd3,
                    unescapeMode,
                    syntax,
                    isQuery,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UnescapeString_Il2CppString0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        start: i32,
        end: i32,
        dest: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        destPosition: quest_hook::libil2cpp::ByRefMut<i32>,
        rsvd1: char,
        rsvd2: char,
        rsvd3: char,
        unescapeMode: crate::System::UnescapeMode,
        syntax: quest_hook::libil2cpp::Gc<crate::System::UriParser>,
        isQuery: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "UnescapeString",
                (
                    input,
                    start,
                    end,
                    dest,
                    destPosition,
                    rsvd1,
                    rsvd2,
                    rsvd3,
                    unescapeMode,
                    syntax,
                    isQuery,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+UriHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::System::UriHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
