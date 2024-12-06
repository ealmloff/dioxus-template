// This file sets variables based on the results of the shared-init script.

const WORKSPACE_MEMBERS_VAR = "workspace_members";
const ROUTER_FEATURE_VAR = "router_feature";

const IS_FULLSTACK = "is_fullstack";
const IS_ROUTER = "is_router";

let workspace_members = ["web", "desktop", "mobile"];

if variable::get(IS_FULLSTACK) {
    workspace_members += "server"
}

variable::set(WORKSPACE_MEMBERS_VAR, workspace_members);

// Router feature
if variable::get(IS_ROUTER) {
    variable::set(ROUTER_FEATURE_VAR, "\"router\"");
} else {
    variable::set(ROUTER_FEATURE_VAR, "");
}
