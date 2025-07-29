#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_PDG_EventType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_PDG_EventType {
    #[default]
    HAPI_PDG_CONTEXT_EVENTS = 41i32,
    HAPI_PDG_EVENT_ALL = 35i32,
    HAPI_PDG_EVENT_BATCH_ITEM_INITIALIZED = 34i32,
    HAPI_PDG_EVENT_COOK_COMPLETE = 11i32,
    HAPI_PDG_EVENT_COOK_ERROR = 9i32,
    HAPI_PDG_EVENT_COOK_START = 30i32,
    HAPI_PDG_EVENT_COOK_WARNING = 10i32,
    HAPI_PDG_EVENT_DIRTY_ALL = 14i32,
    HAPI_PDG_EVENT_DIRTY_START = 12i32,
    HAPI_PDG_EVENT_DIRTY_STOP = 13i32,
    HAPI_PDG_EVENT_LOG = 36i32,
    HAPI_PDG_EVENT_NODE_CLEAR = 8i32,
    HAPI_PDG_EVENT_NODE_CONNECT = 19i32,
    HAPI_PDG_EVENT_NODE_CREATE = 16i32,
    HAPI_PDG_EVENT_NODE_DISCONNECT = 20i32,
    HAPI_PDG_EVENT_NODE_PROGRESS_UPDATE = 33i32,
    HAPI_PDG_EVENT_NODE_REMOVE = 17i32,
    HAPI_PDG_EVENT_NODE_RENAME = 18i32,
    HAPI_PDG_EVENT_NULL = 0i32,
    HAPI_PDG_EVENT_SCHEDULER_ADDED = 37i32,
    HAPI_PDG_EVENT_SCHEDULER_REMOVED = 38i32,
    HAPI_PDG_EVENT_SERVICE_MANAGER_ALL = 40i32,
    HAPI_PDG_EVENT_SET_SCHEDULER = 39i32,
    HAPI_PDG_EVENT_UI_SELECT = 15i32,
    HAPI_PDG_EVENT_WORKITEM_ADD = 1i32,
    HAPI_PDG_EVENT_WORKITEM_ADD_DEP = 4i32,
    HAPI_PDG_EVENT_WORKITEM_ADD_PARENT = 6i32,
    HAPI_PDG_EVENT_WORKITEM_ADD_STATIC_ANCESTOR = 31i32,
    HAPI_PDG_EVENT_WORKITEM_MERGE = 27i32,
    HAPI_PDG_EVENT_WORKITEM_PRIORITY = 29i32,
    HAPI_PDG_EVENT_WORKITEM_REMOVE = 2i32,
    HAPI_PDG_EVENT_WORKITEM_REMOVE_DEP = 5i32,
    HAPI_PDG_EVENT_WORKITEM_REMOVE_PARENT = 7i32,
    HAPI_PDG_EVENT_WORKITEM_REMOVE_STATIC_ANCESTOR = 32i32,
    HAPI_PDG_EVENT_WORKITEM_RESULT = 28i32,
    HAPI_PDG_EVENT_WORKITEM_SET_FILE = 24i32,
    HAPI_PDG_EVENT_WORKITEM_SET_FLOAT = 22i32,
    HAPI_PDG_EVENT_WORKITEM_SET_GEOMETRY = 26i32,
    HAPI_PDG_EVENT_WORKITEM_SET_INT = 21i32,
    HAPI_PDG_EVENT_WORKITEM_SET_PYOBJECT = 25i32,
    HAPI_PDG_EVENT_WORKITEM_SET_STRING = 23i32,
    HAPI_PDG_EVENT_WORKITEM_STATE_CHANGE = 3i32,
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_PDG_EventType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::HoudiniEngineUnity::HAPI_PDG_EventType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "HoudiniEngineUnity";
    const CLASS_NAME: &'static str = "HAPI_PDG_EventType";
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
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_PDG_EventType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::HoudiniEngineUnity::HAPI_PDG_EventType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_PDG_EventType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::HoudiniEngineUnity::HAPI_PDG_EventType {
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
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_PDG_EventType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::HoudiniEngineUnity::HAPI_PDG_EventType {
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
#[cfg(feature = "cordl_class_HoudiniEngineUnity+HAPI_PDG_EventType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::HoudiniEngineUnity::HAPI_PDG_EventType {
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
