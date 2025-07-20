#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlNodeChangedEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub action: crate::System::Xml::XmlNodeChangedAction,
    pub node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    pub oldParent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    pub newParent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
    pub oldValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub newValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::XmlNodeChangedEventArgs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "XmlNodeChangedEventArgs";
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
#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
impl std::ops::Deref for crate::System::Xml::XmlNodeChangedEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
impl std::ops::DerefMut for crate::System::Xml::XmlNodeChangedEventArgs {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
impl crate::System::Xml::XmlNodeChangedEventArgs {
    pub fn New(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        oldParent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        newParent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        oldValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        action: crate::System::Xml::XmlNodeChangedAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (node, oldParent, newParent, oldValue, newValue, action),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        oldParent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        newParent: quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
        oldValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        newValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        action: crate::System::Xml::XmlNodeChangedAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
                            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
                            quest_hook::libil2cpp::Gc<crate::System::Xml::XmlNode>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Xml::XmlNodeChangedAction,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (node, oldParent, newParent, oldValue, newValue, action),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Action(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Xml::XmlNodeChangedAction> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::System::Xml::XmlNodeChangedAction,
                        0usize,
                    >("get_Action")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_Action", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Xml::XmlNodeChangedAction = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+XmlNodeChangedEventArgs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlNodeChangedEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
