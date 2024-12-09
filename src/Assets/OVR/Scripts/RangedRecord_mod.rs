#[cfg(feature = "Assets+OVR+Scripts+RangedRecord")]
#[repr(C)]
#[derive(Debug)]
pub struct RangedRecord {
    __cordl_parent: crate::Assets::OVR::Scripts::Record,
    pub value: f32,
    pub min: f32,
    pub max: f32,
}
#[cfg(feature = "Assets+OVR+Scripts+RangedRecord")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Assets::OVR::Scripts::RangedRecord =>
    "Assets.OVR.Scripts"."RangedRecord"
);
#[cfg(feature = "Assets+OVR+Scripts+RangedRecord")]
impl std::ops::Deref for crate::Assets::OVR::Scripts::RangedRecord {
    type Target = crate::Assets::OVR::Scripts::Record;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Assets+OVR+Scripts+RangedRecord")]
impl std::ops::DerefMut for crate::Assets::OVR::Scripts::RangedRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Assets+OVR+Scripts+RangedRecord")]
impl crate::Assets::OVR::Scripts::RangedRecord {
    pub fn New(
        order: i32,
        cat: *mut quest_hook::libil2cpp::Il2CppString,
        msg: *mut quest_hook::libil2cpp::Il2CppString,
        val: f32,
        minVal: f32,
        maxVal: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (order, cat, msg, val, minVal, maxVal))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        order: i32,
        cat: *mut quest_hook::libil2cpp::Il2CppString,
        msg: *mut quest_hook::libil2cpp::Il2CppString,
        val: f32,
        minVal: f32,
        maxVal: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (order, cat, msg, val, minVal, maxVal))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Assets+OVR+Scripts+RangedRecord")]
impl quest_hook::libil2cpp::ObjectType for crate::Assets::OVR::Scripts::RangedRecord {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
