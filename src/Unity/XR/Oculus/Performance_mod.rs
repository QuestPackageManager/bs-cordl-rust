#[cfg(feature = "Unity+XR+Oculus+Performance")]
#[repr(C)]
#[derive(Debug)]
pub struct Performance {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Performance")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::XR::Oculus::Performance {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.XR.Oculus";
    const CLASS_NAME: &'static str = "Performance";
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
#[cfg(feature = "Unity+XR+Oculus+Performance")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Performance {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Performance")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Performance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Performance")]
impl crate::Unity::XR::Oculus::Performance {
    pub fn TryGetAvailableDisplayRefreshRates(
        refreshRates: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
                >),
                bool,
                1usize,
            >("TryGetAvailableDisplayRefreshRates")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetAvailableDisplayRefreshRates", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (refreshRates)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetDisplayRefreshRate(
        refreshRate: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::ByRefMut<f32>),
                bool,
                1usize,
            >("TryGetDisplayRefreshRate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryGetDisplayRefreshRate", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (refreshRate)) };
        Ok(__cordl_ret.into())
    }
    pub fn TrySetCPULevel(level: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), bool, 1usize>("TrySetCPULevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TrySetCPULevel", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (level)) };
        Ok(__cordl_ret.into())
    }
    pub fn TrySetDisplayRefreshRate(
        refreshRate: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), bool, 1usize>("TrySetDisplayRefreshRate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TrySetDisplayRefreshRate", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (refreshRate)) };
        Ok(__cordl_ret.into())
    }
    pub fn TrySetGPULevel(level: i32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), bool, 1usize>("TrySetGPULevel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TrySetGPULevel", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (level)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+Performance")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Performance {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
