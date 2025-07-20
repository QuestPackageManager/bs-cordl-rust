#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpLiteralDataGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub pkOut: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
    >,
    pub oldFormat: bool,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpLiteralDataGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "PgpLiteralDataGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpLiteralDataGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpLiteralDataGenerator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpLiteralDataGenerator {
    pub const Binary: char = 'b';
    pub const Console: &'static str = "_CONSOLE";
    pub const Text: char = 't';
    pub const Utf8: char = 'u';
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Close")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Close",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool1(
        oldFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oldFormat))?;
        Ok(__cordl_object.into())
    }
    pub fn Open_FileInfo2(
        &mut self,
        outStr: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        format: char,
        file: quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            char,
                            quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        3usize,
                    >("Open")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Open",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            cordl_method_info.invoke_unchecked(self, (outStr, format, file))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Open_Il2CppString_DateTime_Il2CppArray1(
        &mut self,
        outStr: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        format: char,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        modificationTime: crate::System::DateTime,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            char,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::DateTime,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        5usize,
                    >("Open")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Open",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (outStr, format, name, modificationTime, buffer),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Open_Il2CppString_i64_DateTime0(
        &mut self,
        outStr: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        format: char,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        length: i64,
        modificationTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            char,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            i64,
                            crate::System::DateTime,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        5usize,
                    >("Open")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Open",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (outStr, format, name, length, modificationTime),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteHeader(
        &mut self,
        outStr: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
        >,
        format: char,
        encName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        modificationTime: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
                            >,
                            char,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i64,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("WriteHeader")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteHeader", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (outStr, format, encName, modificationTime))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        oldFormat: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (oldFormat))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpLiteralDataGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
impl AsRef<crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator>
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpLiteralDataGenerator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpLiteralDataGenerator")]
impl AsMut<crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator>
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpLiteralDataGenerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
