#[cfg(feature = "Unity+XR+Oculus+Stats")]
#[repr(C)]
#[derive(Debug)]
pub struct Stats {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Stats")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::Stats => "Unity.XR.Oculus"
    ."Stats"
);
#[cfg(feature = "Unity+XR+Oculus+Stats")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Stats {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Stats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats")]
impl crate::Unity::XR::Oculus::Stats {
    #[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
    pub type AdaptivePerformance = crate::Unity::XR::Oculus::Stats_AdaptivePerformance;
    #[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
    pub type AppMetrics = crate::Unity::XR::Oculus::Stats_AppMetrics;
    #[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
    pub type PerfMetrics = crate::Unity::XR::Oculus::Stats_PerfMetrics;
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
#[cfg(feature = "Unity+XR+Oculus+Stats")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Stats {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
#[repr(C)]
#[derive(Debug)]
pub struct Stats_AdaptivePerformance {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::Stats_AdaptivePerformance =>
    "Unity.XR.Oculus"."Stats/AdaptivePerformance"
);
#[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Stats_AdaptivePerformance {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Stats_AdaptivePerformance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
impl crate::Unity::XR::Oculus::Stats_AdaptivePerformance {}
#[cfg(feature = "Unity+XR+Oculus+Stats+AdaptivePerformance")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::XR::Oculus::Stats_AdaptivePerformance {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
#[repr(C)]
#[derive(Debug)]
pub struct Stats_AppMetrics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::Stats_AppMetrics =>
    "Unity.XR.Oculus"."Stats/AppMetrics"
);
#[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Stats_AppMetrics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Stats_AppMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
impl crate::Unity::XR::Oculus::Stats_AppMetrics {}
#[cfg(feature = "Unity+XR+Oculus+Stats+AppMetrics")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Stats_AppMetrics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
#[repr(C)]
#[derive(Debug)]
pub struct Stats_PerfMetrics {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::Stats_PerfMetrics =>
    "Unity.XR.Oculus"."Stats/PerfMetrics"
);
#[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
impl std::ops::Deref for crate::Unity::XR::Oculus::Stats_PerfMetrics {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::Stats_PerfMetrics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
impl crate::Unity::XR::Oculus::Stats_PerfMetrics {}
#[cfg(feature = "Unity+XR+Oculus+Stats+PerfMetrics")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::Stats_PerfMetrics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
