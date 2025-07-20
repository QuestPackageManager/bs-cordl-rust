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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+Design+IDesignerHost")]
impl std::ops::DerefMut for crate::System::ComponentModel::Design::IDesignerHost {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::IComponent,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::Design::IDesigner,
                        >,
                        1usize,
                    >("GetDesigner")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDesigner", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::Design::IDesigner,
        > = unsafe { method.invoke_unchecked(self, (component))? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::ComponentModel::IComponent,
                        >,
                        0usize,
                    >("get_RootComponent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_RootComponent", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::IComponent,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
