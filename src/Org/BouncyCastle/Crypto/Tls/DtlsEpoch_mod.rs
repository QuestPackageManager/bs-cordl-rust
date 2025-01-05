#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsEpoch")]
#[repr(C)]
#[derive(Debug)]
pub struct DtlsEpoch {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mReplayWindow: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::DtlsReplayWindow,
    >,
    pub mEpoch: i32,
    pub mCipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
    >,
    pub mSequenceNumber: i64,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsEpoch")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Tls::DtlsEpoch =>
    "Org.BouncyCastle.Crypto.Tls"."DtlsEpoch"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsEpoch")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::DtlsEpoch {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsEpoch")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::DtlsEpoch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsEpoch")]
impl crate::Org::BouncyCastle::Crypto::Tls::DtlsEpoch {
    pub fn AllocateSequenceNumber(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("AllocateSequenceNumber", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        epoch: i32,
        cipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (epoch, cipher))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        epoch: i32,
        cipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (epoch, cipher))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Cipher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::Tls::TlsCipher>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::TlsCipher,
        > = __cordl_object.invoke("get_Cipher", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Epoch(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Epoch", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReplayWindow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsReplayWindow,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Tls::DtlsReplayWindow,
        > = __cordl_object.invoke("get_ReplayWindow", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SequenceNumber(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_SequenceNumber", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+DtlsEpoch")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::DtlsEpoch {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
