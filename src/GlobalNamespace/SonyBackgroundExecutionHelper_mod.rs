#[cfg(feature = "SonyBackgroundExecutionHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyBackgroundExecutionHelper {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub didGoToBackgroundExecutionEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub didGoToForegroundExecutionEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub _isInBackgroundExecution: bool,
}
#[cfg(feature = "SonyBackgroundExecutionHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SonyBackgroundExecutionHelper
    => ""."SonyBackgroundExecutionHelper"
);
#[cfg(feature = "SonyBackgroundExecutionHelper")]
impl std::ops::Deref for crate::GlobalNamespace::SonyBackgroundExecutionHelper {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyBackgroundExecutionHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::SonyBackgroundExecutionHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyBackgroundExecutionHelper")]
impl crate::GlobalNamespace::SonyBackgroundExecutionHelper {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
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
    pub fn add_didGoToBackgroundExecutionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didGoToBackgroundExecutionEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didGoToForegroundExecutionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didGoToForegroundExecutionEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isInBackgroundExecution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_isInBackgroundExecution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didGoToBackgroundExecutionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didGoToBackgroundExecutionEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didGoToForegroundExecutionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didGoToForegroundExecutionEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SonyBackgroundExecutionHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyBackgroundExecutionHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyBackgroundExecutionHelper")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Zenject::ITickable>>
for crate::GlobalNamespace::SonyBackgroundExecutionHelper {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::Zenject::ITickable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "SonyBackgroundExecutionHelper")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Zenject::ITickable>>
for crate::GlobalNamespace::SonyBackgroundExecutionHelper {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::Zenject::ITickable> {
        unsafe { std::mem::transmute(self) }
    }
}
