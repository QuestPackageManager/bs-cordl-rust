#[cfg(feature = "UnityEngine+UI+GraphicRegistry")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphicRegistry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Graphics: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UI::Collections::IndexedSet_1<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
                >,
            >,
        >,
    >,
    pub m_RaycastableGraphics: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::UI::Collections::IndexedSet_1<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
                >,
            >,
        >,
    >,
}
#[cfg(feature = "UnityEngine+UI+GraphicRegistry")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::UI::GraphicRegistry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UI";
    const CLASS_NAME: &'static str = "GraphicRegistry";
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
#[cfg(feature = "UnityEngine+UI+GraphicRegistry")]
impl std::ops::Deref for crate::UnityEngine::UI::GraphicRegistry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+GraphicRegistry")]
impl std::ops::DerefMut for crate::UnityEngine::UI::GraphicRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+GraphicRegistry")]
impl crate::UnityEngine::UI::GraphicRegistry {
    pub fn DisableGraphicForCanvas(
        c: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DisableGraphicForCanvas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DisableGraphicForCanvas", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (c, graphic))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisableRaycastGraphicForCanvas(
        c: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DisableRaycastGraphicForCanvas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DisableRaycastGraphicForCanvas", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (c, graphic))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGraphicsForCanvas(
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
                    >,
                >,
                1usize,
            >("GetGraphicsForCanvas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetGraphicsForCanvas", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
            >,
        > = unsafe { method.invoke_unchecked((), (canvas)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetRaycastableGraphicsForCanvas(
        canvas: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IList_1<
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
                    >,
                >,
                1usize,
            >("GetRaycastableGraphicsForCanvas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRaycastableGraphicsForCanvas", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
            >,
        > = unsafe { method.invoke_unchecked((), (canvas)) };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterGraphicForCanvas(
        c: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RegisterGraphicForCanvas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterGraphicForCanvas", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (c, graphic))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterRaycastGraphicForCanvas(
        c: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("RegisterRaycastGraphicForCanvas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterRaycastGraphicForCanvas", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (c, graphic))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterGraphicForCanvas(
        c: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UnregisterGraphicForCanvas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnregisterGraphicForCanvas", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (c, graphic))
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterRaycastGraphicForCanvas(
        c: quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
        graphic: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Canvas>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Graphic>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("UnregisterRaycastGraphicForCanvas")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnregisterRaycastGraphicForCanvas", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (c, graphic))
        };
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
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::GraphicRegistry>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::GraphicRegistry>,
                0usize,
            >("get_instance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_instance", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UI::GraphicRegistry,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UI+GraphicRegistry")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::GraphicRegistry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
