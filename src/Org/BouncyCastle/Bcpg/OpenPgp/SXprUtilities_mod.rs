#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct SXprUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "SXprUtilities";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities {
    #[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
    pub type MyS2k = crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ParseS2k(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
                1usize,
            >("ParseS2k")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "ParseS2k", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::S2k,
        > = unsafe { method.invoke_unchecked((), (input))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadBytes(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        ch: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("ReadBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "ReadBytes", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (input, ch))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadLength(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        ch: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>, i32),
                i32,
                2usize,
            >("ReadLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "ReadLength", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (input, ch))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReadString(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        ch: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>, i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ReadString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "ReadString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (input, ch))? };
        Ok(__cordl_ret.into())
    }
    pub fn SkipCloseParenthesis(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SkipCloseParenthesis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "SkipCloseParenthesis",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SkipOpenParenthesis(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SkipOpenParenthesis")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as
                    quest_hook::libil2cpp::Type > ::class(), "SkipOpenParenthesis",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
#[repr(C)]
#[derive(Debug)]
pub struct SXprUtilities_MyS2k {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::S2k,
    pub mIterationCount64: i64,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "SXprUtilities/MyS2k";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k {
    type Target = crate::Org::BouncyCastle::Bcpg::S2k;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k {
    pub fn New(
        algorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iterationCount64: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, iv, iterationCount64))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        algorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        iterationCount64: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i64,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (algorithm, iv, iterationCount64))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IterationCount(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i64, 0usize>("get_IterationCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k as
                    quest_hook::libil2cpp::Type > ::class(), "get_IterationCount", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+SXprUtilities+MyS2k")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::SXprUtilities_MyS2k {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
