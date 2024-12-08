#[cfg(feature = "UnityEngine+Yoga+MeasureFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct MeasureFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+Yoga+MeasureFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Yoga::MeasureFunction =>
    "UnityEngine.Yoga"."MeasureFunction"
);
#[cfg(feature = "UnityEngine+Yoga+MeasureFunction")]
impl std::ops::Deref for crate::UnityEngine::Yoga::MeasureFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+MeasureFunction")]
impl std::ops::DerefMut for crate::UnityEngine::Yoga::MeasureFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Yoga+MeasureFunction")]
impl crate::UnityEngine::Yoga::MeasureFunction {
    pub fn Invoke(
        &mut self,
        node: *mut crate::UnityEngine::Yoga::YogaNode,
        width: f32,
        widthMode: crate::UnityEngine::Yoga::YogaMeasureMode,
        height: f32,
        heightMode: crate::UnityEngine::Yoga::YogaMeasureMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Yoga::YogaSize> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Yoga::YogaSize = __cordl_object
            .invoke("Invoke", (node, width, widthMode, height, heightMode))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Yoga+MeasureFunction")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Yoga::MeasureFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
