use crate::error::IpcError;
use fory::{Fory, ForyEnum, ForyStruct};
use serde::{Deserialize, Serialize};

pub const FORY_ID_LEASE_REQ: u32 = 1;
pub const FORY_ID_LEASE_GRANT: u32 = 2;
pub const FORY_ID_LEASE_REVOKE: u32 = 3;
pub const FORY_ID_STAGE_DEPLOY_REQ: u32 = 4;
pub const FORY_ID_STAGE_DEPLOY_RESP: u32 = 5;
pub const FORY_ID_SLOT_SWAP: u32 = 6;
pub const FORY_ID_ROLLBACK: u32 = 7;
pub const FORY_ID_IT_REPORT: u32 = 8;
pub const FORY_ID_ARTIFACT_TAINT: u32 = 9;
pub const FORY_ID_HEALTH_PROBE: u32 = 10;
pub const FORY_ID_HEARTBEAT: u32 = 11;
pub const FORY_ID_DEPLOYMENT_SLOT: u32 = 12;

// Full-name aliases for ergonomic downstream imports
pub const FORY_ID_DEPLOYMENT_LEASE_REQUEST: u32 = FORY_ID_LEASE_REQ;
pub const FORY_ID_DEPLOYMENT_LEASE_GRANT: u32 = FORY_ID_LEASE_GRANT;
pub const FORY_ID_DEPLOYMENT_LEASE_REVOKE: u32 = FORY_ID_LEASE_REVOKE;
pub const FORY_ID_STAGE_DEPLOYMENT_REQUEST: u32 = FORY_ID_STAGE_DEPLOY_REQ;
pub const FORY_ID_STAGE_DEPLOYMENT_RESPONSE: u32 = FORY_ID_STAGE_DEPLOY_RESP;
pub const FORY_ID_SLOT_SWAP_COMMAND: u32 = FORY_ID_SLOT_SWAP;
pub const FORY_ID_ROLLBACK_COMMAND: u32 = FORY_ID_ROLLBACK;
pub const FORY_ID_INTEGRATION_TEST_REPORT: u32 = FORY_ID_IT_REPORT;
pub const FORY_ID_ARTIFACT_TAINT_NOTIFICATION: u32 = FORY_ID_ARTIFACT_TAINT;
pub const FORY_ID_HEALTH_PROBE_TELEMETRY: u32 = FORY_ID_HEALTH_PROBE;
pub const FORY_ID_HEARTBEAT_TELEMETRY: u32 = FORY_ID_HEARTBEAT;

#[derive(ForyEnum, Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum DeploymentSlot {
    #[default]
    SlotA,
    SlotB,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DeploymentLeaseRequest {
    pub deployment_id: String,
    pub node_id: String,
    pub service_name: String,
    pub artifact_sha256: String,
    pub requested_ttl_secs: u64,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DeploymentLeaseGrant {
    pub deployment_id: String,
    pub lease_id: String,
    pub node_id: String,
    pub active_slot: DeploymentSlot,
    pub candidate_slot: DeploymentSlot,
    pub expires_at_epoch_ms: u64,
    pub granted: bool,
    pub denial_reason: Option<String>,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DeploymentLeaseRevoke {
    pub deployment_id: String,
    pub lease_id: String,
    pub reason: String,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StageDeploymentRequest {
    pub deployment_id: String,
    pub lease_id: String,
    pub service_name: String,
    pub artifact_sha256: String,
    pub target_slot: DeploymentSlot,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct StageDeploymentResponse {
    pub deployment_id: String,
    pub target_slot: DeploymentSlot,
    pub btrfs_subvolume_path: String,
    pub staged_ok: bool,
    pub error_message: Option<String>,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SlotSwapCommand {
    pub deployment_id: String,
    pub lease_id: String,
    pub service_name: String,
    pub from_slot: DeploymentSlot,
    pub to_slot: DeploymentSlot,
    pub systemd_unit: String,
    pub warmup_timeout_secs: u64,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RollbackCommand {
    pub deployment_id: String,
    pub lease_id: String,
    pub service_name: String,
    pub restore_slot: DeploymentSlot,
    pub failed_artifact_sha256: String,
    pub reason: String,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct IntegrationTestReport {
    pub deployment_id: String,
    pub service_name: String,
    pub artifact_sha256: String,
    pub executed_tests: Vec<String>,
    pub passed: bool,
    pub failed_tests: Vec<String>,
    pub duration_ms: u64,
    pub log_excerpt: Option<String>,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ArtifactTaintNotification {
    pub deployment_id: String,
    pub service_name: String,
    pub artifact_sha256: String,
    pub tainted_at_epoch_ms: u64,
    pub reason: String,
    pub rollback_completed: bool,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HealthProbeTelemetry {
    pub deployment_id: String,
    pub node_id: String,
    pub service_name: String,
    pub active_slot: DeploymentSlot,
    pub systemd_active: bool,
    pub health_endpoint_ok: bool,
    pub probe_latency_ms: u32,
    pub timestamp_epoch_ms: u64,
}

#[derive(ForyStruct, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct HeartbeatTelemetry {
    pub node_id: String,
    pub active_deployments: u32,
    pub btrfs_free_bytes: u64,
    pub timestamp_epoch_ms: u64,
}

pub fn create_fory_runtime() -> Result<Fory, IpcError> {
    let mut fory = Fory::builder().xlang(false).build();
    fory.register::<DeploymentLeaseRequest>(FORY_ID_LEASE_REQ)?;
    fory.register::<DeploymentLeaseGrant>(FORY_ID_LEASE_GRANT)?;
    fory.register::<DeploymentLeaseRevoke>(FORY_ID_LEASE_REVOKE)?;
    fory.register::<StageDeploymentRequest>(FORY_ID_STAGE_DEPLOY_REQ)?;
    fory.register::<StageDeploymentResponse>(FORY_ID_STAGE_DEPLOY_RESP)?;
    fory.register::<SlotSwapCommand>(FORY_ID_SLOT_SWAP)?;
    fory.register::<RollbackCommand>(FORY_ID_ROLLBACK)?;
    fory.register::<IntegrationTestReport>(FORY_ID_IT_REPORT)?;
    fory.register::<ArtifactTaintNotification>(FORY_ID_ARTIFACT_TAINT)?;
    fory.register::<HealthProbeTelemetry>(FORY_ID_HEALTH_PROBE)?;
    fory.register::<HeartbeatTelemetry>(FORY_ID_HEARTBEAT)?;
    fory.register::<DeploymentSlot>(FORY_ID_DEPLOYMENT_SLOT)?;
    Ok(fory)
}
