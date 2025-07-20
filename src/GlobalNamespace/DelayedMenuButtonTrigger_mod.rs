#[cfg(feature = "DelayedMenuButtonTrigger")]
#[repr(C)]
#[derive(Debug)]
pub struct DelayedMenuButtonTrigger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub menuButtonTriggeredEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _pressDuration: f32,
    pub _timer: f32,
    pub _waitingForButtonRelease: bool,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
}
#[cfg(feature = "DelayedMenuButtonTrigger")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DelayedMenuButtonTrigger {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DelayedMenuButtonTrigger";
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
#[cfg(feature = "DelayedMenuButtonTrigger")]
impl std::ops::Deref for crate::GlobalNamespace::DelayedMenuButtonTrigger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DelayedMenuButtonTrigger")]
impl std::ops::DerefMut for crate::GlobalNamespace::DelayedMenuButtonTrigger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DelayedMenuButtonTrigger")]
impl crate::GlobalNamespace::DelayedMenuButtonTrigger {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::DelayedMenuButtonTrigger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Tick")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::DelayedMenuButtonTrigger as
                    quest_hook::libil2cpp::Type > ::class(), "Tick", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::DelayedMenuButtonTrigger as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::DelayedMenuButtonTrigger as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_menuButtonTriggeredEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::DelayedMenuButtonTrigger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_menuButtonTriggeredEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::DelayedMenuButtonTrigger as
                    quest_hook::libil2cpp::Type > ::class(),
                    "add_menuButtonTriggeredEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_menuButtonTriggeredEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::DelayedMenuButtonTrigger as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_menuButtonTriggeredEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::DelayedMenuButtonTrigger as
                    quest_hook::libil2cpp::Type > ::class(),
                    "remove_menuButtonTriggeredEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DelayedMenuButtonTrigger")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DelayedMenuButtonTrigger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DelayedMenuButtonTrigger")]
impl AsRef<crate::GlobalNamespace::IMenuButtonTrigger>
for crate::GlobalNamespace::DelayedMenuButtonTrigger {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMenuButtonTrigger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DelayedMenuButtonTrigger")]
impl AsMut<crate::GlobalNamespace::IMenuButtonTrigger>
for crate::GlobalNamespace::DelayedMenuButtonTrigger {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMenuButtonTrigger {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DelayedMenuButtonTrigger")]
impl AsRef<crate::Zenject::ITickable>
for crate::GlobalNamespace::DelayedMenuButtonTrigger {
    fn as_ref(&self) -> &crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "DelayedMenuButtonTrigger")]
impl AsMut<crate::Zenject::ITickable>
for crate::GlobalNamespace::DelayedMenuButtonTrigger {
    fn as_mut(&mut self) -> &mut crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
