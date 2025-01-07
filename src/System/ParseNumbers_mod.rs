#[cfg(feature = "System+ParseNumbers")]
#[repr(C)]
#[derive(Debug)]
pub struct ParseNumbers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ParseNumbers")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::ParseNumbers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "ParseNumbers";
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
#[cfg(feature = "System+ParseNumbers")]
impl std::ops::Deref for crate::System::ParseNumbers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ParseNumbers")]
impl std::ops::DerefMut for crate::System::ParseNumbers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ParseNumbers")]
impl crate::System::ParseNumbers {
    pub fn EatWhiteSpace(
        s: crate::System::ReadOnlySpan_1<char>,
        i: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EatWhiteSpace", (s, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn GrabInts(
        radix: i32,
        s: crate::System::ReadOnlySpan_1<char>,
        i: quest_hook::libil2cpp::ByRefMut<i32>,
        isUnsigned: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GrabInts", (radix, s, i, isUnsigned))?;
        Ok(__cordl_ret.into())
    }
    pub fn GrabLongs(
        radix: i32,
        s: crate::System::ReadOnlySpan_1<char>,
        i: quest_hook::libil2cpp::ByRefMut<i32>,
        isUnsigned: bool,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GrabLongs", (radix, s, i, isUnsigned))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntToString(
        n: i32,
        radix: i32,
        width: i32,
        paddingChar: char,
        flags: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntToString", (n, radix, width, paddingChar, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDigit(
        c: char,
        radix: i32,
        result: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDigit", (c, radix, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn LongToString(
        n: i64,
        radix: i32,
        width: i32,
        paddingChar: char,
        flags: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LongToString", (n, radix, width, paddingChar, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToInt_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        radix: i32,
        flags: i32,
        currPos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringToInt", (s, radix, flags, currPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToInt_ReadOnlySpan_1_i32_i32_0(
        s: crate::System::ReadOnlySpan_1<char>,
        radix: i32,
        flags: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringToInt", (s, radix, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToLong_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        radix: i32,
        flags: i32,
        currPos: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringToLong", (s, radix, flags, currPos))?;
        Ok(__cordl_ret.into())
    }
    pub fn StringToLong_ReadOnlySpan_1_i32_i32_0(
        s: crate::System::ReadOnlySpan_1<char>,
        radix: i32,
        flags: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StringToLong", (s, radix, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowOverflowInt32Exception() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowOverflowInt32Exception", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowOverflowInt64Exception() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowOverflowInt64Exception", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowOverflowUInt32Exception() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowOverflowUInt32Exception", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowOverflowUInt64Exception() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowOverflowUInt64Exception", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ParseNumbers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ParseNumbers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
