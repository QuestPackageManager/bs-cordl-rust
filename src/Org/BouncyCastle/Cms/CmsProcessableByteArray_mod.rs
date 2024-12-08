#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableByteArray")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsProcessableByteArray {
    __cordl_parent: crate::System::Object,
    pub _cordl_type: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableByteArray")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsProcessableByteArray
    => "Org.BouncyCastle.Cms"."CmsProcessableByteArray"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableByteArray")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsProcessableByteArray {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableByteArray")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsProcessableByteArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableByteArray")]
impl crate::Org::BouncyCastle::Cms::CmsProcessableByteArray {
    pub fn Write(
        &mut self,
        zOut: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (zOut))?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray0(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bytes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerObjectIdentifier_Il2CppArray1(
        &mut self,
        _cordl_type: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, bytes))?;
        Ok(__cordl_ret)
    }
    pub fn GetContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetInputStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("GetInputStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Il2CppArray0(
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bytes))?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier_Il2CppArray1(
        _cordl_type: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, bytes))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableByteArray")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsProcessableByteArray {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
