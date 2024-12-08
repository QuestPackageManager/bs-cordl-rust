#[cfg(feature = "CombineGroupIdToVector4FloatFxGroupEffectTarget")]
#[repr(C)]
#[derive(Debug)]
pub struct CombineGroupIdToVector4FloatFxGroupEffectTarget {
    __cordl_parent: FloatFxGroupEffectTarget,
    pub _propertyName: *mut crate::System::String,
    pub _materialPropertyBlockController: *mut MaterialPropertyBlockController,
    pub _defaultValue: crate::UnityEngine::Vector4,
    pub _lightGroupsToIndices: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::CombineGroupIdToVector4FloatFxGroupEffectTarget_LightGroupSOToIndex,
    >,
    pub _beatmapCallbacksController: *mut BeatmapCallbacksController,
    pub _didReceiveEventThisFrame: bool,
    pub _groupIdToIndex: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        i32,
    >,
    pub _propertyId: i32,
    pub _data: crate::UnityEngine::Vector4,
}
#[cfg(feature = "CombineGroupIdToVector4FloatFxGroupEffectTarget")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for CombineGroupIdToVector4FloatFxGroupEffectTarget => ""
    ."CombineGroupIdToVector4FloatFxGroupEffectTarget"
);
#[cfg(feature = "CombineGroupIdToVector4FloatFxGroupEffectTarget")]
impl std::ops::Deref for CombineGroupIdToVector4FloatFxGroupEffectTarget {
    type Target = FloatFxGroupEffectTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "CombineGroupIdToVector4FloatFxGroupEffectTarget")]
impl std::ops::DerefMut for CombineGroupIdToVector4FloatFxGroupEffectTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "CombineGroupIdToVector4FloatFxGroupEffectTarget")]
impl CombineGroupIdToVector4FloatFxGroupEffectTarget {
    #[cfg(
        feature = "CombineGroupIdToVector4FloatFxGroupEffectTarget+LightGroupSOToIndex"
    )]
    pub type LightGroupSOToIndex = crate::GlobalNamespace::CombineGroupIdToVector4FloatFxGroupEffectTarget_LightGroupSOToIndex;
    #[cfg(feature = "CombineGroupIdToVector4FloatFxGroupEffectTarget+__c")]
    pub type __c = crate::GlobalNamespace::CombineGroupIdToVector4FloatFxGroupEffectTarget___c;
    pub fn SetValue(
        &mut self,
        groupId: i32,
        elementId: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (groupId, elementId, value))?;
        Ok(__cordl_ret)
    }
    pub fn TriggerValue(
        &mut self,
        groupId: i32,
        elementId: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TriggerValue", (groupId, elementId, value))?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleBeatmapCallbacksControllerDidProcessAllCallbacksThisFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleBeatmapCallbacksControllerDidProcessAllCallbacksThisFrame",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "CombineGroupIdToVector4FloatFxGroupEffectTarget")]
impl quest_hook::libil2cpp::ObjectType
for CombineGroupIdToVector4FloatFxGroupEffectTarget {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "CombineGroupIdToVector4FloatFxGroupEffectTarget+LightGroupSOToIndex")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CombineGroupIdToVector4FloatFxGroupEffectTarget_LightGroupSOToIndex {
    pub lightGroup: *mut LightGroupSO,
    pub index: i32,
}
#[cfg(feature = "CombineGroupIdToVector4FloatFxGroupEffectTarget+LightGroupSOToIndex")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::CombineGroupIdToVector4FloatFxGroupEffectTarget_LightGroupSOToIndex
    => ""."CombineGroupIdToVector4FloatFxGroupEffectTarget/LightGroupSOToIndex"
);
#[cfg(feature = "CombineGroupIdToVector4FloatFxGroupEffectTarget+LightGroupSOToIndex")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::CombineGroupIdToVector4FloatFxGroupEffectTarget_LightGroupSOToIndex {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "CombineGroupIdToVector4FloatFxGroupEffectTarget+LightGroupSOToIndex")]
impl crate::GlobalNamespace::CombineGroupIdToVector4FloatFxGroupEffectTarget_LightGroupSOToIndex {}
