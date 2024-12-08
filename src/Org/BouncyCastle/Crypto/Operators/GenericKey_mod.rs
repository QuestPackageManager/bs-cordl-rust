#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+GenericKey")]
#[repr(C)]
#[derive(Debug)]
pub struct GenericKey {
    __cordl_parent: crate::System::Object,
    pub algorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub representation: *mut crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+GenericKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Operators::GenericKey
    => "Org.BouncyCastle.Crypto.Operators"."GenericKey"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+GenericKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Operators::GenericKey {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+GenericKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Operators::GenericKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+GenericKey")]
impl crate::Org::BouncyCastle::Crypto::Operators::GenericKey {
    pub fn get_AlgorithmIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_AlgorithmIdentifier", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object0(
        &mut self,
        representation: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (representation))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AlgorithmIdentifier_Il2CppArray1(
        &mut self,
        algorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        representation: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithmIdentifier, representation))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AlgorithmIdentifier_Object2(
        &mut self,
        algorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        representation: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithmIdentifier, representation))?;
        Ok(__cordl_ret)
    }
    pub fn get_Representation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Representation", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Object0(
        representation: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (representation))?;
        Ok(__cordl_object)
    }
    pub fn New_AlgorithmIdentifier_Il2CppArray1(
        algorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        representation: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithmIdentifier, representation))?;
        Ok(__cordl_object)
    }
    pub fn New_AlgorithmIdentifier_Object2(
        algorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        representation: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithmIdentifier, representation))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+GenericKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::GenericKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
