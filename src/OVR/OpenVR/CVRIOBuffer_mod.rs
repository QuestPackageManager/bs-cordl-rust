#[cfg(feature = "cordl_class_OVR+OpenVR+CVRIOBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRIOBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRIOBuffer,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRIOBuffer")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVRIOBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRIOBuffer";
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
#[cfg(feature = "OVR+OpenVR+CVRIOBuffer")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRIOBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRIOBuffer")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRIOBuffer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRIOBuffer")]
impl crate::OVR::OpenVR::CVRIOBuffer {
    pub fn Close(
        &mut self,
        ulBuffer: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64),
                        crate::OVR::OpenVR::EIOBufferError,
                        1usize,
                    >("Close")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Close",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = unsafe {
            cordl_method_info.invoke_unchecked(self, (ulBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object.into())
    }
    pub fn Open(
        &mut self,
        pchPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::OVR::OpenVR::EIOBufferMode,
        unElementSize: u32,
        unElements: u32,
        pulBuffer: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::OVR::OpenVR::EIOBufferMode,
                            u32,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<u64>,
                        ),
                        crate::OVR::OpenVR::EIOBufferError,
                        5usize,
                    >("Open")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Open",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (pchPath, mode, unElementSize, unElements, pulBuffer),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PropertyContainer(
        &mut self,
        ulBuffer: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), u64, 1usize>("PropertyContainer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PropertyContainer", 1usize
                        )
                    })
            });
        let __cordl_ret: u64 = unsafe {
            cordl_method_info.invoke_unchecked(self, (ulBuffer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Read(
        &mut self,
        ulBuffer: u64,
        pDst: crate::System::IntPtr,
        unBytes: u32,
        punRead: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::System::IntPtr,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        crate::OVR::OpenVR::EIOBufferError,
                        4usize,
                    >("Read")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Read",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = unsafe {
            cordl_method_info.invoke_unchecked(self, (ulBuffer, pDst, unBytes, punRead))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        ulBuffer: u64,
        pSrc: crate::System::IntPtr,
        unBytes: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EIOBufferError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, crate::System::IntPtr, u32),
                        crate::OVR::OpenVR::EIOBufferError,
                        3usize,
                    >("Write")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Write",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EIOBufferError = unsafe {
            cordl_method_info.invoke_unchecked(self, (ulBuffer, pSrc, unBytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::IntPtr),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (pInterface))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRIOBuffer")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRIOBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
