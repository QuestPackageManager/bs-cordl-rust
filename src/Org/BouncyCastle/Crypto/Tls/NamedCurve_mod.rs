#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+NamedCurve")]
#[repr(C)]
#[derive(Debug)]
pub struct NamedCurve {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+NamedCurve")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Tls::NamedCurve {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "NamedCurve";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+NamedCurve")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::NamedCurve {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+NamedCurve")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::NamedCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+NamedCurve")]
impl crate::Org::BouncyCastle::Crypto::Tls::NamedCurve {
    pub const arbitrary_explicit_char2_curves: i32 = 65282i32;
    pub const arbitrary_explicit_prime_curves: i32 = 65281i32;
    pub const brainpoolP256r1: i32 = 26i32;
    pub const brainpoolP384r1: i32 = 27i32;
    pub const brainpoolP512r1: i32 = 28i32;
    pub const secp160k1: i32 = 15i32;
    pub const secp160r1: i32 = 16i32;
    pub const secp160r2: i32 = 17i32;
    pub const secp192k1: i32 = 18i32;
    pub const secp192r1: i32 = 19i32;
    pub const secp224k1: i32 = 20i32;
    pub const secp224r1: i32 = 21i32;
    pub const secp256k1: i32 = 22i32;
    pub const secp256r1: i32 = 23i32;
    pub const secp384r1: i32 = 24i32;
    pub const secp521r1: i32 = 25i32;
    pub const sect163k1: i32 = 1i32;
    pub const sect163r1: i32 = 2i32;
    pub const sect163r2: i32 = 3i32;
    pub const sect193r1: i32 = 4i32;
    pub const sect193r2: i32 = 5i32;
    pub const sect233k1: i32 = 6i32;
    pub const sect233r1: i32 = 7i32;
    pub const sect239k1: i32 = 8i32;
    pub const sect283k1: i32 = 9i32;
    pub const sect283r1: i32 = 10i32;
    pub const sect409k1: i32 = 11i32;
    pub const sect409r1: i32 = 12i32;
    pub const sect571k1: i32 = 13i32;
    pub const sect571r1: i32 = 14i32;
    pub fn IsValid(namedCurve: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Tls::NamedCurve as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), bool, 1usize>("IsValid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Tls::NamedCurve as
                    quest_hook::libil2cpp::Type > ::class(), "IsValid", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (namedCurve))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefersToASpecificNamedCurve(
        namedCurve: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Tls::NamedCurve as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), bool, 1usize>("RefersToASpecificNamedCurve")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Tls::NamedCurve as
                    quest_hook::libil2cpp::Type > ::class(),
                    "RefersToASpecificNamedCurve", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (namedCurve))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Tls::NamedCurve as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Tls::NamedCurve as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+NamedCurve")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::NamedCurve {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
