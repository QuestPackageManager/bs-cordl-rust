#[cfg(feature = "Org+BouncyCastle+Asn1+BerSetGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct BerSetGenerator {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::BerGenerator,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerSetGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::BerSetGenerator =>
    "Org.BouncyCastle.Asn1"."BerSetGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+BerSetGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::BerSetGenerator {
    type Target = crate::Org::BouncyCastle::Asn1::BerGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerSetGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::BerSetGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerSetGenerator")]
impl crate::Org::BouncyCastle::Asn1::BerSetGenerator {
    pub fn New_Stream0(
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outStream))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32__cordl_bool1(
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        tagNo: i32,
        isExplicit: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outStream, tagNo, isExplicit))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Stream0(
        &mut self,
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (outStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32__cordl_bool1(
        &mut self,
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        tagNo: i32,
        isExplicit: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (outStream, tagNo, isExplicit))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BerSetGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::BerSetGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
