#[cfg(feature = "IServerBeatmapProviderManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IServerBeatmapProviderManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IServerBeatmapProviderManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IServerBeatmapProviderManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IServerBeatmapProviderManager";
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
#[cfg(feature = "IServerBeatmapProviderManager")]
impl std::ops::Deref for crate::GlobalNamespace::IServerBeatmapProviderManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IServerBeatmapProviderManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::IServerBeatmapProviderManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IServerBeatmapProviderManager")]
impl crate::GlobalNamespace::IServerBeatmapProviderManager {
    pub fn GetServerBeatmapProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IServerBeatmapProvider>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IServerBeatmapProviderManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IServerBeatmapProvider,
                >,
                0usize,
            >("GetServerBeatmapProvider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IServerBeatmapProviderManager as
                    quest_hook::libil2cpp::Type > ::class(), "GetServerBeatmapProvider",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IServerBeatmapProvider,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn RefreshAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IServerBeatmapProviderManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                0usize,
            >("RefreshAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IServerBeatmapProviderManager as
                    quest_hook::libil2cpp::Type > ::class(), "RefreshAsync", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IServerBeatmapProviderManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IServerBeatmapProviderManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IServerBeatmapProviderManager")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::IServerBeatmapProviderManager {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IServerBeatmapProviderManager")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::IServerBeatmapProviderManager {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
