#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct PKMacFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub parameters: *mut crate::Org::BouncyCastle::Asn1::Cmp::PbmParameter,
    pub key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crmf::PKMacFactory =>
    "Org.BouncyCastle.Crmf"."PKMacFactory"
);
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacFactory")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::PKMacFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacFactory")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crmf::PKMacFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacFactory")]
impl crate::Org::BouncyCastle::Crmf::PKMacFactory {
    pub fn CreateCalculator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IStreamCalculator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IStreamCalculator,
        > = __cordl_object.invoke("CreateCalculator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PbmParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PbmParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AlgorithmDetails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_AlgorithmDetails", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacFactory")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Crmf::PKMacFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacFactory")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IMacFactory>
for crate::Org::BouncyCastle::Crmf::PKMacFactory {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IMacFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacFactory")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IMacFactory>
for crate::Org::BouncyCastle::Crmf::PKMacFactory {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IMacFactory {
        unsafe { std::mem::transmute(self) }
    }
}
