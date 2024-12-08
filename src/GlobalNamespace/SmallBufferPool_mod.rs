#[cfg(feature = "SmallBufferPool")]
#[repr(C)]
#[derive(Debug)]
pub struct SmallBufferPool {
    __cordl_parent: crate::System::Object,
    pub _cacheSmall: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub _cacheMedium: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub _cacheLarge: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub _cacheMax: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
}
#[cfg(feature = "SmallBufferPool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SmallBufferPool => ""
    ."SmallBufferPool"
);
#[cfg(feature = "SmallBufferPool")]
impl std::ops::Deref for crate::GlobalNamespace::SmallBufferPool {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SmallBufferPool")]
impl std::ops::DerefMut for crate::GlobalNamespace::SmallBufferPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SmallBufferPool")]
impl crate::GlobalNamespace::SmallBufferPool {
    pub const kCacheLargeMaxCapacity: i32 = 16i32;
    pub const kCacheLargeSize: i32 = 2048i32;
    pub const kCacheMaxMaxCapacity: i32 = 8i32;
    pub const kCacheMaxSize: i32 = 4096i32;
    pub const kCacheMediumMaxCapacity: i32 = 32i32;
    pub const kCacheMediumSize: i32 = 1024i32;
    pub const kCacheSmallMaxCapacity: i32 = 128i32;
    pub const kCacheSmallSize: i32 = 512i32;
    pub fn GetBuffer(
        &mut self,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetBuffer", (length))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ReleaseBufferInternal(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseBufferInternal", (buffer))?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseBuffer_ByRefMut1(
        &mut self,
        buffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseBuffer", (buffer))?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseBuffer_Il2CppArray0(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseBuffer", (buffer))?;
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
}
#[cfg(feature = "SmallBufferPool")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::SmallBufferPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
