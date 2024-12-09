#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformationStore")]
#[repr(C)]
#[derive(Debug)]
pub struct SignerInformationStore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub all: *mut crate::System::Collections::IList,
    pub table: *mut crate::System::Collections::IDictionary,
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformationStore")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::SignerInformationStore
    => "Org.BouncyCastle.Cms"."SignerInformationStore"
);
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformationStore")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::SignerInformationStore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformationStore")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::SignerInformationStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformationStore")]
impl crate::Org::BouncyCastle::Cms::SignerInformationStore {
    pub fn GetFirstSigner(
        &mut self,
        selector: *mut crate::Org::BouncyCastle::Cms::SignerID,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Cms::SignerInformation,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Cms::SignerInformation = __cordl_object
            .invoke("GetFirstSigner", (selector))?;
        Ok(__cordl_ret)
    }
    pub fn GetSigners_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("GetSigners", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSigners_SignerID1(
        &mut self,
        selector: *mut crate::Org::BouncyCastle::Cms::SignerID,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::ICollection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::ICollection = __cordl_object
            .invoke("GetSigners", (selector))?;
        Ok(__cordl_ret)
    }
    pub fn New_ICollection1(
        signerInfos: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signerInfos))?;
        Ok(__cordl_object)
    }
    pub fn New_SignerInformation0(
        signerInfo: *mut crate::Org::BouncyCastle::Cms::SignerInformation,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signerInfo))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_ICollection1(
        &mut self,
        signerInfos: *mut crate::System::Collections::ICollection,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signerInfos))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SignerInformation0(
        &mut self,
        signerInfo: *mut crate::Org::BouncyCastle::Cms::SignerInformation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signerInfo))?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+SignerInformationStore")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::SignerInformationStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
