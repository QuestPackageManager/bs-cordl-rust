#[cfg(feature = "GameState")]
#[repr(C)]
#[derive(Debug)]
pub struct GameState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub fsm: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayServerFiniteStateMachine,
    >,
}
#[cfg(feature = "GameState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameState => ""."GameState"
);
#[cfg(feature = "GameState")]
impl std::ops::Deref for crate::GlobalNamespace::GameState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameState")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameState")]
impl crate::GlobalNamespace::GameState {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        fsm: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayServerFiniteStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fsm))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        fsm: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameplayServerFiniteStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fsm))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameState")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GameState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameState")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::GameState {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameState")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::GameState {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
