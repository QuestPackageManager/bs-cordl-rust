#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables8kGcmMultiplier")]
#[repr(C)]
#[derive(Debug)]
pub struct Tables8kGcmMultiplier {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub H: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub M: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
                >,
            >,
        >,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables8kGcmMultiplier")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables8kGcmMultiplier {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Modes.Gcm";
    const CLASS_NAME: &'static str = "Tables8kGcmMultiplier";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables8kGcmMultiplier")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables8kGcmMultiplier {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables8kGcmMultiplier")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables8kGcmMultiplier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables8kGcmMultiplier")]
impl crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables8kGcmMultiplier {
    pub fn Init(
        &mut self,
        H: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (H))
        };
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyH(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("MultiplyH")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MultiplyH", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (x))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables8kGcmMultiplier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables8kGcmMultiplier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables8kGcmMultiplier")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Modes::Gcm::IGcmMultiplier>
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables8kGcmMultiplier {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Modes::Gcm::IGcmMultiplier {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables8kGcmMultiplier")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Modes::Gcm::IGcmMultiplier>
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables8kGcmMultiplier {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Modes::Gcm::IGcmMultiplier {
        unsafe { std::mem::transmute(self) }
    }
}
