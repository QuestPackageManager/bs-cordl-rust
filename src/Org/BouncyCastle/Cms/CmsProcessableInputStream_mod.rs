#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableInputStream")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsProcessableInputStream {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub used: bool,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableInputStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Cms::CmsProcessableInputStream => "Org.BouncyCastle.Cms"
    ."CmsProcessableInputStream"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableInputStream")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsProcessableInputStream {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableInputStream")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsProcessableInputStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableInputStream")]
impl crate::Org::BouncyCastle::Cms::CmsProcessableInputStream {
    pub fn CheckSingleUsage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckSingleUsage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetContent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInputStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = __cordl_object
            .invoke("GetInputStream", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (input))?;
        Ok(__cordl_object.into())
    }
    pub fn Write(
        &mut self,
        output: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        input: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (input))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableInputStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsProcessableInputStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableInputStream")]
impl AsRef<crate::Org::BouncyCastle::Cms::CmsProcessable>
for crate::Org::BouncyCastle::Cms::CmsProcessableInputStream {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Cms::CmsProcessable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableInputStream")]
impl AsMut<crate::Org::BouncyCastle::Cms::CmsProcessable>
for crate::Org::BouncyCastle::Cms::CmsProcessableInputStream {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Cms::CmsProcessable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableInputStream")]
impl AsRef<crate::Org::BouncyCastle::Cms::CmsReadable>
for crate::Org::BouncyCastle::Cms::CmsProcessableInputStream {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Cms::CmsReadable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableInputStream")]
impl AsMut<crate::Org::BouncyCastle::Cms::CmsReadable>
for crate::Org::BouncyCastle::Cms::CmsProcessableInputStream {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Cms::CmsReadable {
        unsafe { std::mem::transmute(self) }
    }
}
