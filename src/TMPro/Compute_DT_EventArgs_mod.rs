#[cfg(feature = "TMPro+Compute_DT_EventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct Compute_DT_EventArgs {
    __cordl_parent: crate::System::Object,
    pub EventType: crate::TMPro::Compute_DistanceTransform_EventTypes,
    pub ProgressPercentage: f32,
    pub Colors: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
}
#[cfg(feature = "TMPro+Compute_DT_EventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::TMPro::Compute_DT_EventArgs => "TMPro"
    ."Compute_DT_EventArgs"
);
#[cfg(feature = "TMPro+Compute_DT_EventArgs")]
impl std::ops::Deref for crate::TMPro::Compute_DT_EventArgs {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+Compute_DT_EventArgs")]
impl std::ops::DerefMut for crate::TMPro::Compute_DT_EventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TMPro+Compute_DT_EventArgs")]
impl crate::TMPro::Compute_DT_EventArgs {
    pub fn _ctor_f32_0(
        &mut self,
        _cordl_type: crate::TMPro::Compute_DistanceTransform_EventTypes,
        progress: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, progress))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        _cordl_type: crate::TMPro::Compute_DistanceTransform_EventTypes,
        colors: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, colors))?;
        Ok(__cordl_ret)
    }
    pub fn New_f32_0(
        _cordl_type: crate::TMPro::Compute_DistanceTransform_EventTypes,
        progress: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, progress))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray1(
        _cordl_type: crate::TMPro::Compute_DistanceTransform_EventTypes,
        colors: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, colors))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "TMPro+Compute_DT_EventArgs")]
impl quest_hook::libil2cpp::ObjectType for crate::TMPro::Compute_DT_EventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
