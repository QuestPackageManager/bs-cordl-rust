#[cfg(feature = "MomentaryLoudnessHistory")]
#[repr(C)]
#[derive(Debug)]
pub struct MomentaryLoudnessHistory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _rmsValues: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub _nextRmsIndex: i32,
    pub _historyFull: bool,
}
#[cfg(feature = "MomentaryLoudnessHistory")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MomentaryLoudnessHistory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MomentaryLoudnessHistory";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "MomentaryLoudnessHistory")]
impl std::ops::Deref for crate::GlobalNamespace::MomentaryLoudnessHistory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MomentaryLoudnessHistory")]
impl std::ops::DerefMut for crate::GlobalNamespace::MomentaryLoudnessHistory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MomentaryLoudnessHistory")]
impl crate::GlobalNamespace::MomentaryLoudnessHistory {
    pub fn Add(
        &mut self,
        momentaryLoudness: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (momentaryLoudness))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAverageLoudness(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetAverageLoudness", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (length))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (length))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MomentaryLoudnessHistory")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MomentaryLoudnessHistory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
