/// A typed wrapper that can be updated from device-originated CMD response payload bytes.
///
/// Important: implementations must *not* write back to the device (no `end_change()`),
/// because CMD responses / broadcasts are passive state synchronization.
pub trait CmdResponseObject: Sized {
    type Diff;

    /// Construct the object from the full cmd-response payload bytes.
    fn from_cmd_response_bytes(uuid: u128, payload: Vec<u8>) -> Self;

    /// Apply cmd-response payload bytes to the local cache and return a diff describing changes.
    fn apply_cmd_response_local_bytes(&self, payload: &[u8]) -> Self::Diff;
}
