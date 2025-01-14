#[cfg(feature = "UnityEngine+AvatarMask")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarMask {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+AvatarMask")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::AvatarMask {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AvatarMask";
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
#[cfg(feature = "UnityEngine+AvatarMask")]
impl std::ops::Deref for crate::UnityEngine::AvatarMask {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AvatarMask")]
impl std::ops::DerefMut for crate::UnityEngine::AvatarMask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AvatarMask")]
impl crate::UnityEngine::AvatarMask {
    pub fn GetHumanoidBodyPartActive(
        &mut self,
        index: crate::UnityEngine::AvatarMaskBodyPart,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::AvatarMaskBodyPart),
                bool,
                1usize,
            >("GetHumanoidBodyPartActive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetHumanoidBodyPartActive", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetTransformActive(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), bool, 1usize>("GetTransformActive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTransformActive", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetTransformPath(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("GetTransformPath")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTransformPath", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetTransformWeight(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), f32, 1usize>("GetTransformWeight")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTransformWeight", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (index)) };
        Ok(__cordl_ret.into())
    }
    pub fn get_transformCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_transformCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_transformCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AvatarMask")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AvatarMask {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
