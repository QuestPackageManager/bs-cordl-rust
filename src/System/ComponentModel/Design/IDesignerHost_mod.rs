#[cfg(feature = "System+ComponentModel+Design+IDesignerHost")]
#[repr(C)]
#[derive(Debug)]
pub struct IDesignerHost {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+Design+IDesignerHost")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::ComponentModel::Design::IDesignerHost {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.ComponentModel.Design";
    const CLASS_NAME: &'static str = "IDesignerHost";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+ComponentModel+Design+IDesignerHost")]
impl std::ops::Deref for crate::System::ComponentModel::Design::IDesignerHost {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IDesignerHost")]
impl std::ops::DerefMut for crate::System::ComponentModel::Design::IDesignerHost {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IDesignerHost")]
impl crate::System::ComponentModel::Design::IDesignerHost {
    pub fn GetDesigner(
        &mut self,
        component: quest_hook::libil2cpp::Gc<crate::System::ComponentModel::IComponent>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::Design::IDesigner>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::Design::IDesigner,
        > = __cordl_object.invoke("GetDesigner", (component))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_RootComponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::ComponentModel::IComponent>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IComponent,
        > = __cordl_object.invoke("get_RootComponent", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+Design+IDesignerHost")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::Design::IDesignerHost {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+ComponentModel+Design+IDesignerHost")]
impl AsRef<crate::System::IServiceProvider>
for crate::System::ComponentModel::Design::IDesignerHost {
    fn as_ref(&self) -> &crate::System::IServiceProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IDesignerHost")]
impl AsMut<crate::System::IServiceProvider>
for crate::System::ComponentModel::Design::IDesignerHost {
    fn as_mut(&mut self) -> &mut crate::System::IServiceProvider {
        unsafe { std::mem::transmute(self) }
    }
}
