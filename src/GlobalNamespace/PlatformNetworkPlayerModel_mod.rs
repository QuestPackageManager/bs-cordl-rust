#[cfg(feature = "PlatformNetworkPlayerModel")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformNetworkPlayerModel {
    __cordl_parent: crate::GlobalNamespace::BaseNetworkPlayerModel,
}
#[cfg(feature = "PlatformNetworkPlayerModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformNetworkPlayerModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformNetworkPlayerModel";
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
#[cfg(feature = "PlatformNetworkPlayerModel")]
impl std::ops::Deref for crate::GlobalNamespace::PlatformNetworkPlayerModel {
    type Target = crate::GlobalNamespace::BaseNetworkPlayerModel;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::PlatformNetworkPlayerModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel")]
impl crate::GlobalNamespace::PlatformNetworkPlayerModel {
    #[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
    pub type CreatePartyConfig = crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig;
    pub fn DestroyPartyConnection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("DestroyPartyConnection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DestroyPartyConnection", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
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
    pub fn get_friends(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
                    >,
                >,
                0usize,
            >("get_friends")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_friends", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformNetworkPlayerModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
#[repr(C)]
#[derive(Debug)]
pub struct PlatformNetworkPlayerModel_CreatePartyConfig {
    __cordl_parent: crate::GlobalNamespace::BaseNetworkPlayerModel_PartyConfig,
}
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlatformNetworkPlayerModel/CreatePartyConfig";
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
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
impl std::ops::Deref
for crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
    type Target = crate::GlobalNamespace::BaseNetworkPlayerModel_PartyConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
impl crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
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
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
impl AsRef<
    crate::GlobalNamespace::INetworkPlayerModelPartyConfig_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlatformNetworkPlayerModel>,
    >,
> for crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::INetworkPlayerModelPartyConfig_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlatformNetworkPlayerModel>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PlatformNetworkPlayerModel+CreatePartyConfig")]
impl AsMut<
    crate::GlobalNamespace::INetworkPlayerModelPartyConfig_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlatformNetworkPlayerModel>,
    >,
> for crate::GlobalNamespace::PlatformNetworkPlayerModel_CreatePartyConfig {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::INetworkPlayerModelPartyConfig_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlatformNetworkPlayerModel>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
