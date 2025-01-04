#[cfg(feature = "HoudiniEngineUnity+HAPI_CurveOrders")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAPI_CurveOrders {
    #[default]
    HAPI_CURVE_ORDER_CUBIC = 4i32,
    HAPI_CURVE_ORDER_INVALID = 1i32,
    HAPI_CURVE_ORDER_LINEAR = 2i32,
    HAPI_CURVE_ORDER_QUADRATIC = 3i32,
    HAPI_CURVE_ORDER_VARYING = 0i32,
}
#[cfg(feature = "HoudiniEngineUnity+HAPI_CurveOrders")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HAPI_CurveOrders =>
    "HoudiniEngineUnity"."HAPI_CurveOrders"
);
