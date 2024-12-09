#[cfg(feature = "Zenject+SignalDeclaration")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalDeclaration {
    __cordl_parent: crate::System::Object,
    pub _subscriptions: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::SignalSubscription,
    >,
    pub _asyncQueue: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Object,
    >,
    pub _bindingId: crate::Zenject::BindingId,
    pub _missingHandlerResponses: crate::Zenject::SignalMissingHandlerResponses,
    pub _isAsync: bool,
    pub _settings: *mut crate::Zenject::ZenjectSettings_SignalSettings,
    pub _TickPriority_k__BackingField: i32,
}
#[cfg(feature = "Zenject+SignalDeclaration")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalDeclaration => "Zenject"
    ."SignalDeclaration"
);
#[cfg(feature = "Zenject+SignalDeclaration")]
impl std::ops::Deref for crate::Zenject::SignalDeclaration {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalDeclaration")]
impl std::ops::DerefMut for crate::Zenject::SignalDeclaration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalDeclaration")]
impl crate::Zenject::SignalDeclaration {
    #[cfg(feature = "Zenject+SignalDeclaration+Factory")]
    pub type Factory = crate::Zenject::SignalDeclaration_Factory;
    pub fn Add(
        &mut self,
        subscription: *mut crate::Zenject::SignalSubscription,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (subscription))?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Fire(
        &mut self,
        signal: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fire", (signal))?;
        Ok(__cordl_ret)
    }
    pub fn FireInternal(
        &mut self,
        subscriptions: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::SignalSubscription,
        >,
        signal: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FireInternal", (subscriptions, signal))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindInfo: *mut crate::Zenject::SignalDeclarationBindInfo,
        zenjectSettings: *mut crate::Zenject::ZenjectSettings,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo, zenjectSettings))?;
        Ok(__cordl_object)
    }
    pub fn Remove(
        &mut self,
        subscription: *mut crate::Zenject::SignalSubscription,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (subscription))?;
        Ok(__cordl_ret)
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bindInfo: *mut crate::Zenject::SignalDeclarationBindInfo,
        zenjectSettings: *mut crate::Zenject::ZenjectSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo, zenjectSettings))?;
        Ok(__cordl_ret)
    }
    pub fn get_BindingId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::BindingId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::BindingId = __cordl_object
            .invoke("get_BindingId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAsync(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TickPriority(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TickPriority", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_TickPriority(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TickPriority", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+SignalDeclaration")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SignalDeclaration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+SignalDeclaration+Factory")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalDeclaration_Factory {
    __cordl_parent: crate::Zenject::PlaceholderFactory_2<
        *mut crate::Zenject::SignalDeclarationBindInfo,
        *mut crate::Zenject::SignalDeclaration,
    >,
}
#[cfg(feature = "Zenject+SignalDeclaration+Factory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalDeclaration_Factory => "Zenject"
    ."SignalDeclaration/Factory"
);
#[cfg(feature = "Zenject+SignalDeclaration+Factory")]
impl std::ops::Deref for crate::Zenject::SignalDeclaration_Factory {
    type Target = crate::Zenject::PlaceholderFactory_2<
        *mut crate::Zenject::SignalDeclarationBindInfo,
        *mut crate::Zenject::SignalDeclaration,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalDeclaration+Factory")]
impl std::ops::DerefMut for crate::Zenject::SignalDeclaration_Factory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalDeclaration+Factory")]
impl crate::Zenject::SignalDeclaration_Factory {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+SignalDeclaration+Factory")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SignalDeclaration_Factory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
