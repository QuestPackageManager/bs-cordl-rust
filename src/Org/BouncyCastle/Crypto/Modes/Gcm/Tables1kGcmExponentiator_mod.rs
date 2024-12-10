#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables1kGcmExponentiator")]
#[repr(C)]
#[derive(Debug)]
pub struct Tables1kGcmExponentiator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub lookupPowX2: *mut crate::System::Collections::IList,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables1kGcmExponentiator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Modes::Gcm::Tables1kGcmExponentiator =>
    "Org.BouncyCastle.Crypto.Modes.Gcm"."Tables1kGcmExponentiator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables1kGcmExponentiator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables1kGcmExponentiator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables1kGcmExponentiator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables1kGcmExponentiator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables1kGcmExponentiator")]
impl crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables1kGcmExponentiator {
    pub fn EnsureAvailable(
        &mut self,
        bit: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnsureAvailable", (bit))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExponentiateX(
        &mut self,
        pow: i64,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExponentiateX", (pow, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        x: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (x))?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables1kGcmExponentiator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables1kGcmExponentiator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables1kGcmExponentiator")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Modes::Gcm::IGcmExponentiator>
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables1kGcmExponentiator {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Crypto::Modes::Gcm::IGcmExponentiator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Modes+Gcm+Tables1kGcmExponentiator")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Modes::Gcm::IGcmExponentiator>
for crate::Org::BouncyCastle::Crypto::Modes::Gcm::Tables1kGcmExponentiator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Modes::Gcm::IGcmExponentiator {
        unsafe { std::mem::transmute(self) }
    }
}
