#[cfg(feature = "OVR+OpenVR+CVRSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRSettings,
}
#[cfg(feature = "OVR+OpenVR+CVRSettings")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVRSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRSettings";
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
#[cfg(feature = "OVR+OpenVR+CVRSettings")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSettings")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSettings")]
impl crate::OVR::OpenVR::CVRSettings {
    pub fn GetBool(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
                ),
                bool,
                3usize,
            >("GetBool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBool", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (pchSection, pchSettingsKey, peError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFloat(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
                ),
                f32,
                3usize,
            >("GetFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFloat", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked(self, (pchSection, pchSettingsKey, peError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInt32(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
                ),
                i32,
                3usize,
            >("GetInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInt32", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (pchSection, pchSettingsKey, peError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSettingsErrorNameFromEnum(
        &mut self,
        eError: crate::OVR::OpenVR::EVRSettingsError,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::OVR::OpenVR::EVRSettingsError),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetSettingsErrorNameFromEnum")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSettingsErrorNameFromEnum", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (eError)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetString(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchValue: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        unValueLen: u32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >("GetString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetString", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (pchSection, pchSettingsKey, pchValue, unValueLen, peError),
                )
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
    pub fn RemoveKeyInSection(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("RemoveKeyInSection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemoveKeyInSection", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pchSection, pchSettingsKey, peError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveSection(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RemoveSection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemoveSection", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pchSection, peError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBool(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bValue: bool,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetBool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetBool", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pchSection, pchSettingsKey, bValue, peError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetFloat(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flValue: f32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    f32,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetFloat", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pchSection, pchSettingsKey, flValue, peError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetInt32(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        nValue: i32,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetInt32", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pchSection, pchSettingsKey, nValue, peError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetString(
        &mut self,
        pchSection: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchSettingsKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        pchValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SetString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetString", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (pchSection, pchSettingsKey, pchValue, peError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Sync(
        &mut self,
        bForce: bool,
        peError: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::EVRSettingsError>,
                ),
                bool,
                2usize,
            >("Sync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Sync", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (bForce, peError))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (pInterface))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+CVRSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
