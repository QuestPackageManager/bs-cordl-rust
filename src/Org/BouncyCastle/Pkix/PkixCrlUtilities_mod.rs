#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCrlUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct PkixCrlUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCrlUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkix::PkixCrlUtilities =>
    "Org.BouncyCastle.Pkix"."PkixCrlUtilities"
);
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCrlUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkix::PkixCrlUtilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCrlUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkix::PkixCrlUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCrlUtilities")]
impl crate::Org::BouncyCastle::Pkix::PkixCrlUtilities {
    pub fn FindCrls_PkixParameters_DateTime0(
        &mut self,
        crlselect: *mut crate::Org::BouncyCastle::X509::Store::X509CrlStoreSelector,
        paramsPkix: *mut crate::Org::BouncyCastle::Pkix::PkixParameters,
        currentDate: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("FindCrls", (crlselect, paramsPkix, currentDate))?;
        Ok(__cordl_ret)
    }
    pub fn FindCrls_PkixParameters1(
        &mut self,
        crlselect: *mut crate::Org::BouncyCastle::X509::Store::X509CrlStoreSelector,
        paramsPkix: *mut crate::Org::BouncyCastle::Pkix::PkixParameters,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Utilities::Collections::ISet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Utilities::Collections::ISet = __cordl_object
            .invoke("FindCrls", (crlselect, paramsPkix))?;
        Ok(__cordl_ret)
    }
    pub fn FindCrls_IList2(
        &mut self,
        crlSelect: *mut crate::Org::BouncyCastle::X509::Store::X509CrlStoreSelector,
        crlStores: *mut crate::System::Collections::IList,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("FindCrls", (crlSelect, crlStores))?;
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
#[cfg(feature = "Org+BouncyCastle+Pkix+PkixCrlUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkix::PkixCrlUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
