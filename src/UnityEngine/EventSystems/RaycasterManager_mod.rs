#[cfg(feature = "UnityEngine+EventSystems+RaycasterManager")]
#[repr(C)]
#[derive(Debug)]
pub struct RaycasterManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+EventSystems+RaycasterManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::EventSystems::RaycasterManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.EventSystems";
    const CLASS_NAME: &'static str = "RaycasterManager";
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
#[cfg(feature = "UnityEngine+EventSystems+RaycasterManager")]
impl std::ops::Deref for crate::UnityEngine::EventSystems::RaycasterManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+RaycasterManager")]
impl std::ops::DerefMut for crate::UnityEngine::EventSystems::RaycasterManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+EventSystems+RaycasterManager")]
impl crate::UnityEngine::EventSystems::RaycasterManager {
    pub fn AddRaycaster(
        baseRaycaster: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseRaycaster,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddRaycaster", (baseRaycaster))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRaycasters() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::BaseRaycaster,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::EventSystems::BaseRaycaster,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetRaycasters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveRaycasters(
        baseRaycaster: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::BaseRaycaster,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveRaycasters", (baseRaycaster))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+EventSystems+RaycasterManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::EventSystems::RaycasterManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
