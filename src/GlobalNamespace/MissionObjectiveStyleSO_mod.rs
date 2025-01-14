#[cfg(feature = "MissionObjectiveStyleSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MissionObjectiveStyleSO {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub titleTextStyle: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TextStyleSO>,
    pub titleColorStyle: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorStyleSO>,
    pub backgroundColorStyle: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorStyleSO,
    >,
}
#[cfg(feature = "MissionObjectiveStyleSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MissionObjectiveStyleSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MissionObjectiveStyleSO";
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
#[cfg(feature = "MissionObjectiveStyleSO")]
impl std::ops::Deref for crate::GlobalNamespace::MissionObjectiveStyleSO {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveStyleSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MissionObjectiveStyleSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MissionObjectiveStyleSO")]
impl crate::GlobalNamespace::MissionObjectiveStyleSO {
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MissionObjectiveStyleSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MissionObjectiveStyleSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
