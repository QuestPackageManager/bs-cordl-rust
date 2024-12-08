#[cfg(feature = "Org+BouncyCastle+Asn1+BerOutputStream")]
#[repr(C)]
#[derive(Debug)]
pub struct BerOutputStream {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerOutputStream,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerOutputStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::BerOutputStream =>
    "Org.BouncyCastle.Asn1"."BerOutputStream"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+BerOutputStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::BerOutputStream {
    type Target = crate::Org::BouncyCastle::Asn1::DerOutputStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerOutputStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::BerOutputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerOutputStream")]
impl crate::Org::BouncyCastle::Asn1::BerOutputStream {
    pub fn New(
        os: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (os))?;
        Ok(__cordl_object)
    }
    pub fn WriteObject(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteObject", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        os: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (os))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerOutputStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::BerOutputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}