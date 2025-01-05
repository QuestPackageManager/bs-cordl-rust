#[cfg(feature = "System+Security+Cryptography+DSASignatureDeformatter")]
#[repr(C)]
#[derive(Debug)]
pub struct DSASignatureDeformatter {
    __cordl_parent: crate::System::Security::Cryptography::AsymmetricSignatureDeformatter,
    pub _dsaKey: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::DSA>,
    pub _oid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Security+Cryptography+DSASignatureDeformatter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::DSASignatureDeformatter =>
    "System.Security.Cryptography"."DSASignatureDeformatter"
);
#[cfg(feature = "System+Security+Cryptography+DSASignatureDeformatter")]
impl std::ops::Deref for crate::System::Security::Cryptography::DSASignatureDeformatter {
    type Target = crate::System::Security::Cryptography::AsymmetricSignatureDeformatter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+DSASignatureDeformatter")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::DSASignatureDeformatter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+DSASignatureDeformatter")]
impl crate::System::Security::Cryptography::DSASignatureDeformatter {
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_AsymmetricAlgorithm1(
        key: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsymmetricAlgorithm,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key))?;
        Ok(__cordl_object.into())
    }
    pub fn SetHashAlgorithm(
        &mut self,
        strName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHashAlgorithm", (strName))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKey(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsymmetricAlgorithm,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKey", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifySignature(
        &mut self,
        rgbHash: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        rgbSignature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VerifySignature", (rgbHash, rgbSignature))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AsymmetricAlgorithm1(
        &mut self,
        key: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsymmetricAlgorithm,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+DSASignatureDeformatter")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::DSASignatureDeformatter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
