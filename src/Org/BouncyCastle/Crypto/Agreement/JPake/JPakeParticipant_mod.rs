#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeParticipant")]
#[repr(C)]
#[derive(Debug)]
pub struct JPakeParticipant {
    __cordl_parent: crate::System::Object,
    pub participantId: *mut crate::System::String,
    pub password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub p: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub q: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub g: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub partnerParticipantId: *mut crate::System::String,
    pub x1: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub x2: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub gx1: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub gx2: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub gx3: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub gx4: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub b: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub state: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeParticipant")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeParticipant =>
    "Org.BouncyCastle.Crypto.Agreement.JPake"."JPakeParticipant"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeParticipant")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeParticipant {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeParticipant")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeParticipant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeParticipant")]
impl crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeParticipant {
    pub fn CalculateKeyingMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("CalculateKeyingMaterial", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray0(
        &mut self,
        participantId: *mut crate::System::String,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (participantId, password))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_JPakePrimeOrderGroup1(
        &mut self,
        participantId: *mut crate::System::String,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        group: *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakePrimeOrderGroup,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (participantId, password, group))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_JPakePrimeOrderGroup_IDigest_SecureRandom2(
        &mut self,
        participantId: *mut crate::System::String,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        group: *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakePrimeOrderGroup,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (participantId, password, group, digest, random))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateRound1PayloadReceived(
        &mut self,
        round1PayloadReceived: *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound1Payload,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateRound1PayloadReceived", (round1PayloadReceived))?;
        Ok(__cordl_ret)
    }
    pub fn CreateRound2PayloadToSend(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound2Payload,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound2Payload = __cordl_object
            .invoke("CreateRound2PayloadToSend", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateRound1PayloadToSend(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound1Payload,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound1Payload = __cordl_object
            .invoke("CreateRound1PayloadToSend", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateRound3PayloadReceived(
        &mut self,
        round3PayloadReceived: *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound3Payload,
        keyingMaterial: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ValidateRound3PayloadReceived",
                (round3PayloadReceived, keyingMaterial),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_State(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_State", ())?;
        Ok(__cordl_ret)
    }
    pub fn ValidateRound2PayloadReceived(
        &mut self,
        round2PayloadReceived: *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound2Payload,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateRound2PayloadReceived", (round2PayloadReceived))?;
        Ok(__cordl_ret)
    }
    pub fn CreateRound3PayloadToSend(
        &mut self,
        keyingMaterial: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound3Payload,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeRound3Payload = __cordl_object
            .invoke("CreateRound3PayloadToSend", (keyingMaterial))?;
        Ok(__cordl_ret)
    }
    pub fn New_String_Il2CppArray0(
        participantId: *mut crate::System::String,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (participantId, password))?;
        Ok(__cordl_object)
    }
    pub fn New_JPakePrimeOrderGroup1(
        participantId: *mut crate::System::String,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        group: *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakePrimeOrderGroup,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (participantId, password, group))?;
        Ok(__cordl_object)
    }
    pub fn New_JPakePrimeOrderGroup_IDigest_SecureRandom2(
        participantId: *mut crate::System::String,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        group: *mut crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakePrimeOrderGroup,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (participantId, password, group, digest, random))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+JPake+JPakeParticipant")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::JPake::JPakeParticipant {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
