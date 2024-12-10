#[cfg(feature = "MonotonicTimeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct MonotonicTimeProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _timeSpanTicksPerStopwatchTick: f64,
    pub _startTicks: i64,
    pub _stopwatch: *mut crate::System::Diagnostics::Stopwatch,
}
#[cfg(feature = "MonotonicTimeProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MonotonicTimeProvider => ""
    ."MonotonicTimeProvider"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetTicks", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTimeMs(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetTimeMs", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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
