#[cfg(feature = "System+Net+FileWebStream")]
#[repr(C)]
#[derive(Debug)]
pub struct FileWebStream {
    __cordl_parent: crate::System::IO::FileStream,
    pub m_request: *mut crate::System::Net::FileWebRequest,
}
#[cfg(feature = "System+Net+FileWebStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FileWebStream => "System.Net"
    ."FileWebStream"
);
#[cfg(feature = "System+Net+FileWebStream")]
impl std::ops::Deref for crate::System::Net::FileWebStream {
    type Target = crate::System::IO::FileStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FileWebStream")]
impl std::ops::DerefMut for crate::System::Net::FileWebStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FileWebStream")]
impl crate::System::Net::FileWebStream {
    pub fn BeginRead(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
        callback: *mut crate::System::AsyncCallback,
        state: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginRead", (buffer, offset, _cordl_size, callback, state))?;
        Ok(__cordl_ret)
    }
    pub fn BeginWrite(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
        callback: *mut crate::System::AsyncCallback,
        state: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginWrite", (buffer, offset, _cordl_size, callback, state))?;
        Ok(__cordl_ret)
    }
    pub fn CheckError(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckError", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn EndRead(
        &mut self,
        ar: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EndRead", (ar))?;
        Ok(__cordl_ret)
    }
    pub fn EndWrite(
        &mut self,
        ar: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndWrite", (ar))?;
        Ok(__cordl_ret)
    }
    pub fn New_FileWebRequest_Il2CppString_FileMode_FileAccess_FileShare0(
        request: *mut crate::System::Net::FileWebRequest,
        path: *mut quest_hook::libil2cpp::Il2CppString,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        sharing: crate::System::IO::FileShare,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (request, path, mode, access, sharing))?;
        Ok(__cordl_object)
    }
    pub fn New_i32__cordl_bool1(
        request: *mut crate::System::Net::FileWebRequest,
        path: *mut quest_hook::libil2cpp::Il2CppString,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        sharing: crate::System::IO::FileShare,
        length: i32,
        _cordl_async: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (request, path, mode, access, sharing, length, _cordl_async),
            )?;
        Ok(__cordl_object)
    }
    pub fn Read(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Read", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn System_Net_ICloseEx_CloseEx(
        &mut self,
        closeState: crate::System::Net::CloseExState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Net.ICloseEx.CloseEx", (closeState))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_FileWebRequest_Il2CppString_FileMode_FileAccess_FileShare0(
        &mut self,
        request: *mut crate::System::Net::FileWebRequest,
        path: *mut quest_hook::libil2cpp::Il2CppString,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        sharing: crate::System::IO::FileShare,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (request, path, mode, access, sharing))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32__cordl_bool1(
        &mut self,
        request: *mut crate::System::Net::FileWebRequest,
        path: *mut quest_hook::libil2cpp::Il2CppString,
        mode: crate::System::IO::FileMode,
        access: crate::System::IO::FileAccess,
        sharing: crate::System::IO::FileShare,
        length: i32,
        _cordl_async: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (request, path, mode, access, sharing, length, _cordl_async),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+FileWebStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::FileWebStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
