#[cfg(feature = "AsyncComputeOperation")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncComputeOperation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _timeoutMs: i32,
    pub _stopwatch: *mut crate::System::Diagnostics::Stopwatch,
}
#[cfg(feature = "AsyncComputeOperation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AsyncComputeOperation => ""
    ."AsyncComputeOperation"
);
#[cfg(feature = "AsyncComputeOperation")]
impl std::ops::Deref for crate::GlobalNamespace::AsyncComputeOperation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AsyncComputeOperation")]
impl std::ops::DerefMut for crate::GlobalNamespace::AsyncComputeOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AsyncComputeOperation")]
impl crate::GlobalNamespace::AsyncComputeOperation {
    pub fn Execute(
        &mut self,
        disposed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Execute", (disposed))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        timeoutMs: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (timeoutMs))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        timeoutMs: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (timeoutMs))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_elapsedTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_elapsedTime", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasTimedOut(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasTimedOut", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AsyncComputeOperation")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AsyncComputeOperation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
