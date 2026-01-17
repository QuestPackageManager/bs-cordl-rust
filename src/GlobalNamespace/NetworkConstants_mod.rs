#[cfg(feature = "cordl_class_NetworkConstants")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct NetworkConstants {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_NetworkConstants")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::NetworkConstants {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NetworkConstants";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "NetworkConstants")]
impl std::ops::Deref for crate::GlobalNamespace::NetworkConstants {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkConstants")]
impl std::ops::DerefMut for crate::GlobalNamespace::NetworkConstants {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkConstants")]
impl crate::GlobalNamespace::NetworkConstants {
    pub const backgroundedState: &'static str = "backgrounded";
    pub const kProtocolVersion: u32 = 9u32;
    pub const playerState: &'static str = "player";
    pub const spectatingState: &'static str = "spectating";
    pub const terminatingState: &'static str = "terminating";
}
#[cfg(feature = "cordl_class_NetworkConstants")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::NetworkConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
