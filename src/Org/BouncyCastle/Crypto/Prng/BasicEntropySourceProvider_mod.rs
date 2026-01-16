#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicEntropySourceProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mSecureRandom: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    pub mPredictionResistant: bool,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Prng";
    const CLASS_NAME: &'static str = "BasicEntropySourceProvider";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
impl crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider {
    #[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource")]
    pub type BasicEntropySource =
        crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource;
    pub fn Get(
        &mut self,
        bitsRequired: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IEntropySource>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::IEntropySource,
                        >,
                        1usize,
                    >("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Get",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IEntropySource,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (bitsRequired))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        secureRandom: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
        isPredictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom, isPredictionResistant))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        secureRandom: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
        isPredictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
                        bool,
                    ), quest_hook::libil2cpp::Void, 2usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (secureRandom, isPredictionResistant))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IEntropySourceProvider>
    for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider
{
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IEntropySourceProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IEntropySourceProvider>
    for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider
{
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IEntropySourceProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "cordl_class_Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BasicEntropySourceProvider_BasicEntropySource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mSecureRandom: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    pub mPredictionResistant: bool,
    pub mEntropySize: i32,
}
#[cfg(
    feature = "cordl_class_Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Prng";
    const CLASS_NAME: &'static str = "BasicEntropySourceProvider/BasicEntropySource";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource")]
impl std::ops::Deref
    for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource")]
impl std::ops::DerefMut
    for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource")]
impl crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource {
    pub fn New(
        secureRandom: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
        predictionResistant: bool,
        entropySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom, predictionResistant, entropySize))?;
        Ok(__cordl_object.into())
    }
    pub fn Org_BouncyCastle_Crypto_IEntropySource_GetEntropy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >,
                        0usize,
                    >("Org.BouncyCastle.Crypto.IEntropySource.GetEntropy")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Org.BouncyCastle.Crypto.IEntropySource.GetEntropy", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Org_BouncyCastle_Crypto_IEntropySource_get_EntropySize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>(
                        "Org.BouncyCastle.Crypto.IEntropySource.get_EntropySize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Org.BouncyCastle.Crypto.IEntropySource.get_EntropySize",
                            0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Org_BouncyCastle_Crypto_IEntropySource_get_IsPredictionResistant(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>(
                        "Org.BouncyCastle.Crypto.IEntropySource.get_IsPredictionResistant",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Org.BouncyCastle.Crypto.IEntropySource.get_IsPredictionResistant",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        secureRandom: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
        predictionResistant: bool,
        entropySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
                        bool,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (secureRandom, predictionResistant, entropySize))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IEntropySource>
    for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource
{
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IEntropySource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IEntropySource>
    for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource
{
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IEntropySource {
        unsafe { std::mem::transmute(self) }
    }
}
