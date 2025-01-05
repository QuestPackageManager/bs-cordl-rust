#[cfg(feature = "Zenject+SignalTickPriorityCopyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalTickPriorityCopyBinder {
    __cordl_parent: crate::Zenject::SignalCopyBinder,
    pub _SignalBindInfo_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Zenject::SignalDeclarationBindInfo,
    >,
}
#[cfg(feature = "Zenject+SignalTickPriorityCopyBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalTickPriorityCopyBinder =>
    "Zenject"."SignalTickPriorityCopyBinder"
);
#[cfg(feature = "Zenject+SignalTickPriorityCopyBinder")]
impl std::ops::Deref for crate::Zenject::SignalTickPriorityCopyBinder {
    type Target = crate::Zenject::SignalCopyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalTickPriorityCopyBinder")]
impl std::ops::DerefMut for crate::Zenject::SignalTickPriorityCopyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalTickPriorityCopyBinder")]
impl crate::Zenject::SignalTickPriorityCopyBinder {
    pub fn New(
        signalBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::SignalDeclarationBindInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signalBindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn WithTickPriority(
        &mut self,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SignalCopyBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::SignalCopyBinder> = __cordl_object
            .invoke("WithTickPriority", (priority))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        signalBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::SignalDeclarationBindInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signalBindInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignalBindInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclarationBindInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::SignalDeclarationBindInfo,
        > = __cordl_object.invoke("get_SignalBindInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SignalBindInfo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclarationBindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SignalBindInfo", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SignalTickPriorityCopyBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SignalTickPriorityCopyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
