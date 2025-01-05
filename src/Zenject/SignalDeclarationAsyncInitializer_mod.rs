#[cfg(feature = "Zenject+SignalDeclarationAsyncInitializer")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalDeclarationAsyncInitializer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _tickManager: quest_hook::libil2cpp::Gc<
        crate::Zenject::LazyInject_1<
            quest_hook::libil2cpp::Gc<crate::Zenject::TickableManager>,
        >,
    >,
    pub _declarations: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclaration>,
        >,
    >,
}
#[cfg(feature = "Zenject+SignalDeclarationAsyncInitializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalDeclarationAsyncInitializer =>
    "Zenject"."SignalDeclarationAsyncInitializer"
);
#[cfg(feature = "Zenject+SignalDeclarationAsyncInitializer")]
impl std::ops::Deref for crate::Zenject::SignalDeclarationAsyncInitializer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New(
        declarations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclaration>,
            >,
        >,
        tickManager: quest_hook::libil2cpp::Gc<
            crate::Zenject::LazyInject_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::TickableManager>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (declarations, tickManager))?;
        Ok(__cordl_object.into())
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
    pub fn _ctor(
        &mut self,
        declarations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclaration>,
            >,
        >,
        tickManager: quest_hook::libil2cpp::Gc<
            crate::Zenject::LazyInject_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::TickableManager>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (declarations, tickManager))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Zenject+SignalDeclarationAsyncInitializer")]
impl AsRef<crate::Zenject::IInitializable>
for crate::Zenject::SignalDeclarationAsyncInitializer {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SignalDeclarationAsyncInitializer")]
impl AsMut<crate::Zenject::IInitializable>
for crate::Zenject::SignalDeclarationAsyncInitializer {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
