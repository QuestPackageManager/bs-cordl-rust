#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpCompressedDataGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub algorithm: crate::Org::BouncyCastle::Bcpg::CompressionAlgorithmTag,
    pub compression: i32,
    pub dOut: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub pkOut: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "PgpCompressedDataGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator {
    #[cfg(
        feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeCBZip2OutputStream"
    )]
    pub type SafeCBZip2OutputStream = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator_SafeCBZip2OutputStream;
    #[cfg(
        feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeZOutputStream"
    )]
    pub type SafeZOutputStream = crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator_SafeZOutputStream;
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Close")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Close", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_CompressionAlgorithmTag0(
        algorithm: crate::Org::BouncyCastle::Bcpg::CompressionAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        algorithm: crate::Org::BouncyCastle::Bcpg::CompressionAlgorithmTag,
        compression: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, compression))?;
        Ok(__cordl_object.into())
    }
    pub fn Open_Il2CppArray1(
        &mut self,
        outStr: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        2usize,
                    >("Open")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Open", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            method.invoke_unchecked(self, (outStr, buffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Open_Stream0(
        &mut self,
        outStr: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        1usize,
                    >("Open")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Open", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            method.invoke_unchecked(self, (outStr))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_CompressionAlgorithmTag0(
        &mut self,
        algorithm: crate::Org::BouncyCastle::Bcpg::CompressionAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::Org::BouncyCastle::Bcpg::CompressionAlgorithmTag),
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
            method.invoke_unchecked(self, (algorithm))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        algorithm: crate::Org::BouncyCastle::Bcpg::CompressionAlgorithmTag,
        compression: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::Org::BouncyCastle::Bcpg::CompressionAlgorithmTag, i32),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (algorithm, compression))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn doOpen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("doOpen")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "doOpen", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator")]
impl AsRef<crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator>
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator")]
impl AsMut<crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator>
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Bcpg::OpenPgp::IStreamGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeCBZip2OutputStream"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PgpCompressedDataGenerator_SafeCBZip2OutputStream {
    __cordl_parent: crate::Org::BouncyCastle::Apache::Bzip2::CBZip2OutputStream,
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeCBZip2OutputStream"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator_SafeCBZip2OutputStream {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "PgpCompressedDataGenerator/SafeCBZip2OutputStream";
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
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeCBZip2OutputStream"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator_SafeCBZip2OutputStream {
    type Target = crate::Org::BouncyCastle::Apache::Bzip2::CBZip2OutputStream;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeCBZip2OutputStream"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator_SafeCBZip2OutputStream {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeCBZip2OutputStream"
)]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator_SafeCBZip2OutputStream {
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Close")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Close", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (output))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IO::Stream>),
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
            method.invoke_unchecked(self, (output))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeCBZip2OutputStream"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator_SafeCBZip2OutputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeZOutputStream"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PgpCompressedDataGenerator_SafeZOutputStream {
    __cordl_parent: crate::Org::BouncyCastle::Utilities::Zlib::ZOutputStream,
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeZOutputStream"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator_SafeZOutputStream {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg.OpenPgp";
    const CLASS_NAME: &'static str = "PgpCompressedDataGenerator/SafeZOutputStream";
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
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeZOutputStream"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator_SafeZOutputStream {
    type Target = crate::Org::BouncyCastle::Utilities::Zlib::ZOutputStream;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeZOutputStream"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator_SafeZOutputStream {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeZOutputStream"
)]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator_SafeZOutputStream {
    pub fn Close(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Close")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Close", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        level: i32,
        nowrap: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (output, level, nowrap))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        level: i32,
        nowrap: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            i32,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (output, level, nowrap))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpCompressedDataGenerator+SafeZOutputStream"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpCompressedDataGenerator_SafeZOutputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
