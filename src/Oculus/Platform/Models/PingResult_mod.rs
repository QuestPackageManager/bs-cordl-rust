#[cfg(feature = "Oculus+Platform+Models+PingResult")]
#[repr(C)]
#[derive(Debug)]
pub struct PingResult {
    __cordl_parent: crate::System::Object,
    pub _ID_k__BackingField: u64,
    pub pingTimeUsec: crate::System::Nullable_1<u64>,
}
#[cfg(feature = "Oculus+Platform+Models+PingResult")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Models::PingResult =>
    "Oculus.Platform.Models"."PingResult"
);
#[cfg(feature = "Oculus+Platform+Models+PingResult")]
impl std::ops::Deref for crate::Oculus::Platform::Models::PingResult {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+PingResult")]
impl std::ops::DerefMut for crate::Oculus::Platform::Models::PingResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+PingResult")]
impl crate::Oculus::Platform::Models::PingResult {
    pub fn New(
        id: u64,
        pingTimeUsec: crate::System::Nullable_1<u64>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id, pingTimeUsec))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        id: u64,
        pingTimeUsec: crate::System::Nullable_1<u64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (id, pingTimeUsec))?;
        Ok(__cordl_ret)
    }
    pub fn get_ID(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("get_ID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsTimeout(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsTimeout", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PingTimeUsec(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("get_PingTimeUsec", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ID(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ID", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Oculus+Platform+Models+PingResult")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Models::PingResult {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
