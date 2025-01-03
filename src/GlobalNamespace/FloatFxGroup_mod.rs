#[cfg(feature = "FloatFxGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct FloatFxGroup {
    __cordl_parent: crate::GlobalNamespace::LightGroupSubsystem,
    pub _isTriggerOnly: bool,
    pub _targets: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::FloatFxGroupEffectTarget,
        >,
    >,
}
#[cfg(feature = "FloatFxGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FloatFxGroup => ""
    ."FloatFxGroup"
);
#[cfg(feature = "FloatFxGroup")]
impl std::ops::Deref for crate::GlobalNamespace::FloatFxGroup {
    type Target = crate::GlobalNamespace::LightGroupSubsystem;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroup")]
impl std::ops::DerefMut for crate::GlobalNamespace::FloatFxGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FloatFxGroup")]
impl crate::GlobalNamespace::FloatFxGroup {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isTriggerOnly(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isTriggerOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_targets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::FloatFxGroupEffectTarget,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::GlobalNamespace::FloatFxGroupEffectTarget,
            >,
        > = __cordl_object.invoke("get_targets", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FloatFxGroup")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FloatFxGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
