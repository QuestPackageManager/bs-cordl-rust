#[cfg(feature = "OVR+OpenVR+CVRIOBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRIOBuffer {
    __cordl_parent: crate::System::Object,
    pub FnTable: crate::OVR::OpenVR::IVRIOBuffer,
}
#[cfg(feature = "OVR+OpenVR+CVRIOBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRIOBuffer => "OVR.OpenVR"
    ."CVRIOBuffer"
);
#[cfg(feature = "OVR+OpenVR+CVRIOBuffer")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRIOBuffer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRIOBuffer")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRIOBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRIOBuffer")]
impl crate::OVR::OpenVR::CVRIOBuffer {
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pInterface))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        ulBuffer: u64,
        pSrc: crate::System::IntPtr,
        unBytes: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = __cordl_object
            .invoke("Write", (ulBuffer, pSrc, unBytes))?;
        Ok(__cordl_ret)
    }
    pub fn Close(
        &mut self,
        ulBuffer: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = __cordl_object
            .invoke("Close", (ulBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn Open(
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
            .invoke("Open", (pchPath, mode, unElementSize, unElements, pulBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn Read(
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
            .invoke("Read", (ulBuffer, pDst, unBytes, punRead))?;
        Ok(__cordl_ret)
    }
    pub fn PropertyContainer(
        &mut self,
        ulBuffer: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("PropertyContainer", (ulBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVR+OpenVR+CVRIOBuffer")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRIOBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
