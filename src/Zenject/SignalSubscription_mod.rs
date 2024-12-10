#[cfg(feature = "Zenject+SignalSubscription")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalSubscription {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _pool: *mut crate::Zenject::SignalSubscription_Pool,
    pub _callback: *mut crate::System::Action_1<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _declaration: *mut crate::Zenject::SignalDeclaration,
    pub _signalId: crate::Zenject::BindingId,
}
#[cfg(feature = "Zenject+SignalSubscription")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalSubscription => "Zenject"
    ."SignalSubscription"
);
#[cfg(feature = "Zenject+SignalSubscription")]
impl std::ops::Deref for crate::Zenject::SignalSubscription {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalSubscription")]
impl std::ops::DerefMut for crate::Zenject::SignalSubscription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalSubscription")]
impl crate::Zenject::SignalSubscription {
    #[cfg(feature = "Zenject+SignalSubscription+Pool")]
    pub type Pool = crate::Zenject::SignalSubscription_Pool;
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
    pub fn Invoke(
        &mut self,
        signal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (signal))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pool: quest_hook::libil2cpp::Gc<crate::Zenject::SignalSubscription_Pool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pool))?;
        Ok(__cordl_object.into())
    }
    pub fn OnDeclarationDespawned(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeclarationDespawned", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDespawned(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDespawned", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSpawned(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        declaration: quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclaration>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSpawned", (callback, declaration))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDefaults(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDefaults", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pool: quest_hook::libil2cpp::Gc<crate::Zenject::SignalSubscription_Pool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pool))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignalId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::BindingId> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::BindingId = __cordl_object
            .invoke("get_SignalId", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SignalSubscription")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SignalSubscription {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+SignalSubscription")]
impl AsRef<crate::System::IDisposable> for crate::Zenject::SignalSubscription {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SignalSubscription")]
impl AsMut<crate::System::IDisposable> for crate::Zenject::SignalSubscription {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SignalSubscription")]
impl AsRef<
    crate::Zenject::IPoolable_2<
        *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        *mut crate::Zenject::SignalDeclaration,
    >,
> for crate::Zenject::SignalSubscription {
    fn as_ref(
        &self,
    ) -> &crate::Zenject::IPoolable_2<
        *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        *mut crate::Zenject::SignalDeclaration,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SignalSubscription")]
impl AsMut<
    crate::Zenject::IPoolable_2<
        *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        *mut crate::Zenject::SignalDeclaration,
    >,
> for crate::Zenject::SignalSubscription {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Zenject::IPoolable_2<
        *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        *mut crate::Zenject::SignalDeclaration,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SignalSubscription+Pool")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalSubscription_Pool {
    __cordl_parent: crate::Zenject::PoolableMemoryPool_3<
        *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        *mut crate::Zenject::SignalDeclaration,
        *mut crate::Zenject::SignalSubscription,
    >,
}
#[cfg(feature = "Zenject+SignalSubscription+Pool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalSubscription_Pool => "Zenject"
    ."SignalSubscription/Pool"
);
#[cfg(feature = "Zenject+SignalSubscription+Pool")]
impl std::ops::Deref for crate::Zenject::SignalSubscription_Pool {
    type Target = crate::Zenject::PoolableMemoryPool_3<
        *mut crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppObject>,
        *mut crate::Zenject::SignalDeclaration,
        *mut crate::Zenject::SignalSubscription,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalSubscription+Pool")]
impl std::ops::DerefMut for crate::Zenject::SignalSubscription_Pool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalSubscription+Pool")]
impl crate::Zenject::SignalSubscription_Pool {
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
}
#[cfg(feature = "Zenject+SignalSubscription+Pool")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SignalSubscription_Pool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
