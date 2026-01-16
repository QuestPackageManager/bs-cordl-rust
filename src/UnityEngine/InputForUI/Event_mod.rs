#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Event {
    padding: quest_hook::libil2cpp::ValueTypePadding<112usize>,
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::InputForUI::Event {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputForUI";
    const CLASS_NAME: &'static str = "Event";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::InputForUI::Event {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::InputForUI::Event {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::InputForUI::Event {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::InputForUI::Event {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputForUI::Event {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event")]
impl crate::UnityEngine::InputForUI::Event {
    #[cfg(feature = "UnityEngine+InputForUI+Event+IMapFn_1")]
    type IMapFn_1<TOutputType: quest_hook::libil2cpp::Type> = crate::UnityEngine::InputForUI::Event_IMapFn_1<
        TOutputType,
    >;
    #[cfg(feature = "UnityEngine+InputForUI+Event+MapAsEventModifiers")]
    pub type MapAsEventModifiers = crate::UnityEngine::InputForUI::Event_MapAsEventModifiers;
    #[cfg(feature = "UnityEngine+InputForUI+Event+MapAsEventSource")]
    pub type MapAsEventSource = crate::UnityEngine::InputForUI::Event_MapAsEventSource;
    #[cfg(feature = "UnityEngine+InputForUI+Event+MapAsObject")]
    pub type MapAsObject = crate::UnityEngine::InputForUI::Event_MapAsObject;
    #[cfg(feature = "UnityEngine+InputForUI+Event+Type")]
    pub type Type = crate::UnityEngine::InputForUI::Event_Type;
    pub fn CompareType(
        a: crate::UnityEngine::InputForUI::Event,
        b: crate::UnityEngine::InputForUI::Event,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::InputForUI::Event,
                            crate::UnityEngine::InputForUI::Event,
                        ),
                        i32,
                        2usize,
                    >("CompareType")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CompareType", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Ensure(
        &mut self,
        t: crate::UnityEngine::InputForUI::Event_Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputForUI::Event_Type),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Ensure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Ensure",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (t))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn From_CommandEvent4(
        commandEvent: crate::UnityEngine::InputForUI::CommandEvent,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::Event> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::InputForUI::CommandEvent),
                        crate::UnityEngine::InputForUI::Event,
                        1usize,
                    >("From")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "From",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::Event = unsafe {
            cordl_method_info.invoke_unchecked((), (commandEvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn From_IMECompositionEvent3(
        imeCompositionEvent: crate::UnityEngine::InputForUI::IMECompositionEvent,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::Event> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::InputForUI::IMECompositionEvent),
                        crate::UnityEngine::InputForUI::Event,
                        1usize,
                    >("From")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "From",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::Event = unsafe {
            cordl_method_info.invoke_unchecked((), (imeCompositionEvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn From_KeyEvent0(
        keyEvent: crate::UnityEngine::InputForUI::KeyEvent,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::Event> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::InputForUI::KeyEvent),
                        crate::UnityEngine::InputForUI::Event,
                        1usize,
                    >("From")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "From",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::Event = unsafe {
            cordl_method_info.invoke_unchecked((), (keyEvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn From_NavigationEvent5(
        navigationEvent: crate::UnityEngine::InputForUI::NavigationEvent,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::Event> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::InputForUI::NavigationEvent),
                        crate::UnityEngine::InputForUI::Event,
                        1usize,
                    >("From")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "From",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::Event = unsafe {
            cordl_method_info.invoke_unchecked((), (navigationEvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn From_PointerEvent1(
        pointerEvent: crate::UnityEngine::InputForUI::PointerEvent,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::Event> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::InputForUI::PointerEvent),
                        crate::UnityEngine::InputForUI::Event,
                        1usize,
                    >("From")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "From",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::Event = unsafe {
            cordl_method_info.invoke_unchecked((), (pointerEvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn From_TextInputEvent2(
        textInputEvent: crate::UnityEngine::InputForUI::TextInputEvent,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::Event> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::InputForUI::TextInputEvent),
                        crate::UnityEngine::InputForUI::Event,
                        1usize,
                    >("From")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "From",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::Event = unsafe {
            cordl_method_info.invoke_unchecked((), (textInputEvent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Map_1<TOutputType, TMapType>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<TOutputType>
    where
        TOutputType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TMapType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), TOutputType, 0usize>("Map")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Map",
                            0usize
                        )
                    })
            });
        let __cordl_ret: TOutputType = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Map_TMapType0<TOutputType, TMapType>(
        &mut self,
        _cordl_fn: TMapType,
    ) -> quest_hook::libil2cpp::Result<TOutputType>
    where
        TOutputType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TMapType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(TMapType), TOutputType, 1usize>("Map")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Map",
                            1usize
                        )
                    })
            });
        let __cordl_ret: TOutputType = unsafe {
            cordl_method_info.invoke_unchecked(self, (_cordl_fn))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("ToString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToString", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_asCommandEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::CommandEvent> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputForUI::CommandEvent,
                        0usize,
                    >("get_asCommandEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_asCommandEvent", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::CommandEvent = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_asIMECompositionEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputForUI::IMECompositionEvent,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputForUI::IMECompositionEvent,
                        0usize,
                    >("get_asIMECompositionEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_asIMECompositionEvent", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::IMECompositionEvent = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_asKeyEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::KeyEvent> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputForUI::KeyEvent,
                        0usize,
                    >("get_asKeyEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_asKeyEvent", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::KeyEvent = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_asNavigationEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::NavigationEvent> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputForUI::NavigationEvent,
                        0usize,
                    >("get_asNavigationEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_asNavigationEvent", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::NavigationEvent = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_asObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputForUI::IEventProperties>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputForUI::IEventProperties,
                        >,
                        0usize,
                    >("get_asObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_asObject", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputForUI::IEventProperties,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_asPointerEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::PointerEvent> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputForUI::PointerEvent,
                        0usize,
                    >("get_asPointerEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_asPointerEvent", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::PointerEvent = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_asTextInputEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::TextInputEvent> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputForUI::TextInputEvent,
                        0usize,
                    >("get_asTextInputEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_asTextInputEvent", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::TextInputEvent = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_eventModifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::EventModifiers> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputForUI::EventModifiers,
                        0usize,
                    >("get_eventModifiers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_eventModifiers", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::EventModifiers = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_eventSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::EventSource> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputForUI::EventSource,
                        0usize,
                    >("get_eventSource")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_eventSource", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::EventSource = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::Event_Type> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::InputForUI::Event_Type,
                        0usize,
                    >("get_type")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_type", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::Event_Type = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event")]
impl AsRef<crate::UnityEngine::InputForUI::IEventProperties>
for crate::UnityEngine::InputForUI::Event {
    fn as_ref(&self) -> &crate::UnityEngine::InputForUI::IEventProperties {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event")]
impl AsMut<crate::UnityEngine::InputForUI::IEventProperties>
for crate::UnityEngine::InputForUI::Event {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::InputForUI::IEventProperties {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+IMapFn_1")]
#[repr(C)]
#[derive(Debug)]
pub struct Event_IMapFn_1<TOutputType: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TOutputType: std::marker::PhantomData<TOutputType>,
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+IMapFn_1")]
unsafe impl<TOutputType: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::UnityEngine::InputForUI::Event_IMapFn_1<TOutputType> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputForUI";
    const CLASS_NAME: &'static str = "Event/IMapFn`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.InputForUI",
                        "Event/IMapFn`1",
                    )
                    .unwrap()
                    .make_generic::<(TOutputType)>()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "UnityEngine+InputForUI+Event+IMapFn_1")]
impl<TOutputType: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::UnityEngine::InputForUI::Event_IMapFn_1<TOutputType> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event+IMapFn_1")]
impl<TOutputType: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::UnityEngine::InputForUI::Event_IMapFn_1<TOutputType> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event+IMapFn_1")]
impl<
    TOutputType: quest_hook::libil2cpp::Type,
> crate::UnityEngine::InputForUI::Event_IMapFn_1<TOutputType> {
    pub fn Map<TEventType>(
        &mut self,
        ev: quest_hook::libil2cpp::ByRefMut<TEventType>,
    ) -> quest_hook::libil2cpp::Result<TOutputType>
    where
        TOutputType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<TEventType>),
                        TOutputType,
                        1usize,
                    >("Map")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Map",
                            1usize
                        )
                    })
            });
        let __cordl_ret: TOutputType = unsafe {
            cordl_method_info.invoke_unchecked(self, (ev))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+IMapFn_1")]
impl<TOutputType: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputForUI::Event_IMapFn_1<TOutputType> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventModifiers")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Event_MapAsEventModifiers {}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventModifiers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputForUI::Event_MapAsEventModifiers {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputForUI";
    const CLASS_NAME: &'static str = "Event/MapAsEventModifiers";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventModifiers")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputForUI::Event_MapAsEventModifiers {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventModifiers")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputForUI::Event_MapAsEventModifiers {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventModifiers")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputForUI::Event_MapAsEventModifiers {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventModifiers")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputForUI::Event_MapAsEventModifiers {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventModifiers")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputForUI::Event_MapAsEventModifiers {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event+MapAsEventModifiers")]
impl crate::UnityEngine::InputForUI::Event_MapAsEventModifiers {
    pub fn Map<TEventType>(
        &mut self,
        ev: quest_hook::libil2cpp::ByRefMut<TEventType>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::EventModifiers>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<TEventType>),
                        crate::UnityEngine::InputForUI::EventModifiers,
                        1usize,
                    >("Map")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Map",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::EventModifiers = unsafe {
            cordl_method_info.invoke_unchecked(self, (ev))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event+MapAsEventModifiers")]
impl AsRef<
    crate::UnityEngine::InputForUI::Event_IMapFn_1<
        crate::UnityEngine::InputForUI::EventModifiers,
    >,
> for crate::UnityEngine::InputForUI::Event_MapAsEventModifiers {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputForUI::Event_IMapFn_1<
        crate::UnityEngine::InputForUI::EventModifiers,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event+MapAsEventModifiers")]
impl AsMut<
    crate::UnityEngine::InputForUI::Event_IMapFn_1<
        crate::UnityEngine::InputForUI::EventModifiers,
    >,
> for crate::UnityEngine::InputForUI::Event_MapAsEventModifiers {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputForUI::Event_IMapFn_1<
        crate::UnityEngine::InputForUI::EventModifiers,
    > {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventSource")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Event_MapAsEventSource {}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventSource")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputForUI::Event_MapAsEventSource {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputForUI";
    const CLASS_NAME: &'static str = "Event/MapAsEventSource";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventSource")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputForUI::Event_MapAsEventSource {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventSource")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputForUI::Event_MapAsEventSource {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventSource")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputForUI::Event_MapAsEventSource {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventSource")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputForUI::Event_MapAsEventSource {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsEventSource")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputForUI::Event_MapAsEventSource {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event+MapAsEventSource")]
impl crate::UnityEngine::InputForUI::Event_MapAsEventSource {
    pub fn Map<TEventType>(
        &mut self,
        ev: quest_hook::libil2cpp::ByRefMut<TEventType>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::InputForUI::EventSource>
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<TEventType>),
                        crate::UnityEngine::InputForUI::EventSource,
                        1usize,
                    >("Map")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Map",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::InputForUI::EventSource = unsafe {
            cordl_method_info.invoke_unchecked(self, (ev))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event+MapAsEventSource")]
impl AsRef<
    crate::UnityEngine::InputForUI::Event_IMapFn_1<
        crate::UnityEngine::InputForUI::EventSource,
    >,
> for crate::UnityEngine::InputForUI::Event_MapAsEventSource {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputForUI::Event_IMapFn_1<
        crate::UnityEngine::InputForUI::EventSource,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event+MapAsEventSource")]
impl AsMut<
    crate::UnityEngine::InputForUI::Event_IMapFn_1<
        crate::UnityEngine::InputForUI::EventSource,
    >,
> for crate::UnityEngine::InputForUI::Event_MapAsEventSource {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputForUI::Event_IMapFn_1<
        crate::UnityEngine::InputForUI::EventSource,
    > {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsObject")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Event_MapAsObject {}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsObject")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputForUI::Event_MapAsObject {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputForUI";
    const CLASS_NAME: &'static str = "Event/MapAsObject";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsObject")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputForUI::Event_MapAsObject {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsObject")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputForUI::Event_MapAsObject {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsObject")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputForUI::Event_MapAsObject {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsObject")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputForUI::Event_MapAsObject {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+MapAsObject")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputForUI::Event_MapAsObject {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event+MapAsObject")]
impl crate::UnityEngine::InputForUI::Event_MapAsObject {
    pub fn Map<TEventType>(
        &mut self,
        ev: quest_hook::libil2cpp::ByRefMut<TEventType>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputForUI::IEventProperties>,
    >
    where
        TEventType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<TEventType>),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::InputForUI::IEventProperties,
                        >,
                        1usize,
                    >("Map")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Map",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputForUI::IEventProperties,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (ev))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event+MapAsObject")]
impl AsRef<
    crate::UnityEngine::InputForUI::Event_IMapFn_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputForUI::IEventProperties>,
    >,
> for crate::UnityEngine::InputForUI::Event_MapAsObject {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::InputForUI::Event_IMapFn_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputForUI::IEventProperties>,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputForUI+Event+MapAsObject")]
impl AsMut<
    crate::UnityEngine::InputForUI::Event_IMapFn_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputForUI::IEventProperties>,
    >,
> for crate::UnityEngine::InputForUI::Event_MapAsObject {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputForUI::Event_IMapFn_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputForUI::IEventProperties>,
    > {
        todo!()
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+Type")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Event_Type {
    #[default]
    CommandEvent = 5i32,
    IMECompositionEvent = 4i32,
    Invalid = 0i32,
    KeyEvent = 1i32,
    NavigationEvent = 6i32,
    PointerEvent = 2i32,
    TextInputEvent = 3i32,
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+Type")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::InputForUI::Event_Type {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputForUI";
    const CLASS_NAME: &'static str = "Event/Type";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+Type")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputForUI::Event_Type {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+Type")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputForUI::Event_Type {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+Type")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputForUI::Event_Type {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputForUI+Event+Type")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputForUI::Event_Type {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
