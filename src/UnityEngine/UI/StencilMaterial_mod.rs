#[cfg(feature = "UnityEngine+UI+StencilMaterial")]
#[repr(C)]
#[derive(Debug)]
pub struct StencilMaterial {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UI::StencilMaterial {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "StencilMaterial";
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
#[cfg(feature = "UnityEngine+UI+StencilMaterial")]
impl std::ops::Deref for crate::UnityEngine::UI::StencilMaterial {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial")]
impl std::ops::DerefMut for crate::UnityEngine::UI::StencilMaterial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial")]
impl crate::UnityEngine::UI::StencilMaterial {
    #[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
    pub type MatEntry = crate::UnityEngine::UI::StencilMaterial_MatEntry;
    pub fn Add_Material_i32_0(
        baseMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        stencilID: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>, i32),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                2usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Add", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = unsafe {
            method.invoke_unchecked((), (baseMat, stencilID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Add_StencilOp_CompareFunction_ColorWriteMask1(
        baseMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        stencilID: i32,
        operation: crate::UnityEngine::Rendering::StencilOp,
        compareFunction: crate::UnityEngine::Rendering::CompareFunction,
        colorWriteMask: crate::UnityEngine::Rendering::ColorWriteMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    i32,
                    crate::UnityEngine::Rendering::StencilOp,
                    crate::UnityEngine::Rendering::CompareFunction,
                    crate::UnityEngine::Rendering::ColorWriteMask,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                5usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Add", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (baseMat, stencilID, operation, compareFunction, colorWriteMask),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn Add_StencilOp_CompareFunction_ColorWriteMask_i32_i32_2(
        baseMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        stencilID: i32,
        operation: crate::UnityEngine::Rendering::StencilOp,
        compareFunction: crate::UnityEngine::Rendering::CompareFunction,
        colorWriteMask: crate::UnityEngine::Rendering::ColorWriteMask,
        readMask: i32,
        writeMask: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    i32,
                    crate::UnityEngine::Rendering::StencilOp,
                    crate::UnityEngine::Rendering::CompareFunction,
                    crate::UnityEngine::Rendering::ColorWriteMask,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                7usize,
            >("Add")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Add", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        baseMat,
                        stencilID,
                        operation,
                        compareFunction,
                        colorWriteMask,
                        readMask,
                        writeMask,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearAll() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearAll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ClearAll", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn LogWarningWhenNotInBatchmode(
        warning: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("LogWarningWhenNotInBatchmode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LogWarningWhenNotInBatchmode", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (warning, context))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Remove(
        customMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Remove")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Remove", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (customMat))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::StencilMaterial {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct StencilMaterial_MatEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub baseMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub customMat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub count: i32,
    pub stencilId: i32,
    pub operation: crate::UnityEngine::Rendering::StencilOp,
    pub compareFunction: crate::UnityEngine::Rendering::CompareFunction,
    pub readMask: i32,
    pub writeMask: i32,
    pub useAlphaClip: bool,
    pub colorMask: crate::UnityEngine::Rendering::ColorWriteMask,
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UI::StencilMaterial_MatEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "StencilMaterial/MatEntry";
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
#[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
impl std::ops::Deref for crate::UnityEngine::UI::StencilMaterial_MatEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
impl std::ops::DerefMut for crate::UnityEngine::UI::StencilMaterial_MatEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
impl crate::UnityEngine::UI::StencilMaterial_MatEntry {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "UnityEngine+UI+StencilMaterial+MatEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UI::StencilMaterial_MatEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
