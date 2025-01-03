#[cfg(feature = "Zenject+GuiRenderableManager")]
#[repr(C)]
#[derive(Debug)]
pub struct GuiRenderableManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _renderables: *mut crate::System::Collections::Generic::List_1<
        *mut crate::Zenject::GuiRenderableManager_RenderableInfo,
    >,
}
#[cfg(feature = "Zenject+GuiRenderableManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::GuiRenderableManager => "Zenject"
    ."GuiRenderableManager"
);
#[cfg(feature = "Zenject+GuiRenderableManager")]
impl std::ops::Deref for crate::Zenject::GuiRenderableManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+GuiRenderableManager")]
impl std::ops::DerefMut for crate::Zenject::GuiRenderableManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+GuiRenderableManager")]
impl crate::Zenject::GuiRenderableManager {
    #[cfg(feature = "Zenject+GuiRenderableManager+RenderableInfo")]
    pub type RenderableInfo = crate::Zenject::GuiRenderableManager_RenderableInfo;
    pub fn New(
        renderables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Zenject::IGuiRenderable,
            >,
        >,
        priorities: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (renderables, priorities))?;
        Ok(__cordl_object.into())
    }
    pub fn OnGui(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnGui", ())?;
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
    pub fn _ctor(
        &mut self,
        renderables: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Zenject::IGuiRenderable,
            >,
        >,
        priorities: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::ModestTree::Util::ValuePair_2<*mut crate::System::Type, i32>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (renderables, priorities))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+GuiRenderableManager")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::GuiRenderableManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+GuiRenderableManager+RenderableInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct GuiRenderableManager_RenderableInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Renderable: *mut crate::Zenject::IGuiRenderable,
    pub Priority: i32,
}
#[cfg(feature = "Zenject+GuiRenderableManager+RenderableInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::GuiRenderableManager_RenderableInfo =>
    "Zenject"."GuiRenderableManager/RenderableInfo"
);
#[cfg(feature = "Zenject+GuiRenderableManager+RenderableInfo")]
impl std::ops::Deref for crate::Zenject::GuiRenderableManager_RenderableInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+GuiRenderableManager+RenderableInfo")]
impl std::ops::DerefMut for crate::Zenject::GuiRenderableManager_RenderableInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+GuiRenderableManager+RenderableInfo")]
impl crate::Zenject::GuiRenderableManager_RenderableInfo {
    pub fn New(
        renderable: quest_hook::libil2cpp::Gc<crate::Zenject::IGuiRenderable>,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (renderable, priority))?;
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
        renderable: quest_hook::libil2cpp::Gc<crate::Zenject::IGuiRenderable>,
        priority: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (renderable, priority))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+GuiRenderableManager+RenderableInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::GuiRenderableManager_RenderableInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
