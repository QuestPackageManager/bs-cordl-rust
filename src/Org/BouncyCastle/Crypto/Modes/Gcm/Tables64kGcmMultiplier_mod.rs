#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables64kGcmMultiplier")]
#[repr(C)]
#[derive(Debug)]
pub struct Tables64kGcmMultiplier {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub H: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub M: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<u32>,
            >,
        >,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables64kGcmMultiplier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Modes::Gcm::Tables64kGcmMultiplier =>
    "Org.BouncyCastle.Crypto.Modes.Gcm"."Tables64kGcmMultiplier"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables64kGcmMultiplier")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables64kGcmMultiplier {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables64kGcmMultiplier")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables64kGcmMultiplier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables64kGcmMultiplier")]
impl crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables64kGcmMultiplier {
    pub fn Init(
        &mut self,
        H: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (H))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyH(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MultiplyH", (x))?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables64kGcmMultiplier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables64kGcmMultiplier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables64kGcmMultiplier")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Modes::Gcm::IGcmMultiplier>
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables64kGcmMultiplier {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Modes::Gcm::IGcmMultiplier {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables64kGcmMultiplier")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Modes::Gcm::IGcmMultiplier>
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables64kGcmMultiplier {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Modes::Gcm::IGcmMultiplier {
        unsafe { std::mem::transmute(self) }
    }
}
