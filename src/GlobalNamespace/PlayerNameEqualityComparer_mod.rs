#[cfg(feature = "PlayerNameEqualityComparer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerNameEqualityComparer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "PlayerNameEqualityComparer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerNameEqualityComparer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerNameEqualityComparer";
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
#[cfg(feature = "PlayerNameEqualityComparer")]
impl std::ops::Deref for crate::GlobalNamespace::PlayerNameEqualityComparer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerNameEqualityComparer")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlayerNameEqualityComparer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerNameEqualityComparer")]
impl crate::GlobalNamespace::PlayerNameEqualityComparer {
    pub fn Equals(
        &mut self,
        x: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
        y: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
                    >,
                ),
                bool,
                2usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (x, y)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
                >),
                i32,
                1usize,
            >("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetHashCode", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (obj)) };
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
#[cfg(feature = "PlayerNameEqualityComparer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerNameEqualityComparer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlayerNameEqualityComparer")]
impl AsRef<
    crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
    >,
> for crate::GlobalNamespace::PlayerNameEqualityComparer {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlayerNameEqualityComparer")]
impl AsMut<
    crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
    >,
> for crate::GlobalNamespace::PlayerNameEqualityComparer {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEqualityComparer_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlatformLeaderboardsModel_LeaderboardScore,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
