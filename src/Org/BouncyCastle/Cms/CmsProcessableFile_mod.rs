#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableFile")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsProcessableFile {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _file: quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
    pub _bufSize: i32,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableFile")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::CmsProcessableFile {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "CmsProcessableFile";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableFile")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsProcessableFile {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableFile")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsProcessableFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableFile")]
impl crate::Org::BouncyCastle::Cms::CmsProcessableFile {
    pub const DefaultBufSize: i32 = 32768i32;
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
    pub fn New_FileInfo0(
        file: quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (file))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        file: quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
        bufSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (file, bufSize))?;
        Ok(__cordl_object.into())
    }
    pub fn Write(
        &mut self,
        zOut: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (zOut))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_FileInfo0(
        &mut self,
        file: quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (file))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        file: quest_hook::libil2cpp::Gc<crate::System::IO::FileInfo>,
        bufSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (file, bufSize))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableFile")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsProcessableFile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableFile")]
impl AsRef<crate::Org::BouncyCastle::Cms::CmsProcessable>
for crate::Org::BouncyCastle::Cms::CmsProcessableFile {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Cms::CmsProcessable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableFile")]
impl AsMut<crate::Org::BouncyCastle::Cms::CmsProcessable>
for crate::Org::BouncyCastle::Cms::CmsProcessableFile {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Cms::CmsProcessable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableFile")]
impl AsRef<crate::Org::BouncyCastle::Cms::CmsReadable>
for crate::Org::BouncyCastle::Cms::CmsProcessableFile {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Cms::CmsReadable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsProcessableFile")]
impl AsMut<crate::Org::BouncyCastle::Cms::CmsReadable>
for crate::Org::BouncyCastle::Cms::CmsProcessableFile {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Cms::CmsReadable {
        unsafe { std::mem::transmute(self) }
    }
}
