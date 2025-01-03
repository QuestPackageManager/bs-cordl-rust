#[cfg(feature = "Org+BouncyCastle+Crypto+IDsaExt")]
#[repr(C)]
#[derive(Debug)]
pub struct IDsaExt {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDsaExt")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::IDsaExt =>
    "Org.BouncyCastle.Crypto"."IDsaExt"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+IDsaExt")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::IDsaExt {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDsaExt")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::IDsaExt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDsaExt")]
impl crate::Org::BouncyCastle::Crypto::IDsaExt {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Order(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Order", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDsaExt")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Crypto::IDsaExt {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDsaExt")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IDsa>
for crate::Org::BouncyCastle::Crypto::IDsaExt {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IDsa {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IDsaExt")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IDsa>
for crate::Org::BouncyCastle::Crypto::IDsaExt {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IDsa {
        unsafe { std::mem::transmute(self) }
    }
}
