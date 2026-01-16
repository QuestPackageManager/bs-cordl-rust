#[cfg(feature = "cordl_class_OVR+OpenVR+CVRResources")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRResources {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRResources,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRResources")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVRResources {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRResources";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "OVR+OpenVR+CVRResources")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRResources {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRResources")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRResources {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRResources")]
impl crate::OVR::OpenVR::CVRResources {
    pub fn GetResourceFullPath(
        &mut self,
        pchResourceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchResourceTypeDirectory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchPathBuffer: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                        u32,
                    ), u32, 4usize>("GetResourceFullPath")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetResourceFullPath",
                            4usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    pchResourceName,
                    pchResourceTypeDirectory,
                    pchPathBuffer,
                    unBufferLen,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadSharedResource(
        &mut self,
        pchResourceName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        unBufferLen: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        u32,
                    ), u32, 3usize>("LoadSharedResource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LoadSharedResource",
                            3usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (pchResourceName, pchBuffer, unBufferLen))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::IntPtr), quest_hook::libil2cpp::Void, 1usize>(
                        ".ctor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pInterface))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRResources")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRResources {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
