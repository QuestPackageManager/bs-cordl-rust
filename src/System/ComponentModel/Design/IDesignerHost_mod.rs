#[cfg(feature = "System+ComponentModel+Design+IDesignerHost")]
#[repr(C)]
#[derive(Debug)]
pub struct IDesignerHost {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+ComponentModel+Design+IDesignerHost")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::Design::IDesignerHost =>
    "System.ComponentModel.Design"."IDesignerHost"
);
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
    pub fn get_RootComponent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::ComponentModel::IComponent> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::IComponent = __cordl_object
            .invoke("get_RootComponent", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDesigner(
        &mut self,
        component: *mut crate::System::ComponentModel::IComponent,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::Design::IDesigner,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::Design::IDesigner = __cordl_object
            .invoke("GetDesigner", (component))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
