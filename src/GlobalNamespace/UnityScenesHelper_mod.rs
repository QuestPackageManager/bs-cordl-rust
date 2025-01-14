#[cfg(feature = "UnityScenesHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityScenesHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityScenesHelper")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::UnityScenesHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "UnityScenesHelper";
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
#[cfg(feature = "UnityScenesHelper")]
impl std::ops::Deref for crate::GlobalNamespace::UnityScenesHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityScenesHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::UnityScenesHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityScenesHelper")]
impl crate::GlobalNamespace::UnityScenesHelper {
    pub fn GetComponentsInScene<T>(
        scene: crate::UnityEngine::SceneManagement::Scene,
        components: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::UnityEngine::SceneManagement::Scene,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<T>,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("GetComponentsInScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetComponentsInScene", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (scene, components, includeInactive))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetActiveRootObjectsInScene(
        scene: crate::UnityEngine::SceneManagement::Scene,
        active: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::SceneManagement::Scene, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetActiveRootObjectsInScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetActiveRootObjectsInScene", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (scene, active))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityScenesHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::UnityScenesHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
