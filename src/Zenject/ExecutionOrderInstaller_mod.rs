#[cfg(feature = "Zenject+ExecutionOrderInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct ExecutionOrderInstaller {
    __cordl_parent: crate::Zenject::Installer_2<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::Type>,
        *mut crate::Zenject::ExecutionOrderInstaller,
    >,
    pub _typeOrder: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Type,
    >,
}
#[cfg(feature = "Zenject+ExecutionOrderInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ExecutionOrderInstaller => "Zenject"
    ."ExecutionOrderInstaller"
);
#[cfg(feature = "Zenject+ExecutionOrderInstaller")]
impl std::ops::Deref for crate::Zenject::ExecutionOrderInstaller {
    type Target = crate::Zenject::Installer_2<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::Type>,
        *mut crate::Zenject::ExecutionOrderInstaller,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ExecutionOrderInstaller")]
impl std::ops::DerefMut for crate::Zenject::ExecutionOrderInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ExecutionOrderInstaller")]
impl crate::Zenject::ExecutionOrderInstaller {
    pub fn InstallBindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstallBindings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        typeOrder: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (typeOrder))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        typeOrder: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (typeOrder))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ExecutionOrderInstaller")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ExecutionOrderInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
