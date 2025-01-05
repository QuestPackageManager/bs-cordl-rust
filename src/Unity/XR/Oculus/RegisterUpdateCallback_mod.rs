#[cfg(feature = "Unity+XR+Oculus+RegisterUpdateCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterUpdateCallback {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+RegisterUpdateCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::RegisterUpdateCallback =>
    "Unity.XR.Oculus"."RegisterUpdateCallback"
);
#[cfg(feature = "Unity+XR+Oculus+RegisterUpdateCallback")]
impl std::ops::Deref for crate::Unity::XR::Oculus::RegisterUpdateCallback {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+RegisterUpdateCallback")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::RegisterUpdateCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+RegisterUpdateCallback")]
impl crate::Unity::XR::Oculus::RegisterUpdateCallback {
    pub fn Deinitialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Deinitialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+XR+Oculus+RegisterUpdateCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::XR::Oculus::RegisterUpdateCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
