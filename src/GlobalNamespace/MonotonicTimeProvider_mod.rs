#[cfg(feature = "MonotonicTimeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct MonotonicTimeProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _timeSpanTicksPerStopwatchTick: f64,
    pub _startTicks: i64,
    pub _stopwatch: quest_hook::libil2cpp::Gc<crate::System::Diagnostics::Stopwatch>,
}
#[cfg(feature = "MonotonicTimeProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MonotonicTimeProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MonotonicTimeProvider";
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
#[cfg(feature = "MonotonicTimeProvider")]
impl std::ops::Deref for crate::GlobalNamespace::MonotonicTimeProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MonotonicTimeProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::MonotonicTimeProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MonotonicTimeProvider")]
impl crate::GlobalNamespace::MonotonicTimeProvider {
    pub fn GetTicks(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i64, 0usize>("GetTicks")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTicks", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeMs(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i64, 0usize>("GetTimeMs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTimeMs", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MonotonicTimeProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MonotonicTimeProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MonotonicTimeProvider")]
impl AsRef<crate::BGNet::Core::ITimeProvider>
for crate::GlobalNamespace::MonotonicTimeProvider {
    fn as_ref(&self) -> &crate::BGNet::Core::ITimeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MonotonicTimeProvider")]
impl AsMut<crate::BGNet::Core::ITimeProvider>
for crate::GlobalNamespace::MonotonicTimeProvider {
    fn as_mut(&mut self) -> &mut crate::BGNet::Core::ITimeProvider {
        unsafe { std::mem::transmute(self) }
    }
}
