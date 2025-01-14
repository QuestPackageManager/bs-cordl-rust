#[cfg(feature = "LightTranslationGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct LightTranslationGroup {
    __cordl_parent: crate::GlobalNamespace::LightTransformGroup_3<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightGroupTranslationXTransform,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightGroupTranslationYTransform,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightGroupTranslationZTransform,
        >,
    >,
    pub _xTranslationLimits: crate::UnityEngine::Vector2,
    pub _yTranslationLimits: crate::UnityEngine::Vector2,
    pub _zTranslationLimits: crate::UnityEngine::Vector2,
    pub _xDistributionLimits: crate::UnityEngine::Vector2,
    pub _yDistributionLimits: crate::UnityEngine::Vector2,
    pub _zDistributionLimits: crate::UnityEngine::Vector2,
}
#[cfg(feature = "LightTranslationGroup")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::LightTranslationGroup {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "LightTranslationGroup";
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
#[cfg(feature = "LightTranslationGroup")]
impl std::ops::Deref for crate::GlobalNamespace::LightTranslationGroup {
    type Target = crate::GlobalNamespace::LightTransformGroup_3<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightGroupTranslationXTransform,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightGroupTranslationYTransform,
        >,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LightGroupTranslationZTransform,
        >,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationGroup")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightTranslationGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightTranslationGroup")]
impl crate::GlobalNamespace::LightTranslationGroup {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn __Validate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("__Validate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "__Validate", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xDistributionLimits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Vector2,
                0usize,
            >("get_xDistributionLimits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_xDistributionLimits", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_xTranslationLimits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Vector2,
                0usize,
            >("get_xTranslationLimits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_xTranslationLimits", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yDistributionLimits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Vector2,
                0usize,
            >("get_yDistributionLimits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_yDistributionLimits", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_yTranslationLimits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Vector2,
                0usize,
            >("get_yTranslationLimits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_yTranslationLimits", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_zDistributionLimits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Vector2,
                0usize,
            >("get_zDistributionLimits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_zDistributionLimits", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_zTranslationLimits(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Vector2,
                0usize,
            >("get_zTranslationLimits")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_zTranslationLimits", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector2 = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightTranslationGroup")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightTranslationGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightTranslationGroup")]
impl AsRef<crate::GlobalNamespace::IEditTimeValidated>
for crate::GlobalNamespace::LightTranslationGroup {
    fn as_ref(&self) -> &crate::GlobalNamespace::IEditTimeValidated {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LightTranslationGroup")]
impl AsMut<crate::GlobalNamespace::IEditTimeValidated>
for crate::GlobalNamespace::LightTranslationGroup {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IEditTimeValidated {
        unsafe { std::mem::transmute(self) }
    }
}
