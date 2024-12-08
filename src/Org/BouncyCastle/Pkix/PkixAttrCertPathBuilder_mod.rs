#[cfg(feature = "Org+BouncyCastle+Pkix+PkixAttrCertPathBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixAttrCertPathBuilder {
    __cordl_parent: crate::System::Object,
    pub certPathException: *mut crate::System::Exception,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixAttrCertPathBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkix::PkixAttrCertPathBuilder
    => "Org.BouncyCastle.Pkix"."PkixAttrCertPathBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixAttrCertPathBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixAttrCertPathBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixAttrCertPathBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::PkixAttrCertPathBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixAttrCertPathBuilder")]
impl crate::Org::BouncyCastle::Pkix::PkixAttrCertPathBuilder {
    pub fn Build_PkixBuilderParameters0(
        &mut self,
        pkixParams: *mut crate::Org::BouncyCastle::Pkix::PkixBuilderParameters,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult = __cordl_object
            .invoke("Build", (pkixParams))?;
        Ok(__cordl_ret)
    }
    pub fn Build_IX509AttributeCertificate_X509Certificate_PkixBuilderParameters_IList1(
        &mut self,
        attrCert: *mut crate::Org::BouncyCastle::X509::IX509AttributeCertificate,
        tbvCert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        pkixParams: *mut crate::Org::BouncyCastle::Pkix::PkixBuilderParameters,
        tbvPath: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkix::PkixCertPathBuilderResult = __cordl_object
            .invoke("Build", (attrCert, tbvCert, pkixParams, tbvPath))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixAttrCertPathBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixAttrCertPathBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
