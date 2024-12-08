#[cfg(feature = "Zenject+SignalDeclarationAsyncInitializer")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalDeclarationAsyncInitializer {
    __cordl_parent: crate::System::Object,
    pub _tickManager: *mut crate::Zenject::LazyInject_1<
        *mut crate::Zenject::TickableManager,
    >,
    pub _declarations: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::SignalDeclaration,
    >,
}
#[cfg(feature = "Zenject+SignalDeclarationAsyncInitializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalDeclarationAsyncInitializer =>
    "Zenject"."SignalDeclarationAsyncInitializer"
);
#[cfg(feature = "Zenject+SignalDeclarationAsyncInitializer")]
impl std::ops::Deref for crate::Zenject::SignalDeclarationAsyncInitializer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalDeclarationAsyncInitializer")]
impl std::ops::DerefMut for crate::Zenject::SignalDeclarationAsyncInitializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalDeclarationAsyncInitializer")]
impl crate::Zenject::SignalDeclarationAsyncInitializer {
    pub fn _ctor(
        &mut self,
        declarations: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::SignalDeclaration,
        >,
        tickManager: *mut crate::Zenject::LazyInject_1<
            *mut crate::Zenject::TickableManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (declarations, tickManager))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        declarations: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::SignalDeclaration,
        >,
        tickManager: *mut crate::Zenject::LazyInject_1<
            *mut crate::Zenject::TickableManager,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (declarations, tickManager))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+SignalDeclarationAsyncInitializer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SignalDeclarationAsyncInitializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
