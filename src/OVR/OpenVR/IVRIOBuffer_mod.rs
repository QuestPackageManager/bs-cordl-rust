#[cfg(feature = "OVR+OpenVR+IVRIOBuffer")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IVRIOBuffer {
    pub Open: *mut crate::OVR::OpenVR::IVRIOBuffer__Open,
    pub Close: *mut crate::OVR::OpenVR::IVRIOBuffer__Close,
    pub Read: *mut crate::OVR::OpenVR::IVRIOBuffer__Read,
    pub Write: *mut crate::OVR::OpenVR::IVRIOBuffer__Write,
    pub PropertyContainer: *mut crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer,
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRIOBuffer => "OVR.OpenVR"
    ."IVRIOBuffer"
);
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::IVRIOBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer")]
impl crate::OVR::OpenVR::IVRIOBuffer {
    #[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
    pub type _Open = crate::OVR::OpenVR::IVRIOBuffer__Open;
    #[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
    pub type _Read = crate::OVR::OpenVR::IVRIOBuffer__Read;
    #[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
    pub type _Close = crate::OVR::OpenVR::IVRIOBuffer__Close;
    #[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
    pub type _Write = crate::OVR::OpenVR::IVRIOBuffer__Write;
    #[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
    pub type _PropertyContainer = crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer;
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRIOBuffer__Close {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRIOBuffer__Close => "OVR.OpenVR"
    ."IVRIOBuffer/_Close"
);
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRIOBuffer__Close {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRIOBuffer__Close {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
impl crate::OVR::OpenVR::IVRIOBuffer__Close {
    pub fn BeginInvoke(
        &mut self,
        ulBuffer: u64,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (ulBuffer, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        ulBuffer: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = __cordl_object
            .invoke("Invoke", (ulBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Close")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRIOBuffer__Close {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRIOBuffer__Open {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRIOBuffer__Open => "OVR.OpenVR"
    ."IVRIOBuffer/_Open"
);
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRIOBuffer__Open {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRIOBuffer__Open {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
impl crate::OVR::OpenVR::IVRIOBuffer__Open {
    pub fn BeginInvoke(
        &mut self,
        pchPath: *mut crate::System::String,
        mode: crate::OVR::OpenVR::EIOBufferMode,
        unElementSize: u32,
        unElements: u32,
        pulBuffer: quest_hook::libil2cpp::ByRefMut<u64>,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (pchPath, mode, unElementSize, unElements, pulBuffer, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        pulBuffer: quest_hook::libil2cpp::ByRefMut<u64>,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = __cordl_object
            .invoke("EndInvoke", (pulBuffer, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        pchPath: *mut crate::System::String,
        mode: crate::OVR::OpenVR::EIOBufferMode,
        unElementSize: u32,
        unElements: u32,
        pulBuffer: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = __cordl_object
            .invoke("Invoke", (pchPath, mode, unElementSize, unElements, pulBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Open")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRIOBuffer__Open {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRIOBuffer__PropertyContainer {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRIOBuffer__PropertyContainer =>
    "OVR.OpenVR"."IVRIOBuffer/_PropertyContainer"
);
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
impl crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer {
    pub fn BeginInvoke(
        &mut self,
        ulBuffer: u64,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (ulBuffer, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(&mut self, ulBuffer: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("Invoke", (ulBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_PropertyContainer")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRIOBuffer__PropertyContainer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRIOBuffer__Read {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRIOBuffer__Read => "OVR.OpenVR"
    ."IVRIOBuffer/_Read"
);
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRIOBuffer__Read {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRIOBuffer__Read {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
impl crate::OVR::OpenVR::IVRIOBuffer__Read {
    pub fn BeginInvoke(
        &mut self,
        ulBuffer: u64,
        pDst: crate::System::IntPtr,
        unBytes: u32,
        punRead: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke(
                "BeginInvoke",
                (ulBuffer, pDst, unBytes, punRead, callback, object),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        punRead: quest_hook::libil2cpp::ByRefMut<u32>,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = __cordl_object
            .invoke("EndInvoke", (punRead, result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        ulBuffer: u64,
        pDst: crate::System::IntPtr,
        unBytes: u32,
        punRead: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = __cordl_object
            .invoke("Invoke", (ulBuffer, pDst, unBytes, punRead))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Read")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRIOBuffer__Read {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRIOBuffer__Write {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::IVRIOBuffer__Write => "OVR.OpenVR"
    ."IVRIOBuffer/_Write"
);
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRIOBuffer__Write {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRIOBuffer__Write {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
impl crate::OVR::OpenVR::IVRIOBuffer__Write {
    pub fn BeginInvoke(
        &mut self,
        ulBuffer: u64,
        pSrc: crate::System::IntPtr,
        unBytes: u32,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (ulBuffer, pSrc, unBytes, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        ulBuffer: u64,
        pSrc: crate::System::IntPtr,
        unBytes: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = __cordl_object
            .invoke("Invoke", (ulBuffer, pSrc, unBytes))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRIOBuffer+_Write")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::IVRIOBuffer__Write {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
