async function emit_workspace_set(e) {
    let space = parseInt(e.dataset.workspace, 10);
    await IPC_Front_emit("Workspace", "Set", {"space": space});
}