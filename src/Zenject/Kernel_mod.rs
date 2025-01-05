#[cfg(feature = "Zenject+Kernel")]
#[repr(C)]
#[derive(Debug)]
pub struct Kernel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _tickableManager: quest_hook::libil2cpp::Gc<crate::Zenject::TickableManager>,
    pub _initializableManager: quest_hook::libil2cpp::Gc<
        crate::Zenject::InitializableManager,
    >,
    pub _disposablesManager: quest_hook::libil2cpp::Gc<
        crate::Zenject::DisposableManager,
    >,
}
#[cfg(feature = "Zenject+Kernel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::Kernel => "Zenject"."Kernel"
);
#[cfg(feature = "Zenject+Kernel")]
impl std::ops::Deref for crate::Zenject::Kernel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl std::ops::DerefMut for crate::Zenject::Kernel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl crate::Zenject::Kernel {
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
    pub fn FixedTick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FixedTick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LateDispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateDispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LateTick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateTick", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn __zenCreate(
        P_0: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("__zenCreate", (P_0))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenCreateInjectTypeInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenFieldSetter0(
        P_0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        P_1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenFieldSetter0", (P_0, P_1))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenFieldSetter1(
        P_0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        P_1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenFieldSetter1", (P_0, P_1))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenFieldSetter2(
        P_0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        P_1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenFieldSetter2", (P_0, P_1))?;
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
#[cfg(feature = "Zenject+Kernel")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::Kernel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl AsRef<crate::System::IDisposable> for crate::Zenject::Kernel {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl AsMut<crate::System::IDisposable> for crate::Zenject::Kernel {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl AsRef<crate::Zenject::IFixedTickable> for crate::Zenject::Kernel {
    fn as_ref(&self) -> &crate::Zenject::IFixedTickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl AsMut<crate::Zenject::IFixedTickable> for crate::Zenject::Kernel {
    fn as_mut(&mut self) -> &mut crate::Zenject::IFixedTickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl AsRef<crate::Zenject::IInitializable> for crate::Zenject::Kernel {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl AsMut<crate::Zenject::IInitializable> for crate::Zenject::Kernel {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl AsRef<crate::Zenject::ILateDisposable> for crate::Zenject::Kernel {
    fn as_ref(&self) -> &crate::Zenject::ILateDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl AsMut<crate::Zenject::ILateDisposable> for crate::Zenject::Kernel {
    fn as_mut(&mut self) -> &mut crate::Zenject::ILateDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl AsRef<crate::Zenject::ILateTickable> for crate::Zenject::Kernel {
    fn as_ref(&self) -> &crate::Zenject::ILateTickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl AsMut<crate::Zenject::ILateTickable> for crate::Zenject::Kernel {
    fn as_mut(&mut self) -> &mut crate::Zenject::ILateTickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl AsRef<crate::Zenject::ITickable> for crate::Zenject::Kernel {
    fn as_ref(&self) -> &crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+Kernel")]
impl AsMut<crate::Zenject::ITickable> for crate::Zenject::Kernel {
    fn as_mut(&mut self) -> &mut crate::Zenject::ITickable {
        unsafe { std::mem::transmute(self) }
    }
}
