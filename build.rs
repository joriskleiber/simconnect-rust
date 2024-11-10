use std::{env, path::PathBuf};

const MISSING_SDK_PATH: &str = r#"
Could not find the MSFS SDK. Is it installed?

To generate the bindings for the MSFS SimConnect SDK,
you need to have the SDK installed.

You can download the SDK by following this guide:
https://docs.flightsimulator.com/html/Introduction/SDK_Overview.htm

Once you have it downloaded and installed the environment variable 
`MSFS_SDK` should have been set automatically.
"#;

fn main() {
    let msfs_sdk_env = env::var("MSFS_SDK").expect(MISSING_SDK_PATH);
    let out_dir_env = env::var("OUT_DIR").unwrap();

    let sim_connect_sdk_path = PathBuf::from(msfs_sdk_env).join("SimConnect SDK");

    println!(
        "cargo:rustc-link-search={}",
        sim_connect_sdk_path
            .join("lib")
            .join("static")
            .to_string_lossy()
    );
    println!("cargo:rustc-link-lib=SimConnect");

    let allowed_vars = [
        "SIMCONNECT_UNUSED",
        "SIMCONNECT_OBJECT_ID_USER",
        "SIMCONNECT_CAMERA_IGNORE_FIELD",
        "SIMCONNECT_CLIENTDATA_MAX_SIZE",
        "SIMCONNECT_GROUP_PRIORITY_HIGHEST",
        "SIMCONNECT_GROUP_PRIORITY_HIGHEST_MASKABLE",
        "SIMCONNECT_GROUP_PRIORITY_STANDARD",
        "SIMCONNECT_GROUP_PRIORITY_DEFAULT",
        "SIMCONNECT_GROUP_PRIORITY_LOWEST",
        "MAX_METAR_LENGTH",
        "MAX_THERMAL_SIZE",
        "MAX_THERMAL_RATE",
        "INITPOSITION_AIRSPEED_CRUISE",
        "INITPOSITION_AIRSPEED_KEEP",
        "SIMCONNECT_CLIENTDATATYPE_INT8",
        "SIMCONNECT_CLIENTDATATYPE_INT16",
        "SIMCONNECT_CLIENTDATATYPE_INT32",
        "SIMCONNECT_CLIENTDATATYPE_INT64",
        "SIMCONNECT_CLIENTDATATYPE_FLOAT32",
        "SIMCONNECT_CLIENTDATATYPE_FLOAT64",
        "SIMCONNECT_CLIENTDATAOFFSET_AUTO",
        "SIMCONNECT_OPEN_CONFIGINDEX_LOCAL",
        "SIMCONNECT_RECV_ID_VOR_LIST_HAS_NAV_SIGNAL",
        "SIMCONNECT_RECV_ID_VOR_LIST_HAS_LOCALIZER",
        "SIMCONNECT_RECV_ID_VOR_LIST_HAS_GLIDE_SLOPE",
        "SIMCONNECT_RECV_ID_VOR_LIST_HAS_DME",
        "SIMCONNECT_WAYPOINT_NONE",
        "SIMCONNECT_WAYPOINT_SPEED_REQUESTED",
        "SIMCONNECT_WAYPOINT_THROTTLE_REQUESTED",
        "SIMCONNECT_WAYPOINT_COMPUTE_VERTICAL_SPEED",
        "SIMCONNECT_WAYPOINT_ALTITUDE_IS_AGL",
        "SIMCONNECT_WAYPOINT_ON_GROUND",
        "SIMCONNECT_WAYPOINT_REVERSE",
        "SIMCONNECT_WAYPOINT_WRAP_TO_FIRST",
        "SIMCONNECT_EVENT_FLAG_DEFAULT",
        "SIMCONNECT_EVENT_FLAG_FAST_REPEAT_TIMER",
        "SIMCONNECT_EVENT_FLAG_SLOW_REPEAT_TIMER",
        "SIMCONNECT_EVENT_FLAG_GROUPID_IS_PRIORITY",
        "SIMCONNECT_DATA_REQUEST_FLAG_DEFAULT",
        "SIMCONNECT_DATA_REQUEST_FLAG_CHANGED",
        "SIMCONNECT_DATA_REQUEST_FLAG_TAGGED",
        "SIMCONNECT_DATA_SET_FLAG_DEFAULT",
        "SIMCONNECT_DATA_SET_FLAG_TAGGED",
        "SIMCONNECT_CREATE_CLIENT_DATA_FLAG_DEFAULT",
        "SIMCONNECT_CREATE_CLIENT_DATA_FLAG_READ_ONLY",
        "SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_DEFAULT",
        "SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_CHANGED",
        "SIMCONNECT_CLIENT_DATA_REQUEST_FLAG_TAGGED",
        "SIMCONNECT_CLIENT_DATA_SET_FLAG_DEFAULT",
        "SIMCONNECT_CLIENT_DATA_SET_FLAG_TAGGED",
        "SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_COCKPIT_2D",
        "SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_COCKPIT_VIRTUAL",
        "SIMCONNECT_VIEW_SYSTEM_EVENT_DATA_ORTHOGONAL",
        "SIMCONNECT_SOUND_SYSTEM_EVENT_DATA_MASTER",
        "UNKNOWN_SENDID",
        "UNKNOWN_INDEX",
        "UNKNOWN_GROUP",
        "SIMCONNECT_CLOUD_STATE_ARRAY_WIDTH",
        "SIMCONNECT_CLOUD_STATE_ARRAY_SIZE",
    ];
    let allowed_types = [
        "HANDLE",
        "SIMCONNECT_RECV_ID",
        "SIMCONNECT_DATATYPE",
        "SIMCONNECT_EXCEPTION",
        "SIMCONNECT_SIMOBJECT_TYPE",
        "SIMCONNECT_STATE",
        "SIMCONNECT_PERIOD",
        "SIMCONNECT_MISSION_END",
        "SIMCONNECT_CLIENT_DATA_PERIOD",
        "SIMCONNECT_TEXT_TYPE",
        "SIMCONNECT_TEXT_RESULT",
        "SIMCONNECT_WEATHER_MODE",
        "SIMCONNECT_FACILITY_LIST_TYPE",
        "SIMCONNECT_RECV",
        "SIMCONNECT_RECV_EXCEPTION",
        "SIMCONNECT_RECV_OPEN",
        "SIMCONNECT_RECV_QUIT",
        "SIMCONNECT_RECV_EVENT",
        "SIMCONNECT_RECV_EVENT_FILENAME",
        "SIMCONNECT_RECV_EVENT_OBJECT_ADDREMOVE",
        "SIMCONNECT_RECV_EVENT_FRAME",
        "SIMCONNECT_RECV_EVENT_MULTIPLAYER_SERVER_STARTED",
        "SIMCONNECT_RECV_EVENT_MULTIPLAYER_CLIENT_STARTED",
        "SIMCONNECT_RECV_EVENT_MULTIPLAYER_SESSION_ENDED",
        "SIMCONNECT_RECV_EVENT_RACE_END",
        "SIMCONNECT_RECV_EVENT_RACE_LAP",
        "SIMCONNECT_RECV_SIMOBJECT_DATA",
        "SIMCONNECT_RECV_SIMOBJECT_DATA_BYTYPE",
        "SIMCONNECT_RECV_CLIENT_DATA",
        "SIMCONNECT_RECV_WEATHER_OBSERVATION",
        "SIMCONNECT_RECV_CLOUD_STATE",
        "SIMCONNECT_RECV_ASSIGNED_OBJECT_ID",
        "SIMCONNECT_RECV_RESERVED_KEY",
        "SIMCONNECT_RECV_SYSTEM_STATE",
        "SIMCONNECT_RECV_CUSTOM_ACTION",
        "SIMCONNECT_RECV_EVENT_WEATHER_MODE",
        "SIMCONNECT_RECV_FACILITIES_LIST",
        "SIMCONNECT_DATA_FACILITY_AIRPORT",
        "SIMCONNECT_RECV_AIRPORT_LIST",
        "SIMCONNECT_DATA_FACILITY_WAYPOINT",
        "SIMCONNECT_RECV_WAYPOINT_LIST",
        "SIMCONNECT_DATA_FACILITY_NDB",
        "SIMCONNECT_RECV_NDB_LIST",
        "SIMCONNECT_DATA_FACILITY_VOR",
        "SIMCONNECT_RECV_VOR_LIST",
        "SIMCONNECT_RECV_PICK",
        "SIMCONNECT_DATA_RACE_RESULT",
        "SIMCONNECT_DATA_INITPOSITION",
        "SIMCONNECT_DATA_MARKERSTATE",
        "SIMCONNECT_DATA_WAYPOINT",
        "SIMCONNECT_DATA_LATLONALT",
        "SIMCONNECT_DATA_XYZ",
    ];
    let allowed_functions = [
        "SimConnect_MapClientEventToSimEvent",
        "SimConnect_TransmitClientEvent",
        "SimConnect_SetSystemEventState",
        "SimConnect_AddClientEventToNotificationGroup",
        "SimConnect_RemoveClientEvent",
        "SimConnect_SetNotificationGroupPriority",
        "SimConnect_ClearNotificationGroup",
        "SimConnect_RequestNotificationGroup",
        "SimConnect_AddToDataDefinition",
        "SimConnect_ClearDataDefinition",
        "SimConnect_RequestDataOnSimObject",
        "SimConnect_RequestDataOnSimObjectType",
        "SimConnect_SetDataOnSimObject",
        "SimConnect_MapInputEventToClientEvent",
        "SimConnect_SetInputGroupPriority",
        "SimConnect_RemoveInputEvent",
        "SimConnect_ClearInputGroup",
        "SimConnect_SetInputGroupState",
        "SimConnect_RequestReservedKey",
        "SimConnect_SubscribeToSystemEvent",
        "SimConnect_UnsubscribeFromSystemEvent",
        "SimConnect_WeatherRequestInterpolatedObservation",
        "SimConnect_WeatherRequestObservationAtStation",
        "SimConnect_WeatherRequestObservationAtNearestStation",
        "SimConnect_WeatherCreateStation",
        "SimConnect_WeatherRemoveStation",
        "SimConnect_WeatherSetObservation",
        "SimConnect_WeatherSetModeServer",
        "SimConnect_WeatherSetModeTheme",
        "SimConnect_WeatherSetModeGlobal",
        "SimConnect_WeatherSetModeCustom",
        "SimConnect_WeatherSetDynamicUpdateRate",
        "SimConnect_WeatherRequestCloudState",
        "SimConnect_WeatherCreateThermal",
        "SimConnect_WeatherRemoveThermal",
        "SimConnect_AICreateParkedATCAircraft",
        "SimConnect_AICreateEnrouteATCAircraft",
        "SimConnect_AICreateNonATCAircraft",
        "SimConnect_AICreateSimulatedObject",
        "SimConnect_AIReleaseControl",
        "SimConnect_AIRemoveObject",
        "SimConnect_AISetAircraftFlightPlan",
        "SimConnect_ExecuteMissionAction",
        "SimConnect_CompleteCustomMissionAction",
        "SimConnect_Close",
        "SimConnect_RetrieveString",
        "SimConnect_GetLastSentPacketID",
        "SimConnect_Open",
        "SimConnect_CallDispatch",
        "SimConnect_GetNextDispatch",
        "SimConnect_RequestResponseTimes",
        "SimConnect_InsertString",
        "SimConnect_CameraSetRelative6DOF",
        "SimConnect_MenuAddItem",
        "SimConnect_MenuDeleteItem",
        "SimConnect_MenuAddSubItem",
        "SimConnect_MenuDeleteSubItem",
        "SimConnect_RequestSystemState",
        "SimConnect_SetSystemState",
        "SimConnect_MapClientDataNameToID",
        "SimConnect_CreateClientData",
        "SimConnect_AddToClientDataDefinition",
        "SimConnect_ClearClientDataDefinition",
        "SimConnect_RequestClientData",
        "SimConnect_SetClientData",
        "SimConnect_FlightLoad",
        "SimConnect_FlightSave",
        "SimConnect_FlightPlanLoad",
        "SimConnect_Text",
        "SimConnect_SubscribeToFacilities",
        "SimConnect_UnsubscribeToFacilities",
        "SimConnect_RequestFacilitiesList",
    ];

    let sim_connect_sdk_path = format!(
        "-I{}",
        sim_connect_sdk_path.join("include").to_string_lossy()
    );

    let bindings = bindgen::Builder::default()
        .clang_arg(sim_connect_sdk_path)
        .header("wrapper.hpp")
        .allowlist_var(allowed_vars.join("|"))
        .allowlist_type(allowed_types.join("|"))
        .allowlist_function(allowed_functions.join("|"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .impl_debug(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(out_dir_env);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
