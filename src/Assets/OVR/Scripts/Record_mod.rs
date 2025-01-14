#[cfg(feature = "Assets+OVR+Scripts+Record")]
#[repr(C)]
#[derive(Debug)]
pub struct Record {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub sortOrder: i32,
    pub category: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "Assets+OVR+Scripts+Record")]
unsafe impl quest_hook::libil2cpp::Type for crate::Assets::OVR::Scripts::Record {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Assets.OVR.Scripts";
    const CLASS_NAME: &'static str = "Record";
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
#[cfg(feature = "Assets+OVR+Scripts+Record")]
impl std::ops::Deref for crate::Assets::OVR::Scripts::Record {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Assets+OVR+Scripts+Record")]
impl std::ops::DerefMut for crate::Assets::OVR::Scripts::Record {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Assets+OVR+Scripts+Record")]
impl crate::Assets::OVR::Scripts::Record {
    pub fn New(
        order: i32,
        cat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (order, cat, msg))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        order: i32,
        cat: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (order, cat, msg))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Assets+OVR+Scripts+Record")]
impl quest_hook::libil2cpp::ObjectType for crate::Assets::OVR::Scripts::Record {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
