#[cfg(feature = "Org+BouncyCastle+Crypto+IXof")]
#[repr(C)]
#[derive(Debug)]
pub struct IXof {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IXof")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::IXof =>
    "Org.BouncyCastle.Crypto"."IXof"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+IXof")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::IXof {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IXof")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::IXof {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IXof")]
impl crate::Org::BouncyCastle::Crypto::IXof {
    pub fn DoFinal(
        &mut self,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outOff: i32,
        outLen: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("DoFinal", (output, outOff, outLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoOutput(
        &mut self,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outOff: i32,
        outLen: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("DoOutput", (output, outOff, outLen))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IXof")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Crypto::IXof {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IXof")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IDigest>
for crate::Org::BouncyCastle::Crypto::IXof {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IDigest {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+IXof")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IDigest>
for crate::Org::BouncyCastle::Crypto::IXof {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IDigest {
        unsafe { std::mem::transmute(self) }
    }
}
