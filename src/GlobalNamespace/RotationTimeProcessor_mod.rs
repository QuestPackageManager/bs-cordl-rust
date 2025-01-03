#[cfg(feature = "RotationTimeProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct RotationTimeProcessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _rotationChangeDataList: *mut crate::System::Collections::Generic::List_1<
        crate::GlobalNamespace::RotationTimeProcessor_RotationChangeData,
    >,
    pub currentBpmChangesDataIdx: i32,
}
#[cfg(feature = "RotationTimeProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RotationTimeProcessor => ""
    ."RotationTimeProcessor"
);
#[cfg(feature = "RotationTimeProcessor")]
impl std::ops::Deref for crate::GlobalNamespace::RotationTimeProcessor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RotationTimeProcessor")]
impl std::ops::DerefMut for crate::GlobalNamespace::RotationTimeProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RotationTimeProcessor")]
impl crate::GlobalNamespace::RotationTimeProcessor {
    #[cfg(feature = "RotationTimeProcessor+RotationChangeData")]
    pub type RotationChangeData = crate::GlobalNamespace::RotationTimeProcessor_RotationChangeData;
    pub fn ConvertBeatToRotation(
        &mut self,
        beat: f32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ConvertBeatToRotation", (beat))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_IReadOnlyList_1_0(
        events: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (events))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IReadOnlyList_1_1(
        rotationEventsSaveData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::BeatmapSaveDataVersion3::RotationEventData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rotationEventsSaveData))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SpawnRotationForEventValue(index: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SpawnRotationForEventValue", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IReadOnlyList_1_0(
        &mut self,
        events: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::BeatmapSaveDataVersion2_6_0AndEarlier::EventData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (events))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IReadOnlyList_1_1(
        &mut self,
        rotationEventsSaveData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::BeatmapSaveDataVersion3::RotationEventData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rotationEventsSaveData))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "RotationTimeProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RotationTimeProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "RotationTimeProcessor+RotationChangeData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RotationTimeProcessor_RotationChangeData {
    pub beat: f32,
    pub rotation: i32,
}
#[cfg(feature = "RotationTimeProcessor+RotationChangeData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::RotationTimeProcessor_RotationChangeData => ""
    ."RotationTimeProcessor/RotationChangeData"
);
#[cfg(feature = "RotationTimeProcessor+RotationChangeData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::RotationTimeProcessor_RotationChangeData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "RotationTimeProcessor+RotationChangeData")]
impl crate::GlobalNamespace::RotationTimeProcessor_RotationChangeData {
    pub fn _ctor(
        &mut self,
        beat: f32,
        rotation: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (beat, rotation),
        )?;
        Ok(__cordl_ret.into())
    }
}
