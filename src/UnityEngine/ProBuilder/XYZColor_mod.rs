#[cfg(feature = "UnityEngine+ProBuilder+XYZColor")]
#[repr(C)]
#[derive(Debug)]
pub struct XYZColor {
    __cordl_parent: crate::System::Object,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[cfg(feature = "UnityEngine+ProBuilder+XYZColor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::XYZColor =>
    "UnityEngine.ProBuilder"."XYZColor"
);
#[cfg(feature = "UnityEngine+ProBuilder+XYZColor")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::XYZColor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+XYZColor")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::XYZColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+XYZColor")]
impl crate::UnityEngine::ProBuilder::XYZColor {
    pub fn New(x: f32, y: f32, z: f32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (x, y, z))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (x, y, z))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+XYZColor")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::XYZColor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}