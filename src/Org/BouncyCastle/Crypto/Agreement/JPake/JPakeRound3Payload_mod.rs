#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeRound3Payload")]
#[repr(C)]
#[derive(Debug)]
pub struct JPakeRound3Payload {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub participantId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub macTag: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeRound3Payload")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound3Payload =>
    "Org.BouncyCastle.Crypto.Agreement.JPake"."JPakeRound3Payload"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeRound3Payload")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound3Payload {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeRound3Payload")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound3Payload {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeRound3Payload")]
impl crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound3Payload {
    pub fn New(
        participantId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        magTag: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (participantId, magTag))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        participantId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        magTag: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (participantId, magTag))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MacTag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_MacTag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParticipantId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ParticipantId", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeRound3Payload")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound3Payload {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
