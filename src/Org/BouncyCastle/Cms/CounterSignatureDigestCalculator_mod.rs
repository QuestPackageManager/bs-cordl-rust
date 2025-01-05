#[cfg(feature = "Org+BouncyCastle+Cms+CounterSignatureDigestCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct CounterSignatureDigestCalculator {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub alg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CounterSignatureDigestCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CounterSignatureDigestCalculator => "Org.BouncyCastle.Cms"
    ."CounterSignatureDigestCalculator"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CounterSignatureDigestCalculator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Cms::CounterSignatureDigestCalculator {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CounterSignatureDigestCalculator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::CounterSignatureDigestCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CounterSignatureDigestCalculator")]
impl crate::Org::BouncyCastle::Cms::CounterSignatureDigestCalculator {
    pub fn GetDigest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetDigest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        alg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (alg, data))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        alg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (alg, data))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CounterSignatureDigestCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CounterSignatureDigestCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CounterSignatureDigestCalculator")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::IDigestCalculator>>
for crate::Org::BouncyCastle::Cms::CounterSignatureDigestCalculator {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::IDigestCalculator> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CounterSignatureDigestCalculator")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Cms::IDigestCalculator>>
for crate::Org::BouncyCastle::Cms::CounterSignatureDigestCalculator {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::IDigestCalculator,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
