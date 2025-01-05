#[cfg(feature = "Zenject+NoTransitionContext")]
#[repr(C)]
#[derive(Debug)]
pub struct NoTransitionContext {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _noScenesTransitionInstaller: quest_hook::libil2cpp::Gc<
        crate::Zenject::NoTransitionInstaller,
    >,
}
#[cfg(feature = "Zenject+NoTransitionContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::NoTransitionContext => "Zenject"
    ."NoTransitionContext"
);
#[cfg(feature = "Zenject+NoTransitionContext")]
impl std::ops::Deref for crate::Zenject::NoTransitionContext {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+NoTransitionContext")]
impl std::ops::DerefMut for crate::Zenject::NoTransitionContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+NoTransitionContext")]
impl crate::Zenject::NoTransitionContext {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenCreateInjectTypeInfo", ())?;
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
    pub fn get_installMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        > = __cordl_object.invoke("get_installMethod", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_postInstallMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            >,
        > = __cordl_object.invoke("get_postInstallMethod", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+NoTransitionContext")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::NoTransitionContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
