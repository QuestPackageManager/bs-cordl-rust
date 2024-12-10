#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeRound1Payload")]
#[repr(C)]
#[derive(Debug)]
pub struct JPakeRound1Payload {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub participantId: *mut quest_hook::libil2cpp::Il2CppString,
    pub gx1: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub gx2: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub knowledgeProofForX1: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Org::BouncyCastle::Math::BigInteger,
    >,
    pub knowledgeProofForX2: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Org::BouncyCastle::Math::BigInteger,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeRound1Payload")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound1Payload =>
    "Org.BouncyCastle.Crypto.Agreement.JPake"."JPakeRound1Payload"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeRound1Payload")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound1Payload {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeRound1Payload")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound1Payload {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeRound1Payload")]
impl crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound1Payload {
    pub fn New(
        participantId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gx1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        gx2: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        knowledgeProofForX1: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
        knowledgeProofForX2: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (participantId, gx1, gx2, knowledgeProofForX1, knowledgeProofForX2),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        participantId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gx1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        gx2: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        knowledgeProofForX1: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
        knowledgeProofForX2: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (participantId, gx1, gx2, knowledgeProofForX1, knowledgeProofForX2),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Gx1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Gx1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Gx2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Gx2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KnowledgeProofForX1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        > = __cordl_object.invoke("get_KnowledgeProofForX1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KnowledgeProofForX2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        > = __cordl_object.invoke("get_KnowledgeProofForX2", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeRound1Payload")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound1Payload {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
