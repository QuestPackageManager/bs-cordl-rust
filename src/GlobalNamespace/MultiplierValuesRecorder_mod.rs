#[cfg(feature = "MultiplierValuesRecorder")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplierValuesRecorder {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _scoreController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IScoreController,
    >,
    pub _audioTimeSyncController: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AudioTimeSyncController,
    >,
    pub _multiplierValues: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::GlobalNamespace::MultiplierValuesRecorder_MultiplierValue,
        >,
    >,
}
#[cfg(feature = "MultiplierValuesRecorder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplierValuesRecorder => ""
    ."MultiplierValuesRecorder"
);
#[cfg(feature = "MultiplierValuesRecorder")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplierValuesRecorder {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplierValuesRecorder")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplierValuesRecorder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplierValuesRecorder")]
impl crate::GlobalNamespace::MultiplierValuesRecorder {
    #[cfg(feature = "MultiplierValuesRecorder+MultiplierValue")]
    pub type MultiplierValue = crate::GlobalNamespace::MultiplierValuesRecorder_MultiplierValue;
    pub fn HandleScoreControllerMultiplierDidChange(
        &mut self,
        multiplier: i32,
        multiplierProgress: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleScoreControllerMultiplierDidChange",
                (multiplier, multiplierProgress),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_multiplierValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::MultiplierValuesRecorder_MultiplierValue,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::GlobalNamespace::MultiplierValuesRecorder_MultiplierValue,
            >,
        > = __cordl_object.invoke("get_multiplierValues", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplierValuesRecorder")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplierValuesRecorder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplierValuesRecorder+MultiplierValue")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MultiplierValuesRecorder_MultiplierValue {
    pub multiplier: i32,
    pub _cordl_time: f32,
}
#[cfg(feature = "MultiplierValuesRecorder+MultiplierValue")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplierValuesRecorder_MultiplierValue => ""
    ."MultiplierValuesRecorder/MultiplierValue"
);
#[cfg(feature = "MultiplierValuesRecorder+MultiplierValue")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::MultiplierValuesRecorder_MultiplierValue {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "MultiplierValuesRecorder+MultiplierValue")]
impl crate::GlobalNamespace::MultiplierValuesRecorder_MultiplierValue {
    pub fn _ctor(
        &mut self,
        multiplier: i32,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (multiplier, _cordl_time),
        )?;
        Ok(__cordl_ret.into())
    }
}
