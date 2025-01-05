#[cfg(feature = "SynchronizedActionQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct SynchronizedActionQueue {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _synchronizedActionQueue: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SynchronizedActionQueue_SynchronizedAction,
    >,
}
#[cfg(feature = "SynchronizedActionQueue")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SynchronizedActionQueue => ""
    ."SynchronizedActionQueue"
);
#[cfg(feature = "SynchronizedActionQueue")]
impl std::ops::Deref for crate::GlobalNamespace::SynchronizedActionQueue {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SynchronizedActionQueue")]
impl std::ops::DerefMut for crate::GlobalNamespace::SynchronizedActionQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SynchronizedActionQueue")]
impl crate::GlobalNamespace::SynchronizedActionQueue {
    #[cfg(feature = "SynchronizedActionQueue+SynchronizedAction")]
    pub type SynchronizedAction = crate::GlobalNamespace::SynchronizedActionQueue_SynchronizedAction;
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EnqueueAction(
        &mut self,
        _cordl_time: f32,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnqueueAction", (_cordl_time, action))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Update(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SynchronizedActionQueue")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SynchronizedActionQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SynchronizedActionQueue+SynchronizedAction")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SynchronizedActionQueue_SynchronizedAction {
    pub _cordl_time: f32,
    pub action: quest_hook::libil2cpp::Gc<crate::System::Action>,
}
#[cfg(feature = "SynchronizedActionQueue+SynchronizedAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SynchronizedActionQueue_SynchronizedAction => ""
    ."SynchronizedActionQueue/SynchronizedAction"
);
#[cfg(feature = "SynchronizedActionQueue+SynchronizedAction")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::SynchronizedActionQueue_SynchronizedAction {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "SynchronizedActionQueue+SynchronizedAction")]
impl crate::GlobalNamespace::SynchronizedActionQueue_SynchronizedAction {
    pub fn _ctor(
        &mut self,
        _cordl_time: f32,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time, action),
        )?;
        Ok(__cordl_ret.into())
    }
}
