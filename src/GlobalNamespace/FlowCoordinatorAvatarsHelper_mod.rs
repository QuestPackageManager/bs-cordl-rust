#[cfg(feature = "FlowCoordinatorAvatarsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct FlowCoordinatorAvatarsHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "FlowCoordinatorAvatarsHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::FlowCoordinatorAvatarsHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "FlowCoordinatorAvatarsHelper";
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
#[cfg(feature = "FlowCoordinatorAvatarsHelper")]
impl std::ops::Deref for crate::GlobalNamespace::FlowCoordinatorAvatarsHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FlowCoordinatorAvatarsHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::FlowCoordinatorAvatarsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FlowCoordinatorAvatarsHelper")]
impl crate::GlobalNamespace::FlowCoordinatorAvatarsHelper {
    pub fn HasUserSelectedAvatarSystemWithCreatedAvatar(
        avatarSystemCollection: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::AvatarCore::AvatarSystemCollection,
        >,
        playerDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerDataModel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::BeatSaber::AvatarCore::AvatarSystemCollection,
                    >,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerDataModel>,
                ),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
                2usize,
            >("HasUserSelectedAvatarSystemWithCreatedAvatar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HasUserSelectedAvatarSystemWithCreatedAvatar", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = unsafe {
            method.invoke_unchecked((), (avatarSystemCollection, playerDataModel))
        };
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "FlowCoordinatorAvatarsHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::FlowCoordinatorAvatarsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
